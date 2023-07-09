use anyhow::{anyhow, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, strum::Display, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum OpMode {
    #[strum(serialize = "prod")]
    Prod = 0,
    #[strum(serialize = "dev")]
    Dev = 1,
    #[strum(serialize = "demo")]
    Demo = 2,
    #[strum(serialize = "rc")]
    Rc = 3,
    #[strum(serialize = "local")]
    Local = 4,
}
common_macros::r#impl!(FromStr for OpMode; strum);

impl Default for OpMode {
    fn default() -> Self {
        Self::Prod
    }
}

impl OpMode {
    pub fn get_actual(op_mode_by_cmd_line: &Option<Self>) -> Self {
        let op_mode_by_current_exe = OpMode::from_current_exe()
            .map_err(|err| {
                warn!("{}:{}: {err}", file!(), line!());
                err
            })
            .unwrap_or_default();

        if let Some(op_mode) = if let Some(op_mode_by_cmd_line) = op_mode_by_cmd_line {
            if op_mode_by_current_exe == *op_mode_by_cmd_line {
                None
            } else {
                info!("op_mode: {op_mode_by_current_exe} is overrided by '--op-mode {op_mode_by_cmd_line}'");
                Some(*op_mode_by_cmd_line)
            }
        } else {
            None
        } {
            op_mode
        } else {
            info!("op_mode: {}", op_mode_by_current_exe);
            op_mode_by_current_exe
        }
    }
    fn from_path_segment(s: &str) -> Self {
        match s {
            "target" => OpMode::Local,
            "rc" => OpMode::Rc,
            "demo" => OpMode::Demo,
            "dev" => OpMode::Dev,
            _ => OpMode::Prod,
        }
    }
    pub fn from_current_exe() -> Result<Self> {
        let current_exe = std::env::current_exe()?;
        current_exe
            .components()
            .rev()
            .nth(2)
            .and_then(|i| {
                if let std::path::Component::Normal(s) = i {
                    Some(s)
                } else {
                    None
                }
            })
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("failed to obtain op_mode from {:?}", current_exe))
            .map(Self::from_path_segment)
    }
}
