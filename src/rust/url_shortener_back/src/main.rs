#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

mod pasitos;
mod server;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = built_info::PKG_NAME)]
pub struct Opt {
    /// Workdir where to read .env
    #[structopt(short, long, parse(from_os_str))]
    pub workdir: Option<std::path::PathBuf>,

    /// Test config
    #[structopt(short, long)]
    pub test_config: bool,

    /// No show opts
    #[structopt(short, long)]
    pub no_show_opts: bool,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

lazy_static::lazy_static! {
    pub static ref OPT: std::sync::RwLock<Option<Opt>> = std::sync::RwLock::new(None);
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    if let Some(workdir) = &opt.workdir {
        std::env::set_current_dir(workdir)
            .map_err(|err| anyhow!("failed to set {:?} for current dir: {}", opt.workdir, err))?;
    }
    dotenv::dotenv().context("file .env")?;
    pretty_env_logger::init_timed();
    if !opt.no_show_opts {
        info!(
            "{} {}\ncurrent dir: {:?}\nenv_settings: {:#?}",
            built_info::PKG_NAME,
            built_info::PKG_VERSION,
            std::env::current_dir().unwrap(),
            *(ENV_SETTINGS.read().unwrap())
        );
    }
    load_settings()?;
    if !opt.no_show_opts {
        info!(
            "settings from {:?}:\n{:#?}",
            std::path::PathBuf::from(&common_macros::env_settings!(settings_toml_path)),
            (*SETTINGS.read().unwrap()).as_ref().unwrap().content
        );
        info!("opt: {:#?}", opt);
    }
    if opt.test_config {
        return Ok(());
    }
    *(OPT.write().unwrap()) = Some(opt);
    let opt = (*OPT.read().unwrap()).clone().unwrap();

    #[allow(unreachable_patterns)]
    match opt
        .cmd
        .as_ref()
        .ok_or_else(|| anyhow!("no command specified"))?
    {
        Command::Server {
            port,
            op_mode: op_mode_by_cmd_line,
        } => {
            *OP_MODE.write().unwrap() = op_mode::OpMode::get_actual(op_mode_by_cmd_line);
            EnvSettings::set_port(port);
            let port = EnvSettings::port();

            let _ = futures::join!(
                actix_web::HttpServer::new(|| {
                    actix_web::App::new().configure(server::config_app())
                })
                .bind(("0.0.0.0", port))?
                .run(),
                pasitos::pasos::run(),
            );
        }
        _ => {
            pasitos::pasos::run().await?;
        }
    }
    Ok(())
}

common_macros::declare_env_settings_for_server! {
    settings_toml_path: std::path::PathBuf,
}

common_macros::declare_settings! {
    data: SettingsData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsData {
    pub max_at_once: usize,
}

#[derive(Debug, Clone, StructOpt)]
pub enum Command {
    Server {
        #[structopt(short, long)]
        port: Option<u16>,
        #[structopt(long)]
        op_mode: Option<op_mode::OpMode>,
    },
}
