use std::{thread, time::Duration};
use crate::rusb;

pub struct Controller {
    pub device: rusb::Device<rusb::GlobalContext>,
    pub device_desc: rusb::DeviceDescriptor,
    pub handle: rusb::DeviceHandle<rusb::GlobalContext>,
    pub sync_lights: bool
}

#[derive(Clone, Copy)]
pub struct Channel {
    pub fan_speed: FANSPEEDS,
}

static UNIHUB_ACTION_ADDRESS: u16 = 0xe021;
static UNIHUB_COMMIT_ADDRESS: u16 = 0xe02f;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum FANSPEEDS {
    Quiet     = 0x2003, /* Rather slow */
    HighSpeed = 0x2206, /* Rather fast */
    FullSpeed = 0x6c07, /* Super Duper fast */
    PWM       = 0xffff, /* PWM Control */
}


pub fn sync(controller: &mut Controller, channels: [Channel; 4])-> Result<(), u32> {

    // Initialize Controller
    send_config(controller, UNIHUB_ACTION_ADDRESS, &[0x34])?;
    send_commit(controller, UNIHUB_COMMIT_ADDRESS)?;

    if controller.sync_lights {

        // Send Command to Sync Lights with ARGB Header on MoBo
        send_config(controller, UNIHUB_ACTION_ADDRESS, &[0x30, 0x01])?;
        send_commit(controller, UNIHUB_COMMIT_ADDRESS)?;
    } else {
        
        // Disable ARGB Sync
        send_config(controller, UNIHUB_ACTION_ADDRESS, &[0x30, 0x00])?;
        send_commit(controller, UNIHUB_COMMIT_ADDRESS)?;
    }
        
    // Sleep for 1.5 seconds to avoid race condition
    thread::sleep(Duration::from_millis(1500));

    for i in 0..channels.len() {

        if channels[i].fan_speed == FANSPEEDS::PWM {
            
            // Enable PWM Mode
            let enable_pwm_offset: u8;
            match i {
                0 => enable_pwm_offset = 0x1f,
                1 => enable_pwm_offset = 0x2f,
                2 => enable_pwm_offset = 0x4f,
                3 => enable_pwm_offset = 0x8f,
                _ => enable_pwm_offset = 0x1f,
            }
            send_config(controller, 0xe020, &[0x0, 0x31, enable_pwm_offset, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01])?;
            
        } else {

            // Make Sure PWM Mode is disabled
            let disable_pwm_offset: u8;
            match i {
                0 => disable_pwm_offset = 0x10,
                1 => disable_pwm_offset = 0x20,
                2 => disable_pwm_offset = 0x40,
                3 => disable_pwm_offset = 0x80,
                _ => disable_pwm_offset = 0x10,
            }
            send_config(controller, 0xe020, &[0x0, 0x31, disable_pwm_offset, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01])?;

            // Determine Address for Channel and set fan speed
            let channel_addr: u16;
            let commit_addr: u16;
            match i {
                0 => (channel_addr, commit_addr) = (0xd8a0, 0xd890),
                1 => (channel_addr, commit_addr) = (0xd8a2, 0xd891),
                2 => (channel_addr, commit_addr) = (0xd8a4, 0xd892),
                3 => (channel_addr, commit_addr) = (0xd8a6, 0xd893),
                _ => (channel_addr, commit_addr) = (0xd8a0, 0xd890),
            }

            let config: [u8; 2] = [
                (channels[i].fan_speed as u16 >> 0x08) as u8,
                (channels[i].fan_speed as u16 & 0xff) as u8
            ];

            send_config(controller, channel_addr, &config)?;
            send_commit(controller, commit_addr)?;
        }

    }

    Ok(())
}

fn send_config(controller: &mut Controller, index: u16, buf: &[u8]) -> Result<(), u32> {

    match controller.handle.reset() {
        Ok(_) => {},
        Err(_) => return Err(0)
    }

    let timeout = Duration::from_secs(10);

    match controller.handle.write_control(0x40, 0x80, 0x0, index, buf, timeout) {
        Ok(_) => Ok(()),
        Err(_) => return Err(1)
    }
}

fn send_commit(controller: &mut Controller, index: u16) -> Result<(), u32> {
    send_config(controller, index, &[0x01])?;
    Ok(())
}
