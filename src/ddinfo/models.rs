//
// models for ddinfo
//

////////////////////////////////// Marker
//////////////////////////////////

#[derive(Debug, serde::Serialize)]
pub enum OperatingSystem {
    Windows,
    Linux,
}

#[derive(serde::Deserialize, Debug)]
pub struct MarkerResponse {
    pub value: usize,
}

////////////////////////////////// Tool
//////////////////////////////////

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    pub display_name: String,
    pub version_number: String,
    pub version_number_required: String,
    pub changelog: Vec<ChangelogEntry>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogEntry {
    pub version_number: String,
    pub date: String,
    pub changes: Vec<Change>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub description: String,
    pub sub_changes: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DdstatsRustIntegration {
    pub required_version: String,
}

////////////////////////////////// Spawnsets
//////////////////////////////////

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpawnsetFile {
    pub max_display_waves: Option<i32>,
    pub html_description: Option<String>,
    pub last_updated: Option<String>,
    pub spawnset_data: SpawnsetData,
    pub name: String,
    pub author_name: String,
    pub has_custom_leaderboard: bool,
    pub is_practice: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpawnsetData {
    pub spawn_version: i32,
    pub world_version: i32,
    pub game_mode: GameMode,
    pub non_loop_spawn_count: i32,
    pub non_loop_length: Option<f32>,
    pub loop_length: Option<f32>,
    pub hand: Option<u8>,
    pub additional_gems: Option<i32>,
    pub time_start: Option<f32>,
}

#[derive(serde::Deserialize, Debug)]
pub enum GameMode {
    Default,
    TimeAttack,
}

////////////////////////////////// Leaderboards
//////////////////////////////////

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub date_time: String,
    pub players: i32,
    pub time_global: i64,
    pub kills_global: i64,
    pub gems_global: i64,
    pub deaths_global: i64,
    pub daggers_hit_global: i64,
    pub daggers_fired_global: i64,
    pub entries: Vec<Entry>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub rank: i32,
    pub id: i32,
    pub username: String,
    pub time: i32,
    pub kills: i32,
    pub gems: i32,
    pub death_type: i16,
    pub daggers_hit: i32,
    pub daggers_fired: i32,
    pub time_total: i64,
    pub kills_total: i64,
    pub gems_total: i64,
    pub deaths_total: i64,
    pub daggers_hit_total: i64,
    pub daggers_fired_total: i64,
}

impl Entry {
    pub fn time_as_f32(&self) -> f32 {
        (self.time as f32) / 10000.
    }

    pub fn time_total_as_f64(&self) -> f64 {
        (self.time_total as f64) / 10000.
    }
}

