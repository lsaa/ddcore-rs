//
// Submissions to DD Custom Leaderboards
//
        
use crate::{models::{StatsBlockWithFrames, GameMode}, client_https};
use anyhow::bail;
use hyper::{Body, Client, Method, Request};
use futures::StreamExt;
use crate::ddinfo::{time_as_int, get_os};
use super::models::OperatingSystem;

pub struct DdclSecrets {
    pub iv: String,
    pub pass: String,
    pub salt: String
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
    pub game_data: GameState,
    pub status: i32,
    pub replay_data: String,
    pub replay_player_id: i32,
    pub game_mode: GameMode,
    pub time_attack_or_race_finished: bool,
}

#[derive(serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub gems_collected: Vec<i32>,
    pub enemies_killed: Vec<i32>,
    pub daggers_fired: Vec<i32>,
    pub daggers_hit: Vec<i32>,
    pub enemies_alive: Vec<i32>,
    pub homing_daggers: Vec<i32>,
    pub homing_daggers_eaten: Vec<i32>,
    pub gems_despawned: Vec<i32>,
    pub gems_eaten: Vec<i32>,
    pub gems_total: Vec<i32>,
    pub skull1s_alive: Vec<i32>,
    pub skull2s_alive: Vec<i32>,
    pub skull3s_alive: Vec<i32>,
    pub spiderlings_alive: Vec<i32>,
    pub skull4s_alive: Vec<i32>,
    pub squid1s_alive: Vec<i32>,
    pub squid2s_alive: Vec<i32>,
    pub squid3s_alive: Vec<i32>,
    pub centipedes_alive: Vec<i32>,
    pub gigapedes_alive: Vec<i32>,
    pub spider1s_alive: Vec<i32>,
    pub spider2s_alive: Vec<i32>,
    pub leviathans_alive: Vec<i32>,
    pub orbs_alive: Vec<i32>,
    pub thorns_alive: Vec<i32>,
    pub ghostpedes_alive: Vec<i32>,
    pub skull1s_killed: Vec<i32>,
    pub skull2s_killed: Vec<i32>,
    pub skull3s_killed: Vec<i32>,
    pub spiderlings_killed: Vec<i32>,
    pub skull4s_killed: Vec<i32>,
    pub squid1s_killed: Vec<i32>,
    pub squid2s_killed: Vec<i32>,
    pub squid3s_killed: Vec<i32>,
    pub centipedes_killed: Vec<i32>,
    pub gigapedes_killed: Vec<i32>,
    pub spider1s_killed: Vec<i32>,
    pub spider2s_killed: Vec<i32>,
    pub leviathans_killed: Vec<i32>,
    pub orbs_killed: Vec<i32>,
    pub thorns_killed: Vec<i32>,
    pub ghostpedes_killed: Vec<i32>,
    pub spider_eggs_alive: Vec<i32>,
    pub spider_eggs_killed: Vec<i32>,
}

impl SubmitRunRequest {
    pub fn from_compiled_run<T: ToString, K: ToString>(
        run: std::sync::Arc<StatsBlockWithFrames>,
        secrets: Option<DdclSecrets>, 
        client: T, 
        version: K,
        replay_bin: std::sync::Arc<Vec<u8>>
    ) -> anyhow::Result<Self> {
        if secrets.is_none() {
            bail!("Missing DDCL Secrets");
        }

        let game_data = GameState {
            gems_collected: run.frames.iter().map(|f| f.gems_collected).collect(),
            enemies_killed: run.frames.iter().map(|f| f.kills).collect(),
            daggers_fired: run.frames.iter().map(|f| f.daggers_fired).collect(),
            daggers_hit: run.frames.iter().map(|f| f.daggers_hit).collect(),
            enemies_alive: run.frames.iter().map(|f| f.enemies_alive).collect(),
            homing_daggers: run.frames.iter().map(|f| f.homing).collect(),
            homing_daggers_eaten: run.frames.iter().map(|f| f.daggers_eaten).collect(),
            gems_despawned: run.frames.iter().map(|f| f.gems_despawned).collect(),
            gems_eaten: run.frames.iter().map(|f| f.gems_eaten).collect(),
            gems_total: run.frames.iter().map(|f| f.gems_total).collect(),
            skull1s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[0] as i32).collect(),
            skull2s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[1] as i32).collect(),
            skull3s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[2] as i32).collect(),
            spiderlings_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[3] as i32).collect(),
            skull4s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[4] as i32).collect(),
            squid1s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[5] as i32).collect(),
            squid2s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[6] as i32).collect(),
            squid3s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[7] as i32).collect(),
            centipedes_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[8] as i32).collect(),
            gigapedes_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[9] as i32).collect(),
            spider1s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[10] as i32).collect(),
            spider2s_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[11] as i32).collect(),
            leviathans_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[12] as i32).collect(),
            orbs_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[13] as i32).collect(),
            thorns_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[14] as i32).collect(),
            ghostpedes_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[15] as i32).collect(),
            spider_eggs_alive: run.frames.iter().map(|f| f.per_enemy_alive_count[16] as i32).collect(),
            skull1s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[0] as i32).collect(),
            skull2s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[1] as i32).collect(),
            skull3s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[2] as i32).collect(),
            spiderlings_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[3] as i32).collect(),
            skull4s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[4] as i32).collect(),
            squid1s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[5] as i32).collect(),
            squid2s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[6] as i32).collect(),
            squid3s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[7] as i32).collect(),
            centipedes_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[8] as i32).collect(),
            gigapedes_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[9] as i32).collect(),
            spider1s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[10] as i32).collect(),
            spider2s_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[11] as i32).collect(),
            leviathans_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[12] as i32).collect(),
            orbs_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[13] as i32).collect(),
            thorns_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[14] as i32).collect(),
            ghostpedes_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[15] as i32).collect(),
            spider_eggs_killed: run.frames.iter().map(|f| f.per_enemy_kill_count[16] as i32).collect(),
        };

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
            crate::utils::md5_to_string(&run.block.survival_md5[..]),
            vec![
                time_as_int(run.block.time_lvl2).to_string(),
                time_as_int(run.block.time_lvl3).to_string(),
                time_as_int(run.block.time_lvl4).to_string()
            ].join(",")
        ].join(";");

        let validation = crypto_encoder::encrypt_and_encode(to_encrypt, sec.pass, sec.salt, sec.iv)?;
        
        let replay_bin = base64::encode(&replay_bin[..]);

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
            client_version: version.to_string(),
            operating_system: get_os(),
            build_mode: "Release".to_owned(),
            client: client.to_string(),
            validation: validation.replace('=', ""),
            is_replay: run.block.is_replay,
            prohibited_mods: run.block.prohibited_mods,
            game_data,
            status: run.block.status,
            replay_data: replay_bin,
            replay_player_id: run.block.replay_player_id,
            game_mode: run.block.game_mode.into(),
            // TODO: !!!!!!! !!!!!!!!!!! !!!!!!!!!!!!!!!! !!!!!!! 
            // TODO: """"""""""""""""""""""""""""""""""""""""""""
            // TODO: Remove this shit when the linux update drops
            time_attack_or_race_finished: if cfg!(target_os = "linux") { false } else { run.block.is_time_attack_or_race_finished },
        })
    }
}

pub async fn submit<T: ToString, K: ToString>(
    data: std::sync::Arc<StatsBlockWithFrames>,
    secrets: Option<DdclSecrets>, 
    client: T, 
    version: K, 
    replay_bin: std::sync::Arc<Vec<u8>>
) -> anyhow::Result<()> {
    if replay_bin.is_empty() {
        bail!("No bytes in replay!");
    }

    let req = SubmitRunRequest::from_compiled_run(data, secrets, client, version, replay_bin);
    if req.is_ok() {
        let client: Client<_, hyper::Body> = client_https!();
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
        if res.status() != 200 {
            unsafe { bail!(String::from_utf8_unchecked(body)); }
        }
    }
    Ok(())
}

//
//  Crypto Encoding for DDCL
//

pub mod crypto_encoder {
    use std::num::NonZeroU32;
    use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut};
    use ring::pbkdf2;
    use base32::Alphabet::RFC4648;
    use anyhow::Result;
    use aes::cipher::KeyIvInit;

    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

    pub fn encrypt_and_encode(plain: String, password: String, salt: String, iv: String) -> Result<String> {
        let password = &password;
        let mut pbkdf2_hash = [0u8; 16]; // 16 bytes for Aes128
        let n_iter = NonZeroU32::new(65536).unwrap();
        let salt = salt.as_bytes();
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA1,
            n_iter,
            salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        let plain = plain.as_bytes();
        let mut buffer = [0_u8; 1000]; // big buffer
        let cipher = match Aes128CbcEnc::new_from_slices(&pbkdf2_hash, iv.as_bytes()) {
            Ok(v) => Ok(v),
            Err(_) => Err(anyhow::anyhow!("Cipher Error")),
        }?;
        let pos = plain.len();
        buffer[..pos].copy_from_slice(plain);
        let ciphertext = match cipher.encrypt_padded_mut::<Pkcs7>(&mut buffer, pos) {
            Ok(v) => Ok(v),
            Err(_e) => Err(anyhow::anyhow!("Ciphertext Err")),
        }?;
        Ok(base32::encode(RFC4648 { padding: true }, ciphertext))
    }
}

