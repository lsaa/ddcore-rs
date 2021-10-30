//
// Models
//

pub mod replay;

use crate::utils;
use std::fmt::Write;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(C)]
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct StatsDataBlock {
    marker: [u8; 11],
    pub ddstats_version: i32,
    pub player_id: i32,
    pub username: [u8; 32],
    pub time: f32,
    pub gems_collected: i32,
    pub kills: i32,
    pub daggers_fired: i32,
    pub daggers_hit: i32,
    pub enemies_alive: i32,
    pub level_gems: i32,
    pub homing: i32,
    pub gems_despawned: i32,
    pub gems_eaten: i32,
    pub gems_total: i32,
    pub daggers_eaten: i32,
    pub per_enemy_alive_count: [i16; 17],
    pub per_enemy_kill_count: [i16; 17],
    pub is_player_alive: bool,
    pub is_replay: bool,
    pub death_type: u8,
    pub is_in_game: bool,
    pub replay_player_id: i32,
    pub replay_player_name: [u8; 32],
    pub survival_md5: [u8; 16],
    pub time_lvl2: f32,
    pub time_lvl3: f32,
    pub time_lvl4: f32,
    pub levi_down_time: f32,
    pub orb_down_time: f32,
    pub status: i32, // 0 = Intro Screen | 1 = Main Menu | 2 = InGame | 3 = DEAD
    pub max_homing: i32,
    pub time_max_homing: f32, // gets updated every gem you get even if you dont have any homing
    pub enemies_alive_max: i32, // doesn't get reset sometimes when restarting a run
    pub time_enemies_alive_max: f32,
    pub time_max: f32,       // Max time of replay / current time in-game
    padding1: [u8; 4],       // fun
    pub stats_base: [u8; 8], // Pointer to frames
    pub stats_frames_loaded: i32,
    pub stats_finished_loading: bool,
    padding2: [u8; 3],
    pub starting_hand: i32,
    pub starting_homing: i32,
    pub starting_time: f32,
    pub prohibited_mods: bool,
    padding3: [u8; 3],
    pub replay_base: [u8; 8],
    pub replay_buffer_length: i32,
    pub replay_flag: bool
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct StatsFrame {
    pub gems_collected: i32,
    pub kills: i32,
    pub daggers_fired: i32,
    pub daggers_hit: i32,
    pub enemies_alive: i32,
    pub level_gems: i32,
    pub homing: i32,
    pub gems_despawned: i32,
    pub gems_eaten: i32,
    pub gems_total: i32,
    pub daggers_eaten: i32,
    pub per_enemy_alive_count: [i16; 17],
    pub per_enemy_kill_count: [i16; 17],
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct StatsBlockWithFrames {
    pub block: StatsDataBlock,
    pub frames: Vec<StatsFrame>,
}

impl StatsDataBlock {
    pub fn player_username(&self) -> String {
        utils::byte_array_to_string(&self.username[..]).unwrap_or("unknown".into())
    }

    pub fn replay_player_username(&self) -> String {
        utils::byte_array_to_string(&self.replay_player_name[..]).unwrap_or("unknown".into())
    }

    pub fn level_hash(&self) -> String {
        let mut s = String::with_capacity(2 * self.survival_md5.len());
        for byte in self.survival_md5 {
            write!(s, "{:02X}", byte).expect("Couldn't decode hash byte");
        }
        return s;
    }

    pub fn get_stats_pointer(&self) -> usize {
        i64::from_le_bytes(self.stats_base) as usize
    }

    pub fn get_replay_pointer(&self) -> usize {
        i64::from_le_bytes(self.replay_base) as usize
    }

    pub fn status(&self) -> GameStatus {
        FromPrimitive::from_i32(self.status).unwrap()
    }
}

impl StatsBlockWithFrames {
    #[rustfmt::skip]
    pub fn get_frame_for_time(&self, time: f32) -> Option<&StatsFrame> {
        let real_time = time - self.block.starting_time;
        if real_time <= 0. { return None; }
        if real_time + 1. > self.frames.len() as f32 { return None; }
        return Some(&self.frames[real_time as usize]);
    }

    pub fn get_frames_until_time(&self, mut time: f32) -> Vec<&StatsFrame> {
        let mut res = vec![];
        if time as usize + 1 > self.frames.len() {
            time = self.frames.len() as f32 - 1.;
        }
        res.extend(self.frames[..time as usize].iter());
        res
    }

    #[rustfmt::skip]
    pub fn homing_usage_from_frames(&self, time: Option<f32>) -> u32 {
        let mut neg_diff_lvl3 = 0;
        let mut neg_diff_lvl4 = 0;
        let mut last_frame_homing_lvl3 = 0;
        let mut last_frame_homing_lvl4 = 0;
        let cutoff = if time.is_none() { f32::MAX } else { time.unwrap() };
        for frame in &self.get_frames_until_time(cutoff) {
            if frame.level_gems == 70 {
                if frame.homing < last_frame_homing_lvl3 {
                    neg_diff_lvl3 += -(frame.homing - last_frame_homing_lvl3);
                }
                last_frame_homing_lvl3 = frame.homing;
            } else if frame.level_gems == 71 {
                if frame.homing < last_frame_homing_lvl4 {
                    neg_diff_lvl4 += -(frame.homing - last_frame_homing_lvl4);
                }
                last_frame_homing_lvl4 = frame.homing;
            }
        }
        (neg_diff_lvl3 + neg_diff_lvl4) as u32
    }
}

#[derive(FromPrimitive, Debug, PartialEq, Clone, Copy)]
pub enum GameStatus {
    Title = 0,
    Menu,
    Lobby,
    Playing,
    Dead,
    OwnReplayFromLastRun,
    OwnReplayFromLeaderboard,
    OtherReplay,
    LocalReplay,
}
