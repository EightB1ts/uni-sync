use std::env;
use std::path::PathBuf;

mod devices;

fn main() -> Result<(), std::io::Error> {
    let mut configs = devices::Configs {
        configs: vec![]
    };

    let args: Vec<String> = env::args().collect();
    let mut config_path: PathBuf = env::current_exe()?;

    if args.len() == 2 {
        config_path.clear();
        config_path.push(&args[1])
    } else {
        config_path.pop();
        config_path.push("uni-sync.json");
    }

    if !config_path.exists() {
        println!("Config path {:?} does not exist. Generating default configuration.", config_path);
        std::fs::write(&config_path, serde_json::to_string_pretty(&configs).unwrap())?;
    } else {
        println!("Loading configuration {:?}", config_path)
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    configs = serde_json::from_str::<devices::Configs>(&config_content)?;

    let new_configs = devices::run(configs);
    std::fs::write(&config_path, serde_json::to_string_pretty(&new_configs)?)?;

    Ok(())
}
