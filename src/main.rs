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

    let config_path = if args.len() > 2 && args[1] == "--config" {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("/etc/uni-sync/uni-sync.json")
    };

    let configs = if config_path.exists() {
        let config_content = std::fs::read_to_string(&config_path)?;
        serde_json::from_str::<devices::Configs>(&config_content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
    } else {
        devices::Configs { configs: vec![] }
    };

    let new_configs = devices::run(configs);
    
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    std::fs::write(
        &config_path,
        serde_json::to_string_pretty(&new_configs).unwrap(),
    )?;

    Ok(())
}
