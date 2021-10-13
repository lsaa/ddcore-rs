//
// Client Module for devildaggers.info api
//

use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use futures::StreamExt;

use crate::models::StatsBlockWithFrames;

pub struct DdclSecrets {
    pub iv: String,
    pub pass: String,
    pub salt: String
}

#[derive(Debug, serde::Serialize)]
pub enum OperatingSystem {
    Windows,
    Linux,
}

#[derive(serde::Deserialize, Debug)]
pub struct MarkerResponse {
    pub value: usize,
}

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

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmitRunRequest {
    pub survival_hash_md5: String,
    pub player_id: i32,
    pub player_name: String,
    pub time: i32,
    pub gems_collected: i32,
    pub enemies_killed: i32,
    pub daggers_fired: i32,
    pub daggers_hit: i32,
    pub enemies_alive: i32,
    pub homing_daggers: i32,
    pub homing_daggers_eaten: i32,
    pub gems_despawned: i32,
    pub gems_eaten: i32,
    pub gems_total: i32,
    pub death_type: u8,
    pub level_up_time2: i32,
    pub level_up_time3: i32,
    pub level_up_time4: i32,
    pub client_version: String,
    pub operating_system: OperatingSystem,
    pub build_mode: String,
    pub client: String,
    pub validation: String,
    pub is_replay: bool,
    pub prohibited_mods: bool,
    pub game_states: Vec<GameState>,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub gems_collected: i32,
    pub enemies_killed: i32,
    pub daggers_fired: i32,
    pub daggers_hit: i32,
    pub enemies_alive: i32,
    pub homing_daggers: i32,
    pub homing_daggers_eaten: i32,
    pub gems_despawned: i32,
    pub gems_eaten: i32,
    pub gems_total: i32,
    pub skull1s_alive: i32,
    pub skull2s_alive: i32,
    pub skull3s_alive: i32,
    pub spiderlings_alive: i32,
    pub skull4s_alive: i32,
    pub squid1s_alive: i32,
    pub squid2s_alive: i32,
    pub squid3s_alive: i32,
    pub centipedes_alive: i32,
    pub gigapedes_alive: i32,
    pub spider1s_alive: i32,
    pub spider2s_alive: i32,
    pub leviathans_alive: i32,
    pub orbs_alive: i32,
    pub thorns_alive: i32,
    pub ghostpedes_alive: i32,
    pub skull1s_killed: i32,
    pub skull2s_killed: i32,
    pub skull3s_killed: i32,
    pub spiderlings_killed: i32,
    pub skull4s_killed: i32,
    pub squid1s_killed: i32,
    pub squid2s_killed: i32,
    pub squid3s_killed: i32,
    pub centipedes_killed: i32,
    pub gigapedes_killed: i32,
    pub spider1s_killed: i32,
    pub spider2s_killed: i32,
    pub leviathans_killed: i32,
    pub orbs_killed: i32,
    pub thorns_killed: i32,
    pub ghostpedes_killed: i32,
}

impl SubmitRunRequest {
    #[cfg(feature = "ddcl_submit")]
    pub fn from_compiled_run(run: &StatsBlockWithFrames, secrets: Option<DdclSecrets>, client: String, version: String) -> anyhow::Result<Self> {
        use anyhow::bail;

        if secrets.is_none() {
            bail!("Missing DDCL Secrets");
        }

        let states: Vec<GameState> = run.frames.iter().map(|frame| {
            GameState {
                gems_collected: frame.gems_collected,
                enemies_killed: frame.kills,
                daggers_fired: frame.daggers_fired,
                daggers_hit: frame.daggers_hit,
                enemies_alive: frame.enemies_alive,
                homing_daggers: frame.homing,
                homing_daggers_eaten: frame.daggers_eaten,
                gems_despawned: frame.gems_despawned,
                gems_eaten: frame.gems_eaten,
                gems_total: frame.gems_total,
                skull1s_alive: frame.per_enemy_alive_count[0] as i32,
                skull2s_alive: frame.per_enemy_alive_count[1] as i32,
                skull3s_alive: frame.per_enemy_alive_count[2] as i32,
                spiderlings_alive: frame.per_enemy_alive_count[3] as i32,
                skull4s_alive: frame.per_enemy_alive_count[4] as i32,
                squid1s_alive: frame.per_enemy_alive_count[5] as i32,
                squid2s_alive: frame.per_enemy_alive_count[6] as i32,
                squid3s_alive: frame.per_enemy_alive_count[7] as i32,
                centipedes_alive: frame.per_enemy_alive_count[8] as i32,
                gigapedes_alive: frame.per_enemy_alive_count[9] as i32,
                spider1s_alive: frame.per_enemy_alive_count[10] as i32,
                spider2s_alive: frame.per_enemy_alive_count[11] as i32,
                leviathans_alive: frame.per_enemy_alive_count[12] as i32,
                orbs_alive: frame.per_enemy_alive_count[13] as i32,
                thorns_alive: frame.per_enemy_alive_count[14] as i32,
                ghostpedes_alive: frame.per_enemy_alive_count[15] as i32,
                skull1s_killed: frame.per_enemy_kill_count[0] as i32,
                skull2s_killed: frame.per_enemy_kill_count[1] as i32,
                skull3s_killed: frame.per_enemy_kill_count[2] as i32,
                spiderlings_killed: frame.per_enemy_kill_count[3] as i32,
                skull4s_killed: frame.per_enemy_kill_count[4] as i32,
                squid1s_killed: frame.per_enemy_kill_count[5] as i32,
                squid2s_killed: frame.per_enemy_kill_count[6] as i32,
                squid3s_killed: frame.per_enemy_kill_count[7] as i32,
                centipedes_killed: frame.per_enemy_kill_count[8] as i32,
                gigapedes_killed: frame.per_enemy_kill_count[9] as i32,
                spider1s_killed: frame.per_enemy_kill_count[10] as i32,
                spider2s_killed: frame.per_enemy_kill_count[11] as i32,
                leviathans_killed: frame.per_enemy_kill_count[12] as i32,
                orbs_killed: frame.per_enemy_kill_count[13] as i32,
                thorns_killed: frame.per_enemy_kill_count[14] as i32,
                ghostpedes_killed: frame.per_enemy_kill_count[15] as i32,
            }
        }).collect();
        let sec = secrets.unwrap();
        let last = run.frames.last().unwrap();

        let to_encrypt = vec![
            run.block.player_id.to_string(),
            time_as_int(run.block.time).to_string(),
            last.gems_collected.to_string(),
            last.gems_despawned.to_string(),
            last.gems_eaten.to_string(),
            last.gems_total.to_string(),
            last.kills.to_string(),
            run.block.death_type.to_string(),
            last.daggers_hit.to_string(),
            last.daggers_fired.to_string(),
            last.enemies_alive.to_string(),
            last.homing.to_string(),
            last.daggers_eaten.to_string(),
            if run.block.is_replay { "1".to_owned() } else { "0".to_owned() },
            md5_byte_string(&run.block.survival_md5),
            vec![
                time_as_int(run.block.time_lvl2).to_string(),
                time_as_int(run.block.time_lvl3).to_string(),
                time_as_int(run.block.time_lvl4).to_string()
            ].join(",")
        ].join(";");

        let validation = crypto_encoder::encrypt_and_encode(to_encrypt, sec.pass, sec.salt, sec.iv)?;
        Ok(Self {
            survival_hash_md5: base64::encode(&run.block.survival_md5),
            player_id: run.block.player_id,
            player_name: run.block.player_username(),
            time: time_as_int(run.block.time),
            gems_collected: last.gems_collected,
            enemies_killed: last.kills,
            daggers_fired: last.daggers_fired,
            daggers_hit: last.daggers_hit,
            enemies_alive: last.enemies_alive,
            homing_daggers: last.homing,
            homing_daggers_eaten: last.daggers_eaten,
            gems_despawned: last.gems_despawned,
            gems_eaten: last.gems_eaten,
            gems_total: last.gems_total,
            death_type: run.block.death_type,
            level_up_time2: time_as_int(run.block.time_lvl2),
            level_up_time3: time_as_int(run.block.time_lvl3),
            level_up_time4: time_as_int(run.block.time_lvl4),
            client_version: version,
            operating_system: get_os(),
            build_mode: "Release".to_owned(),
            client,
            validation: validation.replace("=", ""),
            is_replay: run.block.is_replay,
            prohibited_mods: run.block.prohibited_mods,
            game_states: states
        })
    }
}

#[cfg(target_os = "windows")]
fn get_os() -> OperatingSystem {
    OperatingSystem::Windows
}

#[cfg(target_os = "linux")]
fn get_os() -> OperatingSystem {
    OperatingSystem::Linux
}

fn time_as_int(t: f32) -> i32 {
    (t * 10000.) as i32
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn get_ddstats_memory_marker(os: OperatingSystem) -> Result<MarkerResponse> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/process-memory/marker?operatingSystem={:?}", os);
    let uri = format!("https://devildaggers.info/{}", path);
    let req = Request::builder()
        .header("accept", "application/json")
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let mut res = client.request(req).await?;
    let mut body = Vec::new();
    while let Some(chunk) = res.body_mut().next().await {
        body.extend_from_slice(&chunk?);
    }
    let res: MarkerResponse = serde_json::from_slice(&body)?;
    Ok(res)
}

pub async fn get_tool(tool_name: String) -> Result<Tool> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/tools/{}", tool_name);
    let uri = format!("https://devildaggers.info/{}", path);
    let req = Request::builder()
        .header("accept", "application/json")
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let mut res = client.request(req).await?;
    let mut body = Vec::new();
    while let Some(chunk) = res.body_mut().next().await {
        body.extend_from_slice(&chunk?);
    }
    let res: Tool = serde_json::from_slice(&body)?;
    Ok(res)
}

fn md5_byte_string(b: &[u8]) -> String {
    let mut res = String::new();
    for byte in b {
        res.push_str(&format!("{:02X}", byte));
    }
    res
}

#[cfg(feature = "ddcl_submit")]
pub async fn submit_to_ddcl(data: &StatsBlockWithFrames, secrets: Option<DdclSecrets>, client: String, version: String) -> Result<()> {
    let req = SubmitRunRequest::from_compiled_run(data, secrets, client, version);
    if req.is_ok() {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        let path = "api/custom-entries/submit";
        let uri = format!("https://devildaggers.info/{}", path);
        let req = Request::builder()
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(serde_json::to_string(&req.unwrap())?))
            .unwrap();
        let mut res = client.request(req).await?;
        let mut body = Vec::new();
        while let Some(chunk) = res.body_mut().next().await {
            body.extend_from_slice(&chunk?);
        }
    }
    Ok(())
}

//
//  Crypto Encoding for DDCL
//

#[cfg(feature = "ddcl_submit")]
pub mod crypto_encoder {
    use std::num::NonZeroU32;
    use aes::Aes128;
    use block_modes::{BlockMode, Cbc};
    use block_modes::block_padding::Pkcs7;
    use ring::pbkdf2;
    use base32::Alphabet::RFC4648;
    use anyhow::Result;

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    pub fn encrypt_and_encode(plain: String, password: String, salt: String, iv: String) -> Result<String> {
        let password = &password;
        let mut pbkdf2_hash = [0u8; 16]; // 16 bytes for Aes128
        let n_iter = NonZeroU32::new(65536).unwrap();
        let salt = salt.as_bytes();
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA1,
            n_iter,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        let plain = plain.as_bytes();
        let cipher = Aes128Cbc::new_from_slices(&pbkdf2_hash, &iv.as_bytes())?;
        let mut buffer = [0_u8; 1000]; // big buffer
        let pos = plain.len();
        buffer[..pos].copy_from_slice(plain);
        let ciphertext = cipher.encrypt(&mut buffer, pos)?;
        Ok(base32::encode(RFC4648 { padding: true }, ciphertext))
    }
}

