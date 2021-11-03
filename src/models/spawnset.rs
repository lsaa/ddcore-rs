//
// spawnsets
//

use std::{io::{Read, Sink, Write}, mem::size_of};
use anyhow::Result;
use crate::utils::{align_bytes, as_bytes, writer_buf};

#[derive(Debug)]
pub struct Spawnset<SpawnType> {
    pub header: Header,
    pub arena: Arena,
    pub spawns_header: SpawnsHeader,
    pub spawns: Vec<Spawn<SpawnType>>,
    pub settings: Option<Settings>
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
    _u1: u32,
    _u2: u32,
    _u3: u32,
    _u4: u32,
    _u5: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct Header {
    pub spawn_version: i32,
    pub world_version: i32,
    pub shrink_end_radius: f32,
    pub shrink_start_radius: f32,
    pub shrink_rate: f32,
    pub brightness: f32,
    pub game_mode: i32,
    _u1: u32,
    _u2: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Arena {
    pub data: [f32; 51*51],
}

#[repr(C)]
#[derive(Debug)]
pub struct SpawnsHeader {
    _u1: u32,
    _u2: u32,
    _u3: u32,
    _u4: u32,
    pub devil_dagger_time: i32,
    pub gold_dagger_time: i32,
    pub silver_dagger_time: i32,
    pub bronze_dagger_time: i32,
    _u5: u32,
    pub spawn_count: i32,
}

#[derive(Debug)]
pub struct Settings {
    pub initial_hand: u8,
    pub additional_gems: i32,
    pub timer_start: Option<f32>,
}

impl<SpawnType: Clone> Spawnset<SpawnType> {
    pub unsafe fn deserialize<R: Read>(source: &mut R) -> Result<Self> {
        let mut header: Header = std::mem::zeroed();
        let mut arena: Arena = std::mem::zeroed();
        let mut spawns_header: SpawnsHeader = std::mem::zeroed();
        let mut header_buf = writer_buf::<Header>(&mut header);
        let mut arena_buf = writer_buf::<Arena>(&mut arena);
        let mut spawns_header_buf = writer_buf::<SpawnsHeader>(&mut spawns_header);
        source.read(&mut header_buf)?;
        source.read(&mut arena_buf)?;
        source.read(&mut spawns_header_buf)?;
        let spawns_len = size_of::<Spawn<SpawnType>>() * spawns_header.spawn_count as usize;
        let mut spawns_buf = vec![0u8; spawns_len];
        source.read(&mut spawns_buf)?;
        let spawns: &[Spawn<SpawnType>] = align_bytes(&spawns_buf);
        let mut settings = None;
        if header.spawn_version >= 5 {
            let mut b1 = [0u8; 1];
            let mut b2 = [0u8; 4];
            source.read(&mut b1)?;
            source.read(&mut b2)?;
            settings = Some(Settings {
                initial_hand: u8::from_le_bytes(b1),
                additional_gems: i32::from_le_bytes(b2),
                timer_start: None,
            });
        }
        if header.spawn_version >= 6 {
            if let Some(sett) = &mut settings {
                let mut b2 = [0u8; 4];
                source.read(&mut b2)?;
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

    pub fn serialize<W: std::io::Write>(&self, sink: &mut W) -> Result<()> {
        sink.write(as_bytes(&self.header))?;
        sink.write(as_bytes(&self.arena))?;
        sink.write(as_bytes(&self.spawns_header))?;
        for spawn in &self.spawns {
            sink.write(as_bytes(spawn))?;
        }
        if let Some(settings) = &self.settings {
            if self.header.spawn_version >= 5 {
                sink.write(&settings.initial_hand.to_le_bytes())?;
                sink.write(&settings.additional_gems.to_le_bytes())?;
            }
            if let Some(timer_start) = settings.timer_start {
                if self.header.spawn_version >= 6 {
                    sink.write(&timer_start.to_le_bytes())?;
                }
            }
        }
        Ok(())
    }
}

