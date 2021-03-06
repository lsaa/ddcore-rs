//
// spawnsets
//

use std::{io::{Read, Write}, mem::size_of};
use anyhow::Result;
use crate::utils::{align_bytes, as_bytes, writer_buf};

#[derive(Debug, Clone)]
pub struct Spawnset<SpawnType> {
    pub header: Header,
    pub arena: Arena,
    pub spawns_header: SpawnsHeader,
    pub spawns: Vec<Spawn<SpawnType>>,
    pub settings: Option<Settings>
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum V3Enemies {
    Squid1 = 0,
    Squid2 = 1,
    Centipede = 2,
    Spider1 = 3,
    Leviathan = 4,
    Gigapede = 5,
    Squid3 = 6,
    Thorn = 7,
    Spider2 = 8,
    Ghostpede = 9,
    Empty = -1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum V2Enemies {
    Squid1 = 0,
    Squid2 = 1,
    Centipede = 2,
    Spider1 = 3,
    Leviathan = 4,
    Gigapede = 5,
    Squid3 = 6,
    Andras = 7,
    Spider2 = 8,
    Empty = -1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum V1Enemies {
    Squid1 = 0,
    Squid2 = 1,
    Centipede = 2,
    Spider1 = 3,
    Leviathan = 4,
    Gigapede = 5,
    Empty = -1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Spawn<SpawnType> {
    pub enemy_type: SpawnType,
    pub delay: f32,
    pub _u1: u32,
    pub _u2: u32,
    pub _u3: u32,
    pub _u4: u32,
    pub _u5: u32
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Header {
    pub spawn_version: i32,
    pub world_version: i32,
    pub shrink_end_radius: f32,
    pub shrink_start_radius: f32,
    pub shrink_rate: f32,
    pub brightness: f32,
    pub game_mode: i32,
    pub _u1: u32,
    pub _u2: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Arena {
    pub data: [f32; 51*51],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpawnsHeader {
    pub _u1: u32,
    pub _u2: u32,
    pub _u3: u32,
    pub _u4: u32,
    pub devil_dagger_time: i32,
    pub gold_dagger_time: i32,
    pub silver_dagger_time: i32,
    pub bronze_dagger_time: i32,
    pub _u5: u32,
    pub spawn_count: i32,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub initial_hand: u8,
    pub additional_gems: i32,
    pub timer_start: Option<f32>,
}

///////////////
/*** IMPLS ***/
///////////////

impl std::default::Default for V3Enemies {
    fn default() -> Self {
        V3Enemies::Empty
    }
}

impl std::default::Default for V2Enemies {
    fn default() -> Self {
        V2Enemies::Empty
    }
}

impl std::default::Default for V1Enemies {
    fn default() -> Self {
        V1Enemies::Empty
    }
}

impl<T: Default> std::default::Default for Spawn<T> {
    fn default() -> Self {
        Spawn {
            enemy_type: T::default(),
            delay: 0.0,
            _u1: 0,
            _u2: 3,
            _u3: 0,
            _u4: 1106247680,
            _u5: 10,
        }
    }
}

impl std::default::Default for Header {
    fn default() -> Self {
        Header {
            spawn_version: 6,
            world_version: 9,
            shrink_end_radius: 20.,
            shrink_start_radius: 50.,
            shrink_rate: 0.025,
            brightness: 60.,
            game_mode: 0,
            _u1: 51,
            _u2: 1,
        }
    }
}

impl std::default::Default for Arena {
    fn default() -> Self {
        Arena {
            data: [-1000.; 51*51],
        }
    }
}

// Accessors 2D -> 1D
impl Arena {
    pub fn get_tile(&self, x: u16, y: u16) -> &f32 {
        &self.data[y as usize * 51 + x as usize]
    }

    pub fn get_tile_mut(&mut self, x: u16, y: u16) -> &mut f32 {
        &mut self.data[y as usize * 51 + x as usize]
    }
}

impl std::default::Default for SpawnsHeader {
    fn default() -> Self {
        SpawnsHeader {
            devil_dagger_time: 500,
            gold_dagger_time: 250,
            silver_dagger_time: 120,
            bronze_dagger_time: 60,
            spawn_count: 0,
            _u1: 0,
            _u2: 0,
            _u3: 0,
            _u4: 1,
            _u5: 0,
        }
    }
}

impl std::default::Default for Settings {
    fn default() -> Self {
        Settings {
            additional_gems: 0,
            initial_hand: 0,
            timer_start: Some(0.0),
        }
    }
}

impl<SpawnType: Clone> Spawnset<SpawnType> {
    pub fn deserialize<R: Read>(source: &mut R) -> Result<Self> {
        unsafe {
            let mut header: Header = std::mem::zeroed();
            let mut arena: Arena = std::mem::zeroed();
            let mut spawns_header: SpawnsHeader = std::mem::zeroed();
            let header_buf = writer_buf::<Header>(&mut header);
            let arena_buf = writer_buf::<Arena>(&mut arena);
            let spawns_header_buf = writer_buf::<SpawnsHeader>(&mut spawns_header);
            source.read_exact(header_buf)?;
            source.read_exact(arena_buf)?;
            source.read_exact(spawns_header_buf)?;
            let spawns_len = size_of::<Spawn<SpawnType>>() * spawns_header.spawn_count as usize;
            let mut spawns_buf = vec![0u8; spawns_len];
            source.read_exact(&mut spawns_buf)?;
            let spawns: &[Spawn<SpawnType>] = align_bytes(&spawns_buf);
            let mut settings = None;
            if header.spawn_version >= 5 {
                let mut b1 = [0u8; 1];
                let mut b2 = [0u8; 4];
                source.read_exact(&mut b1)?;
                source.read_exact(&mut b2)?;
                settings = Some(Settings {
                    initial_hand: u8::from_le_bytes(b1),
                    additional_gems: i32::from_le_bytes(b2),
                    timer_start: None,
                });
            }
            if header.spawn_version >= 6 {
                if let Some(sett) = &mut settings {
                    let mut b2 = [0u8; 4];
                    source.read_exact(&mut b2)?;
                    sett.timer_start = Some(f32::from_le_bytes(b2));
                }
            }

            Ok(Spawnset {
                header,
                arena,
                spawns_header,
                spawns: spawns.to_vec(),
                settings,
            })
        }
    }

    pub fn serialize<W: Write>(&self, sink: &mut W) -> Result<()> {
        // Safe unconditionally as it's only translating the structs to bytes
        unsafe {
            sink.write_all(as_bytes(&self.header))?;
            sink.write_all(as_bytes(&self.arena))?;
            sink.write_all(as_bytes(&self.spawns_header))?;
            for spawn in &self.spawns {
                sink.write_all(as_bytes(spawn))?;
            }
        }
        if let Some(settings) = &self.settings {
            if self.header.spawn_version >= 5 {
                sink.write_all(&settings.initial_hand.to_le_bytes())?;
                sink.write_all(&settings.additional_gems.to_le_bytes())?;
            }
            if let Some(timer_start) = settings.timer_start {
                if self.header.spawn_version >= 6 {
                    sink.write_all(&timer_start.to_le_bytes())?;
                }
            }
        }
        sink.flush()?;
        Ok(())
    }

    pub fn recalculate_spawn_count(&mut self) {
        self.spawns_header.spawn_count = self.spawns.len() as i32;
    }
}

