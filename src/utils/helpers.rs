use std::env;

use dotenvy::dotenv;

pub fn get_env_variable(varialable: &str) -> Option<String> {
    match env::var(varialable) {
        Ok(env_var) => Some(env_var.trim().to_string()),
        Err(_) => {
            dotenv().ok();
            match env::var(varialable) {
                Ok(var_from_file) => Some(var_from_file.trim().to_string()),
                Err(_) => None,
            }
        }
    }
}
