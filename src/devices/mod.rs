use serde_derive::{Deserialize, Serialize};
use hidapi::{self, HidDevice};
use std::{thread, time};

#[derive(Serialize, Deserialize, Clone)]
pub struct Configs {
    pub configs: Vec<Config>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub device_id: String,
    pub sync_rgb: bool,
    pub channels: Vec<Channel>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    pub mode: String,
    pub speed: usize,
}

const VENDOR_IDS: [u16; 1] = [ 0x0cf2 ];
const PRODUCT_IDS: [u16; 7] = [ 0x7750, 0xa100, 0xa101, 0xa102, 0xa103, 0xa104, 0xa105 ];

pub fn run(mut existing_configs: Configs) -> Configs {

    let mut default_channels: Vec<Channel> = Vec::new();
    for _x in 0..4 {
        default_channels.push(Channel {
            mode: "Manual".to_string(),
            speed: 50
        });
    }

    // Get All Devices
    let api = match hidapi::HidApi::new() {
        Ok(api) => api,
        Err(_) => panic!("Could not find any controllers")
    };
    
    for hiddevice in api.device_list() {

        if VENDOR_IDS.contains(&hiddevice.vendor_id()) && PRODUCT_IDS.contains(&hiddevice.product_id()) {

            let serial_number: &str = match hiddevice.serial_number() {
                Some(sn) => sn,
                None => {
                    println!("Serial number not available for device {:?}", hiddevice);
                    continue; 
                }
            };
            let device_id: String = format!("VID:{}/PID:{}/SN:{}", hiddevice.vendor_id().to_string(), hiddevice.product_id().to_string(), serial_number.to_string());
            let hid: HidDevice = match api.open(hiddevice.vendor_id(), hiddevice.product_id()) {
                Ok(hid) => hid,
                Err(_) => {
                    println!("Please run uni-sync with elevated permissions.");
                    std::process::exit(0);
                }
            };
            let mut channels: Vec<Channel> = default_channels.clone();
            let mut sync_rgb: bool = false;


            println!("Found: {:?}", device_id);

            if let Some(config) = existing_configs.configs.iter().find( | config | config.device_id == device_id) {
                channels = config.channels.clone();
                sync_rgb = config.sync_rgb;
            } else {
                existing_configs.configs.push(Config {
                    device_id: device_id,
                    sync_rgb: false,
                    channels: channels.clone()
                });
            }

            
            // Send Command to Sync to RGB Header
            let sync_byte: u8 = if sync_rgb { 1 } else { 0 };
            let _ = match &hiddevice.product_id() {
                0xa100|0x7750 => hid.write(&[224, 16, 48, sync_byte, 0, 0, 0]), // SL
                0xa101 => hid.write(&[224, 16, 65, sync_byte, 0, 0, 0]), // AL
                0xa102 => hid.write(&[224, 16, 97, sync_byte, 0, 0, 0]), // SLI
                0xa103|0xa105 => hid.write(&[224, 16, 97, sync_byte, 0, 0, 0]), // SLv2
                0xa104 => hid.write(&[224, 16, 97, sync_byte, 0, 0, 0]), // ALv2
                _ => hid.write(&[224, 16, 48, sync_byte, 0, 0, 0]), // SL
            };

            // Avoid Race Condition
            thread::sleep(time::Duration::from_millis(200));


            for x in 0..channels.len() {
                
                // Disable Sync to fan header
                let mut channel_byte = 0x10 << x;

                if channels[x].mode == "PWM" {
                    channel_byte = channel_byte | 0x1 << x;
                }

                let _ = match &hiddevice.product_id() {
                    0xa100|0x7750 => hid.write(&[224, 16, 49, channel_byte]), // SL
                    0xa101 => hid.write(&[224, 16, 66, channel_byte]), // AL
                    0xa102 => hid.write(&[224, 16, 98, channel_byte]), // SLI
                    0xa103|0xa105 => hid.write(&[224, 16, 98, channel_byte]), // SLv2
                    0xa104 => hid.write(&[224, 16, 98, channel_byte]), // ALv2
                    _ => hid.write(&[224, 16, 49, channel_byte]), // SL
                };

                // Avoid Race Condition
                thread::sleep(time::Duration::from_millis(200));

                // Set Channel Speed
                if channels[x].mode == "Manual" {

                    let mut speed = channels[x].speed as f64;
                    if speed > 100.0 { speed = 100.0 }

                    let speed_800_1900: u8 = ((800.0 + (11.0 * speed)) as usize / 19).try_into().unwrap();
                    let speed_250_2000: u8 = ((250.0 + (17.5 * speed)) as usize / 20).try_into().unwrap();
                    let speed_200_2100: u8 = ((200.0 + (19.0 * speed)) as usize  / 21).try_into().unwrap();

                    let _ = match &hiddevice.product_id() {
                        0xa100|0x7750 => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_800_1900]), // SL
                        0xa101 => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_800_1900]), // AL
                        0xa102 => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_200_2100]), // SLI
                        0xa103|0xa105 => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_250_2000]), // SLv2
                        0xa104 => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_250_2000]), // ALv2
                        _ => hid.write(&[224, (x+32).try_into().unwrap(), 0, speed_800_1900]), // SL
                    };

                    // Avoid Race Condition
                    thread::sleep(time::Duration::from_millis(100));

                }
            }

        }
        
    }

    return existing_configs;

}
