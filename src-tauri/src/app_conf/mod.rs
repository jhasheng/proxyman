use snafu::ResultExt;
use std::{fs, path::PathBuf};

use crate::error::{self, configuration_error::AppConfIoError, ConfigurationError};

use self::app_setting::{read_app_setting, write_app_setting};

pub(crate) use self::app_setting::AppSetting;

mod app_setting;

pub fn init() -> Result<(), error::Error> {
    ensure_app_dir()
}

pub fn app_logger_file() -> PathBuf {
    get_app_path("proxyman.log")
}

pub fn app_rule_dir() -> PathBuf {
    get_app_path("rule")
}

fn app_setting_file() -> PathBuf {
    get_app_path("settings.json")
}

pub fn get_app_setting() -> AppSetting {
    read_app_setting(app_setting_file()).unwrap_or(AppSetting::default())
}

pub fn save_app_setting(conf: AppSetting) -> () {
    if let Err(err) = write_app_setting(app_setting_file(), conf) {
        log::error!("save app setting error: {}", err);
    }
}

fn get_app_path(name: &str) -> PathBuf {
    let mut path = app_dir();
    path.push(name);
    path
}

pub fn app_dir() -> PathBuf {
    const APP_DOT: &str = ".proxyman";

    let mut app_dir = home::home_dir().unwrap();
    app_dir.push(APP_DOT);

    app_dir.clone()
}

fn ensure_app_dir() -> Result<(), error::Error> {
    let app_dir = app_dir();

    match fs::metadata(&app_dir) {
        Ok(meta) => {
            if !meta.is_dir() {
                fs::create_dir(app_dir)
                    .context(AppConfIoError {})
                    .context(ConfigurationError {
                        scenario: "Ensure app dir",
                    })
            } else {
                Ok(())
            }
        }
        Err(_) => fs::create_dir(app_dir)
            .context(AppConfIoError {})
            .context(ConfigurationError {
                scenario: "Ensure app dir",
            }),
    }
}
