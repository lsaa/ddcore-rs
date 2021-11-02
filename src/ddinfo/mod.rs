//
// Client Module for devildaggers.info api
//

pub mod models;

#[cfg(feature = "ddcl_submit")]
pub mod ddcl_submit;

use anyhow::{Result, bail};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use futures::StreamExt;
use crate::ddinfo::models::{DdstatsRustIntegration, Entry, Leaderboard, SpawnsetFile, SpawnsetForDdcl};

use self::models::{OperatingSystem, MarkerResponse, Tool};

#[cfg(target_os = "windows")]
pub fn get_os() -> OperatingSystem {
    OperatingSystem::Windows
}

#[cfg(target_os = "linux")]
pub fn get_os() -> OperatingSystem {
    OperatingSystem::Linux
}

pub fn time_as_int(t: f32) -> i32 {
    (t * 10000.) as i32
}

////////////////////////////////// Process Memory
//////////////////////////////////

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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: MarkerResponse = serde_json::from_slice(&body)?;
    Ok(res)
}

////////////////////////////////// Tool
//////////////////////////////////

pub async fn get_tool<T: ToString>(tool_name: T) -> Result<Tool> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/tools/{}", tool_name.to_string());
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Tool = serde_json::from_slice(&body)?;
    Ok(res)
}

////////////////////////////////// Integrations
//////////////////////////////////

pub async fn get_integration_ddstats_rust() -> Result<DdstatsRustIntegration> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/integrations/ddstats-rust");
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: DdstatsRustIntegration = serde_json::from_slice(&body)?;
    Ok(res)
}

////////////////////////////////// Leaderboards
//////////////////////////////////

pub async fn get_leaderboard(rank_start: i32) -> Result<Leaderboard> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/leaderboards?rankStart={}", rank_start);
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Leaderboard = serde_json::from_slice(&body)?;
    Ok(res)
}

pub async fn get_leaderboard_user_by_id(uid: i32) -> Result<Entry> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/leaderboards/user/by-id?userId={}", uid);
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Entry = serde_json::from_slice(&body)?;
    Ok(res)
}

pub async fn get_leaderboard_user_by_username<T: ToString>(username: T) -> Result<Vec<Entry>>{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/leaderboards/user/by-username?username={}", username.to_string());
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Vec<Entry> = serde_json::from_slice(&body)?;
    Ok(res)
}

pub async fn get_leaderboard_user_by_rank(rank: i32) -> Result<Entry> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/leaderboards/user/by-rank?rank={}", rank);
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Entry = serde_json::from_slice(&body)?;
    Ok(res)
}

////////////////////////////////// Spawnsets
//////////////////////////////////

pub async fn get_all_spawnsets<T: ToString, K: ToString>(name_filter: T, author_filter: K) -> Result<Vec<SpawnsetFile>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/spawnsets/ddse?authorFilter={}&nameFilter={}", author_filter.to_string(), name_filter.to_string());
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: Vec<SpawnsetFile> = serde_json::from_slice(&body)?;
    Ok(res)
}

pub async fn get_spawnset_by_hash<T: ToString>(hash: T) -> Result<SpawnsetForDdcl> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let vv = crate::utils::decode_hex(&hash.to_string())?;
    let b = base64::encode(vv)
        .replace("=", "%3D")
        .replace("/", "%2F")
        .replace("+", "%2B");
    let path = format!("api/spawnsets/by-hash?hash={}", b);
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
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    let res: SpawnsetForDdcl = serde_json::from_slice(&body)?;
    Ok(res)
}

////////////////////////////////// Custom Leaderboards
//////////////////////////////////

pub async fn get_replay_by_id(entry_id: i32) -> Result<Vec<u8>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("api/custom-entries/{}/replay", entry_id);
    let uri = format!("https://devildaggers.info/{}", path);
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let mut res = client.request(req).await?;
    let mut body = Vec::new();
    while let Some(chunk) = res.body_mut().next().await {
        body.extend_from_slice(&chunk?);
    }
    if res.status() != 200 {
        unsafe { bail!(String::from_utf8_unchecked(body)); }
    }
    Ok(body)
}
