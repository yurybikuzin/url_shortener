use super::*;

use server::{resolve, shorten, stat};
use std::collections::{HashMap, VecDeque};
// use std::{fs, path::Path, sync::Arc};

// use parquet::{
//     file::{
//         properties::WriterProperties,
//         writer::SerializedFileWriter,
//     },
//     schema::parser::parse_message_type,
// };

// let path = Path::new("/path/to/sample.parquet");

// let message_type = "
//   message schema {
//     REQUIRED INT32 b;
//   }
// ";

// ==========================================================

pub type ShortenResult = Result<String>;
pub async fn shorten(payload: shorten::Payload) -> ShortenResult {
    let mut checksum = (crc32fast::hash(payload.url.as_bytes()) % *CHECKSUM_MAX) as usize;
    let chars = &*CHARS.read().unwrap();
    let chars_len = chars.len();
    let mut ret = String::new();
    while checksum > 0 {
        ret.push(chars[checksum % chars_len]);
        checksum /= chars_len;
    }
    // TODO: save shortened url to persistent storage
    SHORTENED.write().unwrap().insert(
        ret.clone(),
        Shortened {
            url: payload.url,
            stat: VecDeque::new(),
        },
    );
    Ok(ret)
}

pub fn shorten_sync(res: ShortenResult, tx: TxHandle) -> Result<()> {
    send_response_message(ResponseMessage::Shorten(res), tx);
    Ok(())
}

// ==========================================================

pub type ResolveResult = Result<Option<String>>;
pub async fn resolve(payload: resolve::Payload) -> ResolveResult {
    // TODO: save resolution to persistent storage
    Ok(
        if let Some(Shortened { url, stat }) =
            SHORTENED.write().unwrap().get_mut(&payload.shortened)
        {
            if let Some(ip) = payload.ip {
                stat.push_front(StatItem {
                    at: chrono::Utc::now(),
                    ip,
                });
            }
            Some(url.clone())
        } else {
            None
        },
    )
}

pub fn resolve_sync(res: ResolveResult, tx: TxHandle) -> Result<()> {
    send_response_message(ResponseMessage::Resolve(res), tx);
    Ok(())
}

// ==========================================================

pub type StatResult = Result<Option<Shortened>>;
pub async fn stat(payload: stat::Payload) -> StatResult {
    // TODO: save resolution to persistent storage
    Ok(SHORTENED
        .write()
        .unwrap()
        .get_mut(&payload.shortened)
        .map(|ret| ret.clone()))
}

pub fn stat_sync(res: StatResult, tx: TxHandle) -> Result<()> {
    send_response_message(ResponseMessage::Stat(res), tx);
    Ok(())
}

// ==========================================================

const CHARS_COUNT: usize = 5;
lazy_static::lazy_static! {
    pub static ref CHECKSUM_MAX: u32 = {
        let mut ret = 1u32;
        let chars_len = CHARS.read().unwrap().len() as u32;
        for _ in 0..CHARS_COUNT {
            ret *= chars_len;
        }
        ret
    };
    pub static ref CHARS: std::sync::RwLock<Vec<char>> = std::sync::RwLock::new(
        // https://stackoverflow.com/questions/7109143/what-characters-are-valid-in-a-url
        ('A'..='Z')
            .chain(
        'a'..='z'
            )
            .chain(
        '0'..='9'
            )
            .chain(
        ['-', '.', '_', '~', ':'].into_iter()
            )
        // SPECIAL MEANING:
        //     .chain(
        // ['/', '?'].into_iter()
        //     )
        // CURL denie unmatched brackets in url  :
        //     .chain(
        // ['[', ']'].into_iter()
        //     )
            .chain(
        ['@', '!', '$'].into_iter()
            )
        // SPECIAL MEANING:
        //     .chain(
        // ['&'].into_iter()
        //     )
            .chain(
        ['\'', '(', ')', '*'].into_iter()
            )
        // SPECIAL MEANING:
        //     .chain(
        // ['+', '#'].into_iter()
        //     )
            .chain(
        [',', ';'].into_iter()
            )
            .chain(
        [',', ';'].into_iter()
            )
        // SPECIAL MEANING:
        //     .chain(
        // ['%', '#'].into_iter()
        //     )
            .chain(
        ['='].into_iter()
            )
        .collect()
    );
    pub static ref SHORTENED: std::sync::RwLock<HashMap<String, Shortened>> = std::sync::RwLock::new(HashMap::new());
}

use chrono::serde::ts_seconds;
pub type Stat = VecDeque<StatItem>;
#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatItem {
    #[serde(with = "ts_seconds")]
    at: chrono::DateTime<chrono::Utc>,
    ip: std::net::IpAddr,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortened {
    url: String,
    stat: Stat,
}
