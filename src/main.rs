use std::env;
use std::path::PathBuf;

mod devices;
pub(crate) mod fancurve;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--list-sensors" {
        println!("Available sensors:");
        for sensor in fancurve::list_available_sensors() {
            println!("  {}", sensor);
        }
        return Ok(());
    }

    let mut configs = devices::Configs { configs: vec![] };

    let config_path = PathBuf::from("/etc/uni-sync/uni-sync.json");

    if !config_path.exists() {
        match std::fs::create_dir_all(config_path.parent().unwrap()) {
            Ok(result) => result,
            Err(_) => {
                println!("Please run uni-sync with elevated permissions.");
                std::process::exit(0);
            }
        };
        match std::fs::write(
            &config_path,
            serde_json::to_string_pretty(&configs).unwrap(),
        ) {
            Ok(result) => result,
            Err(_) => {
                println!("Please run uni-sync with elevated permissions.");
                std::process::exit(0);
            }
        };
    }

    let config_content = std::fs::read_to_string(&config_path).unwrap();
    configs = serde_json::from_str::<devices::Configs>(&config_content).unwrap();

    let new_configs = devices::run(configs);
    let _ = std::fs::write(
        &config_path,
        serde_json::to_string_pretty(&new_configs).unwrap(),
    );

    Ok(())
}
