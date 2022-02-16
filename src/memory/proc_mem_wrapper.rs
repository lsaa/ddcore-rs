//
//  We have fun around these parts
//

use anyhow::Result;

#[cfg(target_os = "windows")]
use std::os::windows::prelude::OwnedHandle;
#[cfg(target_os = "windows")]
use process_memory::Architecture;

#[cfg(target_os = "linux")]
use process_memory::{CopyAddress, ProcessHandle, ProcessHandleExt, TryIntoProcessHandle};

#[cfg(target_os = "windows")]
pub struct Handle {
    pub win_handle: OwnedHandle,
    pub pid: usize,
}

#[cfg(target_os = "linux")]
pub struct Handle {
    pub inner: ProcessHandle,
    pub pid: usize,
}

impl Handle {
    #[cfg(target_os = "windows")]
    pub fn new(pid: usize) -> Result<Self> {
        use std::os::windows::prelude::FromRawHandle;

        use winapi::shared::minwindef::DWORD;
        let handle = unsafe {
            winapi::um::processthreadsapi::OpenProcess(
                winapi::um::winnt::PROCESS_ALL_ACCESS,
                winapi::shared::minwindef::FALSE,
                pid as DWORD,
            )
        };
        if handle == (0 as winapi::um::winnt::HANDLE) {
            Err(anyhow::anyhow!(std::io::Error::last_os_error()))
        } else {
            let handle = unsafe { OwnedHandle::from_raw_handle(handle) };
            Ok(Handle { win_handle: handle, pid })
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_offset(&self, offsets: &[usize]) -> Result<usize> {
        let mut offset: usize = 0;
        let noffsets: usize = offsets.len();
        let arch = Architecture::from_native();
        let mut copy = vec![0_u8; arch as usize];
        for next_offset in offsets.iter().take(noffsets - 1) {
            offset += next_offset;
            self.copy_address(offset, &mut copy)?;
            offset = arch.pointer_from_ne_bytes(&copy);
        }

        offset += offsets[noffsets - 1];
        Ok(offset)
    }

    #[cfg(target_os = "windows")]
    pub fn check_handle(&self) -> bool {
        use std::os::windows::prelude::AsRawHandle;
        self.win_handle.as_raw_handle().is_null()
    }

    #[cfg(target_os = "windows")]
    pub fn copy_address(&self, addr: usize, buf: &mut [u8]) -> Result<()> {
        use std::os::windows::prelude::AsRawHandle;
        use winapi::shared::minwindef;
        if buf.is_empty() {
            return Ok(());
        }

        if unsafe {
            winapi::um::memoryapi::ReadProcessMemory(
                self.win_handle.as_raw_handle(),
                addr as minwindef::LPVOID,
                buf.as_mut_ptr() as minwindef::LPVOID,
                buf.len() as winapi::shared::basetsd::SIZE_T,
                std::ptr::null_mut(),
            )
        } == winapi::shared::minwindef::FALSE
        {
            Err(anyhow::anyhow!(std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    #[cfg(target_os = "windows")]
    pub fn put_address(&self, addr: usize, buf: &[u8]) -> Result<()> {
        use std::os::windows::prelude::AsRawHandle;
        use winapi::shared::minwindef;

        if buf.is_empty() {
            return Ok(());
        }
        if unsafe {
            winapi::um::memoryapi::WriteProcessMemory(
                self.win_handle.as_raw_handle(),
                addr as minwindef::LPVOID,
                buf.as_ptr() as minwindef::LPCVOID,
                buf.len() as winapi::shared::basetsd::SIZE_T,
                std::ptr::null_mut(),
            )
        } == winapi::shared::minwindef::FALSE
        {
            Err(anyhow::anyhow!(std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    #[cfg(target_os = "windows")]
    pub fn null_type() -> Self {
        use std::os::windows::prelude::FromRawHandle;
        Self {
            win_handle: unsafe { OwnedHandle::from_raw_handle(std::ptr::null_mut()) },
            pid: 0,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn new(pid: usize) -> Result<Self> {
        Ok(Self {
            inner: (pid as i32).try_into_process_handle()?,
            pid
        })
    }

    #[cfg(target_os = "linux")]
    pub fn null_type() -> Self {
        Self {
            inner: ProcessHandle::null_type(),
            pid: 0
        }
    }

    #[cfg(target_os = "linux")]
    pub fn copy_address(&self, addr: usize, buf: &mut [u8]) -> Result<()> {
        Ok(self.inner.copy_address(addr, buf)?)
    }

    #[cfg(target_os = "linux")]
    pub fn put_address(&self, addr: usize, buf: &[u8]) -> Result<()> {
        use process_memory::PutAddress;
        self.inner.put_address(addr, buf)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn get_offset(&self, offsets: &[usize]) -> Result<usize> {
        Ok(self.inner.get_offset(offsets)?)
    }

    #[cfg(target_os = "linux")]
    pub fn check_handle(&self) -> bool {
        self.inner.check_handle()
    }
}
