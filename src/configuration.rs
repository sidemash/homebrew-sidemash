use std::collections::HashMap;
use std::path::PathBuf;

use confy;
use dirs;
use serde::{Deserialize, Serialize};

use crate::auth::{Auth, AuthParam};
use crate::util;

pub static DEFAULT_PROFILE : &'static str = "default";

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    profile: HashMap<String, Auth>,
}
impl ::std::default::Default for Configuration {
    fn default() -> Self { Self { profile : HashMap::new() } }
}


impl Configuration {

    pub fn load() -> Result<Configuration> {
        let path = Configuration::get_config_file_path();
        match confy::load_path(path) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into())
        }
    }

    pub fn store(profile_key:String, auth_cfg: Auth) -> Result<()> {
        let path = Configuration::get_config_file_path();
        let mut config = Configuration::load()?;
        config.profile.insert(profile_key, auth_cfg);
        match confy::store_path(path, config) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into())
        }
    }


    pub fn auth_from (auth_param: &AuthParam) -> Auth {
        match auth_param {
            AuthParam{auth_token : Some(token), auth_secret_key : Some(secret_key), ..} =>
                Auth{token: token.to_string(), secret_key : secret_key.to_string()},

            AuthParam{auth_profile : Some(profile), ..} => {
                let c = Configuration::load().unwrap();
                match c.profile.get(profile) {
                    Some(auth) => Auth{ token: String::from(&auth.token), secret_key: String::from(&auth.secret_key) },
                    None if c.profile.is_empty() => {
                        eprintln!("Invalid auth profile '{}' name submitted.", profile);
                        std::process::exit(1);
                    },
                    None => {
                        eprintln!("Invalid auth profile '{}' name submitted. Valid values are {}", profile, util::join(c.profile.keys(), "'", "', '", "'"));
                        std::process::exit(1);
                    }
                }
            },
            _ => {
                let c = Configuration::load().unwrap();
                match c.profile.get(DEFAULT_PROFILE) {
                    Some(auth) => Auth{ token: String::from(&auth.token), secret_key: String::from(&auth.secret_key) },
                    None if c.profile.is_empty() => {
                        eprintln!("Missing auth access. Try sdm config -h");
                        std::process::exit(1);
                    },
                    None  => {
                        eprintln!("Missing auth access. Failed to use default profile. Found profiles {}", util::join(c.profile.keys(), "'", "', '", "'"));
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    pub fn get_config_file_path() -> PathBuf {
        match dirs::home_dir() {
            Some(path_buf) => path_buf.join(".sidemash").join("configuration"),
            None => {
                eprintln!("Impossible to get the home directory in your system. ");
                std::process::exit(1);
            }
        }
    }
}
