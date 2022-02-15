//
// replay file models and utils
//

use std::{io::Read, time::{SystemTime, Duration, UNIX_EPOCH}};
use anyhow::{Result, bail};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::spawnset::V3Enemies;

type EntityId = i32;
type PositionInt = [i16; 3];
type PositionFloat = [f32; 3];
type LeviathanData = i32;

#[derive(Debug, Clone)]
pub struct DfRpl2 {
    pub header: DfRpl2Header,
    pub data: ReplayData
}

#[derive(Debug, Clone)]
pub struct ReplayData {
    pub frames: Vec<ReplayFrame>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct DfRpl2Header {
    pub player_name: String,
    pub funny_bytes: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct DdRpl {
    pub header: DdRplHeader,
    pub compressed_data: Option<Vec<u8>>,
    pub data: Option<ReplayData>,
    pub extra: Option<ExtraData>,
}

#[derive(Debug, Clone)]
pub struct ExtraData {
    pub homing: Vec<i32>,
    pub homing_used: Vec<i32>,
    pub starting_gems: i32,
    pub starting_hand: u8,
    pub look_speed: f32,
    pub lvl2_time: f32,
    pub lvl3_time: f32,
    pub lvl4_time: f32,
}

#[derive(Debug, Clone)]
pub struct DdRplHeader {
    pub file_version: u32,
    pub recorded_at: SystemTime,
    pub time: f32,
    pub starting_time: f32,
    pub daggers_fired: u32,
    pub death_type: i32,
    pub gems: u32,
    pub kills: u32,
    pub daggers_hit: u32,
    pub player_name: String,
    pub player_id: i32,
    pub spawnset_bin: Vec<u8>,
    pub spawnset_hash: String,
    pub compressed_data_len: u32,
    pub spawnset: Option<crate::models::spawnset::Spawnset<V3Enemies>>
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone)]
pub struct ReplayFrame {
    pub events: Vec<ReplayEvent>
}

#[derive(Debug, Clone)]
pub enum ReplayEvent {
    Spawn(EntityData),
    UpdateEntityPosition(EntityId, PositionInt),
    UpdateEntityOrientation(EntityId, UpdateOrientationData),
    UpdateEntityTarget(EntityId, [i16; 3]),
    DaggerDewspawn(DaggerDespawnData),
    EnemyHitWeakSpot(EnemyHitData),
    EnemyHitArmor(EnemyHitData),
    PlayerDeath(PlayerDeathData),
    GemPickup,
    Transmute(EntityId, TransmuteData),
    EndFrame(ButtonData, MouseData),
    EndReplay,
}

#[derive(Debug, Clone, FromPrimitive)]
pub enum EntityType {
    Dagger = 0x1,
    Squid1 = 0x3,
    Squid2 = 0x4,
    Squid3 = 0x5,
    Boid = 0x6, // Skulls and Spiderlings
    Centipede = 0x7,
    Spider1 = 0x8,
    Spider2 = 0x9,
    Egg = 0xA,
    Leviathan = 0xB,
    Gigapede = 0xC,
    Thorn = 0xD,
    Ghostpede = 0xF,
}

#[derive(Debug, Clone)]
pub enum EntityData {
    Dagger(DaggerData),
    Squid1(SquidData),
    Squid2(SquidData),
    Squid3(SquidData),
    Boid(BoidData), // Skulls and Spiderlings
    Centipede(PedeData),
    Spider1(SpiderData),
    Spider2(SpiderData),
    Egg(EggData),
    Leviathan(LeviathanData),
    Gigapede(PedeData),
    Thorn(ThornData),
    Ghostpede(PedeData),
}

#[derive(Debug, Clone)]
pub struct ThornData {
    pub a: i32,
    pub position: PositionFloat,
    pub rotation: f32,
}

#[derive(Debug, Clone)]
pub struct EggData {
    pub spider_spawner: EntityId,
    pub funny1: [f32; 3],
    pub funny2: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct SpiderData {
    pub a: i32,
    pub position: PositionFloat,
}

#[derive(Debug, Clone)]
pub struct PedeData {
    pub a: i32,
    pub position: PositionFloat,
    pub b: [f32; 3],
    pub funny1: [f32; 3],
    pub funny2: [f32; 3],
    pub funny3: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct BoidData {
    pub boid_type: BoidType,
    pub spanwer: EntityId,
    pub position: PositionInt,
    pub funny1: [i16; 3],
    pub funny2: [i16; 3],
    pub funny3: [i16; 3],
    pub funny4: [f32; 3],
    pub speed: f32,
}

#[derive(Debug, Clone, FromPrimitive)]
pub enum BoidType {
    Skull1 = 1,
    Skull2 = 2,
    Skull3 = 3,
    Skull4 = 5,
    Spiderling = 4,
}

#[derive(Debug, Clone)]
pub struct DaggerData {
    pub a: i32,
    pub position: PositionInt,
    pub orientationa: [i16; 3],
    pub orientationb: [i16; 3],
    pub orientationc: [i16; 3],
    pub b: u8,
    pub dagger_level: DaggerLevel,
}

#[derive(Debug, Clone)]
pub struct SquidData {
    pub a: i32,
    pub position: PositionFloat,
    pub b: [f32; 3],
    pub rotation: f32, // Radians
}

#[derive(Debug, Clone, FromPrimitive, PartialEq)]
pub enum DaggerLevel {
    Level0 = 0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

#[derive(Debug, Clone)]
pub struct UpdateOrientationData {
    pub a: [i16; 3],
    pub b: [i16; 3],
    pub c: [i16; 3],
}

#[derive(Debug, Clone)]
pub struct DaggerDespawnData {
    pub dagger_id: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerDeathData {
    pub death_type: i32,
}

#[derive(Debug, Clone)]
pub struct EnemyHitData {
    pub enemy_id: i32,
    pub dagger_id: i32,
    pub segment: i32,
}

#[derive(Debug, Clone)]
pub struct TransmuteData {
    pub a: [i16; 3],
    pub b: [i16; 3],
    pub c: [i16; 3],
    pub d: [i16; 3],
}

#[derive(Debug, Clone)]
pub struct ButtonData {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub backwards: bool,
    pub jump: JumpButtonState,
    pub shoot: MouseButtonState,
    pub homing: MouseButtonState,
}

#[derive(Debug, Clone)]
pub enum JumpButtonState {
    NotPressed = 0,
    Held,
    JustPressed,
}

#[derive(Debug, Clone)]
pub enum MouseButtonState {
    NotPressed = 0,
    Held,
    Released,
}

#[derive(Debug, Clone)]
pub struct MouseData {
    pub x: i16,
    pub y: i16,
    pub look_speed: Option<f32>,
}

impl ReplayData {

    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn from_reader<R: Read>(source: &mut R) -> Result<Self> {
        use bytestream::*;
        let mut decompressed = vec![];
        libflate::zlib::Decoder::new(source)?.read_to_end(&mut decompressed)?;
        let mut event_reader = &decompressed[..];


        let mut next_entity_id: EntityId = 1;
        let mut entities: Vec<Entity> = vec![];
        let mut frames: Vec<ReplayFrame> = vec![];
        let mut first = true;
        let mut current_frame: Vec<ReplayEvent> = vec![];

        loop {
            let event_type = u8::read_from(&mut event_reader, ByteOrder::LittleEndian)?;

            let event: ReplayEvent = match event_type {
                0x0 => {
                    let entity_type = u8::read_from(&mut event_reader, ByteOrder::LittleEndian)?;
                    let entity_type: EntityType = FromPrimitive::from_u8(entity_type).unwrap();
                    
                    entities.push(Entity {
                        id: next_entity_id,
                        entity_type: entity_type.clone()
                    });

                    next_entity_id += 1;

                    ReplayEvent::Spawn(match entity_type {
                        EntityType::Dagger => EntityData::Dagger(DaggerData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_i16(&mut event_reader)?,
                            orientationa: read3_i16(&mut event_reader)?,
                            orientationb: read3_i16(&mut event_reader)?,
                            orientationc: read3_i16(&mut event_reader)?,
                            b: u8::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                            dagger_level: FromPrimitive::from_u8(u8::read_from(&mut event_reader, ByteOrder::LittleEndian)?).unwrap(),
                        }),
                        EntityType::Squid1 => EntityData::Squid1(SquidData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            rotation: read_f32(&mut event_reader)?
                        }),
                        EntityType::Squid2 => EntityData::Squid2(SquidData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            rotation: read_f32(&mut event_reader)?
                        }),
                        EntityType::Squid3 => EntityData::Squid3(SquidData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            rotation: read_f32(&mut event_reader)?
                        }),
                        EntityType::Boid => EntityData::Boid(BoidData {
                            spanwer: read_entity_id(&mut event_reader)?,
                            boid_type: FromPrimitive::from_u8(u8::read_from(&mut event_reader, ByteOrder::LittleEndian)?).unwrap(),
                            position: read3_i16(&mut event_reader)?,
                            funny1: read3_i16(&mut event_reader)?,
                            funny2: read3_i16(&mut event_reader)?,
                            funny3: read3_i16(&mut event_reader)?,
                            funny4: read3_f32(&mut event_reader)?,
                            speed: read_f32(&mut event_reader)?,
                        }),
                        EntityType::Centipede => EntityData::Centipede(PedeData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            funny1: read3_f32(&mut event_reader)?,
                            funny2: read3_f32(&mut event_reader)?,
                            funny3: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Gigapede => EntityData::Gigapede(PedeData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            funny1: read3_f32(&mut event_reader)?,
                            funny2: read3_f32(&mut event_reader)?,
                            funny3: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Ghostpede => EntityData::Ghostpede(PedeData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            b: read3_f32(&mut event_reader)?,
                            funny1: read3_f32(&mut event_reader)?,
                            funny2: read3_f32(&mut event_reader)?,
                            funny3: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Spider1 => EntityData::Spider1(SpiderData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Spider2 => EntityData::Spider2(SpiderData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Egg => EntityData::Egg(EggData {
                            spider_spawner: read_entity_id(&mut event_reader)?,
                            funny1: read3_f32(&mut event_reader)?,
                            funny2: read3_f32(&mut event_reader)?,
                        }),
                        EntityType::Thorn => EntityData::Thorn(ThornData {
                            a: read_entity_id(&mut event_reader)?,
                            position: read3_f32(&mut event_reader)?,
                            rotation: read_f32(&mut event_reader)?,
                        }),
                        EntityType::Leviathan => EntityData::Leviathan(read_entity_id(&mut event_reader)?),
                    })
                },
                0x1 => ReplayEvent::UpdateEntityPosition(
                    read_entity_id(&mut event_reader)?,
                    read3_i16(&mut event_reader)?
                ),
                0x2 => ReplayEvent::UpdateEntityOrientation(
                    read_entity_id(&mut event_reader)?,
                    UpdateOrientationData {
                        a: read3_i16(&mut event_reader)?,
                        b: read3_i16(&mut event_reader)?,
                        c: read3_i16(&mut event_reader)?,
                    }
                ),
                0x4 => ReplayEvent::UpdateEntityTarget(
                    read_entity_id(&mut event_reader)?,
                    read3_i16(&mut event_reader)?
                ),
                0x5 => {
                    let a = i32::read_from(&mut event_reader, ByteOrder::LittleEndian)?;
                    let b = i32::read_from(&mut event_reader, ByteOrder::LittleEndian)?;
                    let c = i32::read_from(&mut event_reader, ByteOrder::LittleEndian)?;

                    if a == 0 {
                        ReplayEvent::PlayerDeath(PlayerDeathData {
                            death_type: b
                        })
                    } else if b == 0 && c == 0 {
                        ReplayEvent::DaggerDewspawn(DaggerDespawnData {
                            dagger_id: a
                        })
                    } else if a < 0 {
                        ReplayEvent::EnemyHitArmor(EnemyHitData {
                            enemy_id: -a,
                            dagger_id: b,
                            segment: c
                        })
                    } else {
                        ReplayEvent::EnemyHitWeakSpot(EnemyHitData {
                            enemy_id: a,
                            dagger_id: b,
                            segment: c
                        })
                    }
                },
                0x6 => ReplayEvent::GemPickup,
                0x7 => ReplayEvent::Transmute(
                    read_entity_id(&mut event_reader)?,
                    TransmuteData {
                        a: read3_i16(&mut event_reader)?,
                        b: read3_i16(&mut event_reader)?,
                        c: read3_i16(&mut event_reader)?,
                        d: read3_i16(&mut event_reader)?,
                    }
                ),
                0x9 => {
                    let buttons = ButtonData {
                        left: bool::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        right: bool::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        forward: bool::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        backwards: bool::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        jump: read_jump(&mut event_reader)?,
                        shoot: read_mouse_btn(&mut event_reader)?,
                        homing: read_mouse_btn(&mut event_reader)?,
                    };

                    let mut mouse_data = MouseData {
                        x: i16::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        y: i16::read_from(&mut event_reader, ByteOrder::LittleEndian)?,
                        look_speed: None,
                    };

                    if first {
                        mouse_data.look_speed = Some((500. / 3.) * read_f32(&mut event_reader)?);
                        first = false;
                    }

                    if let Ok(funny) = u8::read_from(&mut event_reader, ByteOrder::LittleEndian) {
                        if funny != 0xA {
                            bail!("FUNNY BYTE!");
                        }
                    }

                    ReplayEvent::EndFrame(
                        buttons,
                        mouse_data
                    )
                },
                0xB | _ => ReplayEvent::EndReplay
            };

            current_frame.push(event.clone());
            match event {
                ReplayEvent::EndReplay => {
                    frames.push(ReplayFrame {
                        events: current_frame,
                    });
                    break;
                },
                ReplayEvent::EndFrame(_cool, _cool2) => {
                    frames.push(ReplayFrame {
                        events: current_frame,
                    });
                    current_frame = vec![];
                }
                _ => {},
            }
        }

        Ok(Self {
            frames,
            entities
        })
    }
}

impl DdRplHeader {
    pub fn create_spawnset(&mut self) -> Result<()> {
        let bin_reader = self.spawnset_bin.clone();
        let spawnset = crate::models::spawnset::Spawnset::<V3Enemies>::deserialize(&mut &bin_reader[..])?;
        self.spawnset = Some(spawnset);
        Ok(())
    }
}

impl DdRpl {
    pub fn calc_data(&mut self) -> Result<()> {
        if let Some(compressed_data) = &self.compressed_data {
            self.data = Some(ReplayData::from_reader(&mut &compressed_data[..])?);
            self.compressed_data = None;
            Ok(())
        } else {
            bail!("No compressed data");
        }
    }

    pub fn create_extra(&mut self) -> Result<()> {
        if self.data.is_none() {
            self.calc_data()?;
        }

        if self.header.spawnset.is_none() {
            self.header.create_spawnset()?;
        }

        let spawnset = self.header.spawnset.as_ref().unwrap().clone();
        let mut old_extra = self.extra.as_ref().unwrap().clone();
        if self.extra.is_none() {
            old_extra = ExtraData {
                homing: vec![],
                homing_used: vec![],
                starting_hand: 0,
                starting_gems: 0,
                lvl2_time: 0.,
                lvl3_time: 0.,
                lvl4_time: 0.,
                look_speed: 0.,
            };
        }
        let initial_hand = if spawnset.settings.is_some() { spawnset.settings.as_ref().unwrap().initial_hand } else { 0 };
        let additional_gems = if spawnset.settings.is_some() { spawnset.settings.as_ref().unwrap().additional_gems } else { 0 };

        old_extra.starting_hand = initial_hand;
        old_extra.starting_gems = additional_gems;

        let mut _gem_counter = old_extra.starting_gems;
        let mut hand_level = old_extra.starting_hand;
        #[allow(unused_assignments)]
        let mut level_gems = 0;
        let mut homing = 0;
        let mut homing_used = 0;

        match &old_extra.starting_hand {
            3 => { level_gems = 70; homing = old_extra.starting_gems },
            4 => { level_gems = 71; homing = old_extra.starting_gems },
            _ => { level_gems = old_extra.starting_gems },
        };

        let mut homing_history = vec![];
        let mut homing_used_history = vec![];

        let data = self.data.as_ref().unwrap();

        for (frame_count, frame) in data.frames.iter().enumerate() {
            for event in &frame.events {
                match event {
                    ReplayEvent::EndFrame(_button_data, mouse_data) => {
                        if mouse_data.look_speed.is_some() {
                            old_extra.look_speed = mouse_data.look_speed.unwrap();
                        }
                    },
                    ReplayEvent::GemPickup => {
                        _gem_counter += 1;
                        if hand_level <= 1 {
                            level_gems += 1;
                            if level_gems >= 10 {
                                old_extra.lvl2_time = (frame_count + 1) as f32 / 60.;
                                hand_level = 2;
                                level_gems = 10;
                            }
                        } else if hand_level == 2 {
                            level_gems += 1;
                            if level_gems >= 70 {
                                old_extra.lvl3_time = (frame_count + 1) as f32 / 60.;
                                hand_level = 3;
                                level_gems = 70;
                            }
                        } else if hand_level == 3 {
                            homing += 1;
                            if homing >= 150 {
                                old_extra.lvl4_time = (frame_count + 1) as f32 / 60.;
                                hand_level = 4;
                                level_gems = 71;
                                homing = 0;
                            }
                        } else {
                            homing += 1;
                        }
                    },
                    ReplayEvent::Spawn(EntityData::Dagger(dagger)) => {
                        if dagger.dagger_level.eq(&DaggerLevel::Level6) {
                            homing -= 1;
                            homing_used += 1;
                        }
                    },
                    _ => {},
                }
            }
            
            homing_history.push(homing);
            homing_used_history.push(homing_used);
        }

        old_extra.homing = homing_history;
        old_extra.homing_used = homing_used_history;

        self.extra = Some(old_extra);

        Ok(())
    }

    pub fn from_reader<R: Read>(source: &mut R) -> Result<Self> {
        use bytestream::*;

        source.read_exact(&mut [0u8; 0x6])?; // ddrpl.
        let file_version = u32::read_from(source, ByteOrder::LittleEndian)?;
        let timestamp = u64::read_from(source, ByteOrder::LittleEndian)?;
        let timestamp = UNIX_EPOCH + Duration::from_secs(1455753600 + timestamp);
        let time = read_f32(source)?;
        let starting_time = read_f32(source)?;
        let daggers_fired = u32::read_from(source, ByteOrder::LittleEndian)?;
        let death_type = i32::read_from(source, ByteOrder::LittleEndian)?;
        let gems_collected = u32::read_from(source, ByteOrder::LittleEndian)?;
        let daggers_hit = u32::read_from(source, ByteOrder::LittleEndian)?;
        let kills = u32::read_from(source, ByteOrder::LittleEndian)?;
        let player_id = i32::read_from(source, ByteOrder::LittleEndian)?;
        let username_len = u32::read_from(source, ByteOrder::LittleEndian)?;
        let mut username = vec![0u8; username_len as usize];
        source.read_exact(&mut username)?;
        let username = String::from_utf8(username)?;
        source.read_exact(&mut [0u8; 10])?; // skip unknown
        let mut spawnset_hash = [0u8; 16];
        source.read_exact(&mut spawnset_hash)?;
        let spawnset_hash = crate::utils::md5_to_string_lower(&spawnset_hash);
        let spawnset_len = u32::read_from(source, ByteOrder::LittleEndian)?;
        let mut spawnset_bin = vec![0u8; spawnset_len as usize];
        source.read_exact(&mut spawnset_bin)?;
        let compressed_data_len = u32::read_from(source, ByteOrder::LittleEndian)?;

        if compressed_data_len > 40000000 {
            bail!("Replay data is too big");
        }

        let header = DdRplHeader {
            player_name: username,
            player_id,
            spawnset_hash,
            kills,
            death_type,
            daggers_hit,
            daggers_fired,
            file_version,
            time,
            starting_time,
            gems: gems_collected,
            recorded_at: timestamp,
            spawnset_bin,
            compressed_data_len,
            spawnset: None,
        };
    
        let mut compressed_data = vec![0u8; compressed_data_len as usize];
        source.read_exact(&mut compressed_data[..])?;
        let compressed_data = Some(compressed_data);
        
        if source.read(&mut [0u8; 1])? != 0 {
            bail!("Invalid Replay File");
        }

        Ok(DdRpl {
            header,
            data: None,
            compressed_data,
            extra: None,
        })
    }
}

impl DfRpl2 {
    pub fn from_reader<R: Read>(source: &mut R) -> Result<Self> {
        use bytestream::*;

        // Skip DF_RPL2
        source.read_exact(&mut [0u8; 7])?;
        let username_len = u16::read_from(source, ByteOrder::LittleEndian)?;
        let mut username = vec![0u8; username_len as usize];
        source.read_exact(&mut username)?;
        let username = String::from_utf8(username)?;
        let funny_bytes_len = u16::read_from(source, ByteOrder::LittleEndian)?;
        let mut funny_bytes = vec![0u8; funny_bytes_len as usize];
        source.read_exact(&mut funny_bytes)?;

        let header = DfRpl2Header {
            player_name: username,
            funny_bytes
        };

        let data = ReplayData::from_reader(source)?;

        Ok(DfRpl2 {
            header,
            data
        })
    }
}

fn read_jump<R: Read>(source: &mut R) -> Result<JumpButtonState> {
    use bytestream::*;
    let v = u8::read_from(source, ByteOrder::LittleEndian)?;
    Ok(match v {
        0 => JumpButtonState::NotPressed,
        1 => JumpButtonState::Held,
        2 => JumpButtonState::JustPressed,
        _ => JumpButtonState::Held,
    })
}

fn read_mouse_btn<R: Read>(source: &mut R) -> Result<MouseButtonState> {
    use bytestream::*;
    let v = u8::read_from(source, ByteOrder::LittleEndian)?;
    Ok(match v {
        0 => MouseButtonState::NotPressed,
        1 => MouseButtonState::Held,
        2 => MouseButtonState::Released,
        _ => MouseButtonState::Held,
    })
}

fn read_entity_id<R: Read>(source: &mut R) -> Result<i32> {
    use bytestream::*;
    Ok(i32::read_from(source, ByteOrder::LittleEndian)?)
}

fn read3_i16<R: Read>(source: &mut R) -> Result<[i16; 3]> {
    use bytestream::*;
    Ok([
        i16::read_from(source, ByteOrder::LittleEndian)?,
        i16::read_from(source, ByteOrder::LittleEndian)?,
        i16::read_from(source, ByteOrder::LittleEndian)?,
    ])
}

fn read_f32<R: Read>(source: &mut R) -> Result<f32> {
    let mut buf = [0u8; 4];
    source.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read3_f32<R: Read>(source: &mut R) -> Result<[f32; 3]> {
    let mut b1 = [0u8; 4];
    let mut b2 = [0u8; 4];
    let mut b3 = [0u8; 4];
    source.read_exact(&mut b1)?;
    source.read_exact(&mut b2)?;
    source.read_exact(&mut b3)?;
    Ok([
        f32::from_le_bytes(b1),
        f32::from_le_bytes(b2),
        f32::from_le_bytes(b3),
    ])
}
