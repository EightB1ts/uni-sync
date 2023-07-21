use std::env;

mod devices;

fn main() -> Result<(), std::io::Error> {
 
    let mut configs = devices::Configs {
        configs: vec![]
    };

    let mut config_path = env::current_exe()?;
    config_path.pop();
    config_path.push("uni-sync.json");

    if !config_path.exists() {
        std::fs::write(&config_path, serde_json::to_string_pretty(&configs).unwrap())?;
    }

    let config_content = std::fs::read_to_string(&config_path).unwrap();
    configs = serde_json::from_str::<devices::Configs>(&config_content).unwrap();

    let new_configs = devices::run(configs);
    std::fs::write(&config_path, serde_json::to_string_pretty(&new_configs).unwrap())?;

    Ok(())
}