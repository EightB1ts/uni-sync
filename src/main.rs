use std::env;
use std::path::PathBuf;

mod devices;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configs = devices::Configs { configs: vec![] };

    let config_path = get_config_path()?;

    if !config_path.exists() {
        std::fs::write(&config_path, serde_json::to_string_pretty(&configs)?)?;
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    configs = serde_json::from_str::<devices::Configs>(&config_content)?;

    let new_configs = devices::run(configs);
    std::fs::write(&config_path, serde_json::to_string_pretty(&new_configs)?)?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // TODO add support for other OS and use their standard system config directory
    let config_dir = match env::consts::OS {
        "windows" => {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(appdata)
        }
        _ => PathBuf::from("/etc/uni-sync"),
    };

    let config_path = config_dir.join("uni-sync.json");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_path)
}
