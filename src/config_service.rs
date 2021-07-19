use crate::config_cmd::ConfigCmd;
use crate::auth::Auth;
use crate::configuration::Configuration;
use crate::configuration::DEFAULT_PROFILE;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;


pub fn call(form : ConfigCmd) -> Result<()> {
    match form {
        ConfigCmd::Set{auth_profile, auth_token, auth_secret_key} => {
            let profile_key = match auth_profile {
                Some(profile) => profile,
                None => String::from(DEFAULT_PROFILE)
            };
            Configuration::store(profile_key,Auth {token : auth_token, secret_key : auth_secret_key})
        },
        ConfigCmd::ShowFile=>
            Ok(println!("{}", Configuration::get_config_file_path().to_str().expect("Error Reading config file.")))
    }
}