mod controller;
use rusb;
use controller::{ Controller, Channel, FANSPEEDS};

// USB ID
static VENDOR_ID: u16 = 0x0cf2;
static PRODUCT_ID: u16 = 0xa100;

// Controller Configs
static SYNC_LEDS: bool = true;
static CHANNELS: [controller::Channel; 4] = [
    // Channel 1
    Channel {
        fan_speed: FANSPEEDS::FullSpeed
    },
    // Channel 2
    Channel {
        fan_speed: FANSPEEDS::FullSpeed
    },
    // Channel 3
    Channel {
        fan_speed: FANSPEEDS::PWM
    },
    // Channel 4
    Channel {
        fan_speed: FANSPEEDS::PWM
    }
];

fn main() {

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {

            match device.open() {
                Ok(handle) => {

                    let mut controldev = Controller {
                        device: device,
                        device_desc: device_desc,
                        handle: handle,
                        sync_lights: SYNC_LEDS
                    };

                    controller::sync(&mut controldev, CHANNELS).ok();
                },
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    /**/
}
