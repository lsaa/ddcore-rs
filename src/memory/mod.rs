//
// memory
//

use std::collections::HashMap;
use std::mem::size_of;
use std::cell::RefCell;
use std::process::Child;
use anyhow::bail;
use sysinfo::{Pid, ProcessExt, System, SystemExt, PidExt};
use crate::models::{StatsBlockWithFrames, StatsDataBlock, StatsFrame};

use self::proc_mem_wrapper::Handle;

pub mod proc_mem_wrapper;

///////////////////////////////
///////////////////////////////

const DATA_BLOCK_SIZE: usize = size_of::<StatsDataBlock>();
const STATS_FRAME_SIZE: usize = size_of::<StatsFrame>();

thread_local! {
    static BLOCK_BUF: RefCell<[u8; DATA_BLOCK_SIZE]> = RefCell::new([0_u8; DATA_BLOCK_SIZE]);
    static FRAME_BUF: RefCell<[u8; STATS_FRAME_SIZE]> = RefCell::new([0_u8; STATS_FRAME_SIZE]);
    static SYSTEM: RefCell<System> = RefCell::new(System::new_all());
}

/////////////////////////////// Structs
///////////////////////////////

pub struct OsInfo {
    pub default_block_marker: usize,
    pub default_process_name: String,
    pub can_create_child: bool,
    pub offsets: HashMap<String, Vec<usize>>
}

#[derive(Debug)]
pub enum OperatingSystem {
    Linux,
    LinuxProton,
    Windows
}

pub struct GameConnection {
    pub pid: Pid,
    pub path: String,
    pub handle: Handle,
    pub base_address: usize,
    pub last_fetch: Option<StatsBlockWithFrames>,
    pub child_handle: Option<Child>,
    pub params: ConnectionParams,
    pub(crate) pointers: Pointers
}

#[derive(Default)]
pub struct Pointers {
    pub ddstats_block: Option<usize>,
    pub base_address: Option<usize>
}

#[derive(Default)]
pub struct MemoryOverride {
    pub block_marker: Option<usize>,
    pub process_name: Option<String>,
}

pub struct ConnectionParams {
    pub create_child: bool,
    pub operating_system: OperatingSystem,
    pub overrides: MemoryOverride,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/////////////////////////////// Struct Impls
///////////////////////////////

impl OsInfo {
    pub fn get_from_os(os: &OperatingSystem) -> Self {
        match os {
            OperatingSystem::Linux => Self {
                can_create_child: true,
                default_block_marker: 0x00521C98,
                default_process_name: String::from("devildaggers"),
                offsets: HashMap::new()
            },
            OperatingSystem::Windows => Self {
                can_create_child: false,
                default_block_marker: 0x250DC0,
                default_process_name: String::from("dd.exe"),
                offsets: HashMap::new()
            },
            OperatingSystem::LinuxProton => Self {
                can_create_child: false,
                default_block_marker: 0x250DC0,
                default_process_name: String::from("wine-preloader"),
                offsets: HashMap::new()
            }
        }
    }
}

impl ConnectionParams {
    pub fn empty() -> Self {
        Self {
            create_child: false,
            operating_system: OperatingSystem::Linux,
            overrides: MemoryOverride::default()
        }
    }
}

impl GameConnection {
    #[cfg(target_os = "windows")]
    pub fn try_create(params: ConnectionParams) -> anyhow::Result<Self> {
        let os_info = OsInfo::get_from_os(&params.operating_system);
        let proc_name = params.overrides.process_name.as_ref().unwrap_or(&os_info.default_process_name).clone();
        let proc = get_proc(&proc_name);
        if proc.is_none() { anyhow::bail!("Process not found") }
        let pid = proc.as_ref().unwrap().1;
        let handle = Handle::new(pid.as_u32() as usize)?;
        let base_address = base_addr(&handle, &params);
        if base_address.is_err() { anyhow::bail!("Couldn't get base address") }
        let base_address = base_address.unwrap();
        let mut ptrs = Pointers::default();
        ptrs.base_address = Some(base_address);
        Ok(Self {
            pid,
            handle,
            base_address,
            path: proc.as_ref().unwrap().0.clone(),
            child_handle: None,
            last_fetch: None,
            params,
            pointers: ptrs
        })
    }

    #[cfg(target_os = "linux")]
    pub fn try_create(params: ConnectionParams) -> anyhow::Result<Self> {
        let os_info = OsInfo::get_from_os(&params.operating_system);
        let proc_name = params.overrides.process_name.as_ref().unwrap_or(&os_info.default_process_name).clone();
        let mut proc = get_proc(&proc_name);
        if proc.is_none() { anyhow::bail!("Process not found") }
        let mut pid = proc.as_ref().unwrap().1;
        let mut handle = Handle::new(pid.as_u32() as usize)?;
        let mut c = None;
        if handle.copy_address(0, &mut [0u8]).is_err() && params.create_child{
            c = create_as_child(pid);
            proc = get_proc(&proc_name);
            pid = proc.as_ref().unwrap().1;
            handle = Handle::new(pid.as_u32() as usize)?;
        }
        let base_address = base_addr(&handle, &params);
        if base_address.is_err() { anyhow::bail!("Couldn't get base address") }
        let base_address = base_address.unwrap();
        let ptrs = Pointers { base_address: Some(base_address), ..Default::default() };
        Ok(Self {
            pid,
            handle,
            base_address,
            path: proc.as_ref().unwrap().0.clone(),
            child_handle: c,
            last_fetch: None,
            params,
            pointers: ptrs
        })
    }

    pub fn dead_connection() -> Self {
        Self {
            pid: Pid::from_u32(0),
            base_address: 0,
            last_fetch: None,
            path: String::new(),
            handle: Handle::null_type(),
            child_handle: None,
            params: ConnectionParams::empty(),
            pointers: Pointers::default()
        }
    }

    pub fn is_alive(&mut self) -> bool {
        match self.read_stats_block() {
            Ok(_) => true,
            Err(_e) => false
        }
    }

    pub fn is_alive_res(&mut self) -> anyhow::Result<()> {
        match self.read_stats_block() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn read_stats_block(&mut self) -> anyhow::Result<StatsDataBlock> {
        read_stats_data_block(&self.handle, &self.params, &mut self.pointers)
    }

    pub fn read_mem(&self, addr: usize, buffer: &mut [u8]) -> anyhow::Result<()> {
        self.handle.copy_address(addr, buffer)
    }

    #[cfg(target_os = "windows")]
    pub fn maximize_dd(&self) {
        use winapi::shared::windef::HWND;
        use winapi::shared::minwindef::DWORD;

        enumerate_windows(|hwnd: HWND| {
            let mut pid: DWORD = DWORD::default();
            unsafe { winapi::um::winuser::GetWindowThreadProcessId(hwnd, &mut pid); }
            if pid as u32 != self.pid.as_u32() {
                true
            } else {
                unsafe { winapi::um::winuser::ShowWindow(hwnd, 9); }
                false
            }
        })
    }

    #[cfg(target_os = "linux")]
    pub fn maximize_dd(&self) {
        // pep
    }

    pub fn read_stats_block_with_frames(&mut self) -> anyhow::Result<StatsBlockWithFrames> {
        match read_stats_data_block(&self.handle, &self.params, &mut self.pointers) {
            Ok(data) => {
                let res = StatsBlockWithFrames {
                    frames: self.stat_frames_from_block(&data)?,
                    block: data,
                };
                self.last_fetch = Some(res.clone());
                Ok(res)
            },
            Err(e) => {
                log::info!("[DDCORE] Failed to read stats block {e:?}");
                Err(anyhow::anyhow!(e))
            }
        }
    }

    pub fn stat_frames_from_block(
        &mut self,
        block: &StatsDataBlock,
    ) -> anyhow::Result<Vec<StatsFrame>> {
        let (mut ptr, len) = (
            block.get_stats_pointer(),
            block.stats_frames_loaded as usize,
        );
        let mut res = Vec::with_capacity(len);
        FRAME_BUF.with(|buf| {
            let mut buf = buf.borrow_mut();
            for _ in 0..len {
                self.handle.copy_address(ptr, buf.as_mut())?;
                let (_head, body, _tail) = unsafe { buf.align_to::<StatsFrame>() };
                res.push(body[0]);
                ptr += STATS_FRAME_SIZE;
            }
            Ok(res)
        })
    }

    pub fn replay_bin(&mut self) -> anyhow::Result<Vec<u8>> {
        if let Some(block) = &self.last_fetch {
            let (ptr, len) = (
                block.block.get_replay_pointer(),
                block.block.replay_buffer_length as usize,
            );
            let mut res = vec![0u8; len];
            self.handle.copy_address(ptr, &mut res)?;
            Ok(res)
        } else {
            Err(anyhow::anyhow!(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Stats not available",
            )))
        }
    }

    pub fn stat_frames(&self) -> anyhow::Result<Vec<StatsFrame>> {
        if let Some(last_data) = &self.last_fetch {
            let (mut ptr, len) = (
                last_data.block.get_stats_pointer(),
                last_data.block.stats_frames_loaded as usize,
            );
            let mut res = Vec::with_capacity(len);
            FRAME_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                for _ in 0..len {
                    self.handle.copy_address(ptr, buf.as_mut())?;
                    let (_head, body, _tail) = unsafe { buf.align_to::<StatsFrame>() };
                    res.push(body[0]);
                    ptr += STATS_FRAME_SIZE;
                }
                Ok(res)
            })
        } else {
            Err(anyhow::anyhow!(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Stats not available",
            )))
        }
    }

    pub fn last_stat_frame(&self) -> anyhow::Result<StatsFrame> {
        if let Some(last_data) = &self.last_fetch {
            let (mut ptr, len) = (
                last_data.block.get_stats_pointer(),
                last_data.block.stats_frames_loaded as usize,
            );
            ptr += STATS_FRAME_SIZE * (len - 1);
            FRAME_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                self.handle.copy_address(ptr, buf.as_mut())?;
                let (_head, body, _tail) = unsafe { buf.align_to::<StatsFrame>() };
                Ok(body[0])
            })
        } else {
            Err(anyhow::anyhow!(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Stats not available",
            )))
        }
    }

    pub fn play_replay(&self, replay: std::sync::Arc<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(last_data) = &self.last_fetch {
            #[cfg(feature = "logger")]
            log::info!("[DDCORE] Attempting to load replay");

            let ddstats_addr = self.pointers.ddstats_block.expect("last data can't exist without this also being set");
            let replay_buffer_addr = last_data.block.get_replay_pointer();
            let flag_addr = ddstats_addr + 316;
            let len_addr = ddstats_addr + 312;
            let len = replay.len() as i32;

            #[cfg(feature = "logger")]
            log::info!("[DDCORE] Replay Flag debug: {ddstats_addr:X} {replay_buffer_addr:X} {flag_addr:X}");

            self.handle.put_address(replay_buffer_addr, &replay)?;
            self.handle.put_address(len_addr, &len.to_le_bytes())?;
            self.handle.put_address(flag_addr, &[true as u8])?;

            Ok(())
        } else {
            bail!("No data found");
        }
    }
}

/////////////////////////////// The Funcs
///////////////////////////////

pub fn get_proc(process_name: &str) -> Option<(String, Pid)> {
    SYSTEM.with(|s| {
        let mut s = s.borrow_mut();
        s.refresh_processes();
        if let Some(process) = s.processes_by_exact_name(process_name).next() {
            return Some((String::from(process.exe().to_str().unwrap()), process.pid()));
        }
        None
    })
}

#[cfg(target_os = "windows")]
pub fn base_addr(handle: &Handle, params: &ConnectionParams) -> anyhow::Result<usize> {
    let os_info = OsInfo::get_from_os(&params.operating_system);
    let proc_name = params.overrides.process_name.as_ref().unwrap_or(&os_info.default_process_name).clone();
    #[cfg(feature = "logger")]
    log::info!("[DDCORE] reading base address: {} {proc_name}", handle.pid);
    let addr = unsafe { get_base_address(Pid::from_u32(handle.pid as u32), proc_name) };
    #[cfg(feature = "logger")]
    log::info!("[DDCORE] base address: {addr:?}");
    addr
}

#[cfg(target_os = "linux")]
pub fn base_addr(handle: &Handle, params: &ConnectionParams) ->  anyhow::Result<usize> {
    use std::io::Read;

    use scan_fmt::scan_fmt;
    let os_info = OsInfo::get_from_os(&params.operating_system);
    let proc_name = params.overrides.process_name.as_ref().unwrap_or(&os_info.default_process_name).clone();
    let pid = Pid::from_u32(handle.pid as u32);
    
    match &params.operating_system {
        OperatingSystem::Linux => get_base_address(pid, proc_name),
        OperatingSystem::Windows => get_base_address(pid, proc_name),
        OperatingSystem::LinuxProton => {
            use std::{
                fs::File,
                io::{BufRead, BufReader},
            };

            let mut stat = String::new();
            BufReader::new(File::open(format!("/proc/{}/stat", pid))?).read_to_string(&mut stat)?;

            if !stat.contains("dd.exe") {
                return Err(anyhow::anyhow!(std::io::Error::new(std::io::ErrorKind::NotFound, "Not the correct process")));
            }

            let f = BufReader::new(File::open(format!("/proc/{}/maps", pid))?);
            let mut magic_buf = [0u8; 2];

            #[cfg(feature = "logger")]
            log::info!("[DDCORE] Found LinuxProton dd.exe {:?}", pid);

            for line in f.lines().flatten() {
                if let Ok((start, _end, _perms, mod_path)) = scan_fmt!(&line, "{x}-{x} {} {*} {*} {*} {[^\t\n]}\n", [hex usize], [hex usize], String, String)
                {
                    let r = handle.copy_address(start, &mut magic_buf);
                    if r.is_err() {
                        #[cfg(feature = "logger")]
                        log::info!("[DDCORE] Failed to read memory {:?} {} {:X}", r.err(), pid, start);
                        continue;
                    }

                    if mod_path.contains("dd.exe") && is_windows_exe(&magic_buf) {
                        return Ok(start);
                    }
                }
            }

            Err(anyhow::anyhow!(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No base address",
            )))
        }
    }
}

pub fn is_elf(start_bytes: &[u8; 4]) -> bool {
    let elf_signature: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    elf_signature == *start_bytes
}

pub fn is_windows_exe(start_bytes: &[u8; 2]) -> bool {
    let elf_signature: [u8; 2] = [0x4D, 0x5A];
    elf_signature == *start_bytes
}

#[cfg(target_os = "linux")]
pub fn get_base_address(pid: Pid, proc_name: String) -> anyhow::Result<usize> {
    use scan_fmt::scan_fmt;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let f = BufReader::new(File::open(format!("/proc/{}/maps", pid))?);
    let handle = Handle::new(pid.as_u32() as usize)?;
    let mut magic_buf = [0u8; 4];

    for line in f.lines().flatten() {
        if let Ok((start, _end, perms, mod_path)) = scan_fmt!(&line, "{x}-{x} {} {*} {*} {*} {[^\t\n]}\n", [hex usize], [hex usize], String, String)
        {
            let r = handle.copy_address(start, &mut magic_buf);
            if r.is_err() {
                continue;
            }

            if is_elf(&magic_buf) && mod_path.contains(&proc_name) && perms.contains('x') {
                return Ok(start);
            }
        }
    }

    Err(anyhow::anyhow!(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No base address",
    )))
}

#[cfg(target_os = "windows")]
pub fn enumerate_windows<F>(mut callback: F)
    where F: FnMut(winapi::shared::windef::HWND) -> bool
{
    use winapi::shared::windef::HWND;
    use winapi::shared::minwindef::LPARAM;
    use winapi::um::winuser::EnumWindows;
    use std::mem;
    use winapi::ctypes::c_void;
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enumerate_callback(hwnd: winapi::shared::windef::HWND, lparam: winapi::shared::minwindef::LPARAM) -> winapi::shared::minwindef::BOOL {
    use std::mem;
    use winapi::shared::windef::HWND;
    use winapi::shared::minwindef::{TRUE, FALSE};
    use winapi::ctypes::c_void;
    let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
    if closure(hwnd) { TRUE } else { FALSE }
}

/// # Safety
/// Winapi operation, self contained
#[cfg(target_os = "windows")]
pub unsafe fn get_base_address(pid: Pid, _proc_name: String) -> anyhow::Result<usize> {
    // This is miserable
    use winapi::um::handleapi::CloseHandle;
    use std::{mem::size_of_val, os::raw::c_ulong};

    let snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(
        winapi::um::tlhelp32::TH32CS_SNAPMODULE | winapi::um::tlhelp32::TH32CS_SNAPMODULE32,
        pid.as_u32() as winapi::shared::minwindef::DWORD,
    );

    let mut me: winapi::um::tlhelp32::MODULEENTRY32 = std::mem::zeroed();
    me.dwSize = size_of_val(&me) as c_ulong as winapi::shared::minwindef::DWORD;
    winapi::um::tlhelp32::Module32First(snapshot, &mut me);

    let res = me.modBaseAddr as usize;
    CloseHandle(snapshot);
    Ok(res)
}

#[cfg(target_os = "windows")]
fn _create_as_child(_pid: Pid) -> Option<Child> {
    None
}

#[cfg(target_os = "linux")]
fn create_as_child(pid: Pid) -> Option<Child> {
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
        process::Command,
    };

    let mut exe = String::new();
    BufReader::new(File::open(format!("/proc/{}/cmdline", pid)).expect("Coudln't read cmdline"))
        .read_to_string(&mut exe)
        .unwrap();
    let cwd = Path::new(&format!("/proc/{}/cwd", pid)).to_owned();
    let mut exe = exe.chars();
    exe.next_back();
    let exe = exe.as_str();
    Command::new("kill")
        .arg(format!("{}", pid))
        .spawn()
        .expect("Couldn't kill current DD process");
    let old_cwd = std::env::current_dir().expect("Couldn't save cwd");
    std::env::set_current_dir(&cwd).expect("Coudln't set cwd");
    Command::new("sh")
        .arg("-c")
        .arg("echo")
        .arg("422970 > steam_appid.txt")
        .spawn()
        .expect("Coudln't write steam appid");
    Command::new("nohup")
        .arg(exe)
        .spawn()
        .expect("Couldn't create DD child process");
    std::env::set_current_dir(&old_cwd).expect("Couldn't set cwd");
    None
}

pub fn mem_search(handle: &Handle, to_find: &[u8]) -> anyhow::Result<usize> {
    let mut big_ass_buffer = [0_u8; 1024 * 100]; // 100kb buffer
    let mut offset = 0x00010000;
    loop {
        handle.copy_address(offset, &mut big_ass_buffer)?;
        for (i, w) in big_ass_buffer.windows(to_find.len()).enumerate() {
            if w == to_find {
                return Ok(offset + i);
            }
        }
        offset += big_ass_buffer.len();
    }
}

fn calc_pointer_ddstats_block(handle: &Handle, params: &ConnectionParams, base_address: usize) -> anyhow::Result<usize> {
    let os_info = OsInfo::get_from_os(&params.operating_system);
    let block_start = params.overrides.block_marker.unwrap_or(os_info.default_block_marker);
    log::info!("[DDCORE] block start {block_start}");
    match &params.operating_system {
        OperatingSystem::Linux => {
            handle.get_offset(&[base_address + block_start, 0])
        },
        OperatingSystem::Windows => {
            handle.get_offset(&[base_address + block_start, 0])
        },
        OperatingSystem::LinuxProton => {
            mem_search(handle, b"__ddstats__")
        }
    }
}

pub fn read_stats_data_block(handle: &Handle, params: &ConnectionParams, pointers: &mut Pointers) -> anyhow::Result<StatsDataBlock> {
    let base = if pointers.base_address.is_none() { base_addr(handle, params)? } else { *pointers.base_address.as_ref().unwrap() };
    pointers.base_address = Some(base);
    BLOCK_BUF.with(|buf| {
        let pointer;
        if let Some(ddstats_ptr) = pointers.ddstats_block {
            pointer = ddstats_ptr;
        } else {
            pointers.ddstats_block = Some(calc_pointer_ddstats_block(handle, params, base)?);
            pointer = *pointers.ddstats_block.as_ref().unwrap();
        }
        let mut buf = buf.borrow_mut();
        handle.copy_address(pointer, buf.as_mut())?;
        if !buf.starts_with(b"__ddstats__") {
            return Err(anyhow::anyhow!(std::io::Error::new(std::io::ErrorKind::InvalidData, "No ddstats block found at address")));
        }
        let (_head, body, _tail) = unsafe { buf.as_mut().align_to::<StatsDataBlock>() };
        Ok(body[0].clone())
    })
}

#[cfg(target_os = "windows")]
pub fn start_dd() -> anyhow::Result<()> {
    use std::process::Command;
    Command::new("cmd").arg("/c start steam://run/422970").output()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn start_dd() -> anyhow::Result<()> {
    use std::process::Command;
    Command::new("steam").arg("steam://run/422970").output()?;
    Ok(())
}

