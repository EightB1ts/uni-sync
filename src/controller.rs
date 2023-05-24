use std::time::Duration;
use crate::rusb;

pub struct Controller {
    pub device: rusb::Device<rusb::GlobalContext>,
    pub device_desc: rusb::DeviceDescriptor,
    pub handle: rusb::DeviceHandle<rusb::GlobalContext>,
    pub sync_lights: bool
}

#[derive(Clone, Copy)]
pub struct Channel {
    pub fan_count: FANCOUNTS,
    pub fan_speed: FANSPEEDS,
}

static UNIHUB_ACTION_ADDRESS: u16 = 0xe021;
static UNIHUB_COMMIT_ADDRESS: u16 = 0xe02f;

enum C1 {
    HubActionAddress            = 0xe8a0, /* Channel 1 fan action address for hub control */
    HubCommitPWMActionAddress   = 0xe890, /* Channel 1 fan commit address for hub control AND PWM Action */
    PWMCommitAddress            = 0xe818, /* Channel 1 fan commit address for pwm control */
}

enum C2 {
    HubActionAddress            = 0xe8a2, /* Channel 2 fan action address for hub control */
    HubCommitPWMActionAddress   = 0xe891, /* Channel 2 fan commit address for hub control AND PWM Action */
    PWMCommitAddress            = 0xe81a, /* Channel 2 fan commit address for pwm control */
}

enum C3 {
    HubActionAddress            = 0xe8a4, /* Channel 3 fan action address for hub control */
    HubCommitPWMActionAddress   = 0xe892, /* Channel 3 fan commit address for hub control AND PWM Action */
    PWMCommitAddress            = 0xe81c, /* Channel 3 fan commit address for pwm control */
}

enum C4 {
    HubActionAddress            = 0xe8a6, /* Channel 4 fan action address for hub control */
    HubCommitPWMActionAddress   = 0xe893, /* Channel 4 fan commit address for hub control AND PWM Action */
    PWMCommitAddress            = 0xe81e, /* Channel 4 fan commit address for pwm control */
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum FANCOUNTS {
    Count000 = 0xFF,
    Count001 = 0x00,
    Count002 = 0x01,
    Count003 = 0x02,
    Count004 = 0x03,
}

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
    }


    let mut control: u8 = 0;
    for i in 0..channels.len() {

        send_config(controller, UNIHUB_ACTION_ADDRESS, &[0x32, channels[i].fan_count as u8])?;
        send_commit(controller, UNIHUB_COMMIT_ADDRESS)?;

        if channels[i].fan_speed == FANSPEEDS::PWM {
            
            let (action_addr, commit_addr);
            match i {
                0 => (action_addr, commit_addr) = (C1::HubCommitPWMActionAddress as u16, C1::PWMCommitAddress as u16),
                1 => (action_addr, commit_addr) = (C2::HubCommitPWMActionAddress as u16, C2::PWMCommitAddress as u16),
                2 => (action_addr, commit_addr) = (C3::HubCommitPWMActionAddress as u16, C3::PWMCommitAddress as u16),
                3 => (action_addr, commit_addr) = (C4::HubCommitPWMActionAddress as u16, C4::PWMCommitAddress as u16),
                _ => (action_addr, commit_addr) = (C1::HubCommitPWMActionAddress as u16, C1::PWMCommitAddress as u16),
            }

            send_config(controller, action_addr, &[0x0])?;
            send_commit(controller, commit_addr)?;
            control |= 0x01 << i;

        } else {

            let (action_addr, commit_addr);
            match i {
                0 => (action_addr, commit_addr) = (C1::HubActionAddress as u16, C1::HubCommitPWMActionAddress as u16),
                1 => (action_addr, commit_addr) = (C2::HubActionAddress as u16, C2::HubCommitPWMActionAddress as u16),
                2 => (action_addr, commit_addr) = (C3::HubActionAddress as u16, C3::HubCommitPWMActionAddress as u16),
                3 => (action_addr, commit_addr) = (C4::HubActionAddress as u16, C4::HubCommitPWMActionAddress as u16),
                _ => (action_addr, commit_addr) = (C1::HubActionAddress as u16, C1::HubCommitPWMActionAddress as u16),
            }

            let config: [u8; 2] = [
                (channels[i].fan_speed as u16 >> 0x08) as u8,
                (channels[i].fan_speed as u16 & 0xff) as u8
            ];

            send_config(controller, action_addr, &config)?;
            send_commit(controller, commit_addr)?;
            
        }

    }

    send_config(controller, UNIHUB_ACTION_ADDRESS, &[0x31, (0xf0 | control)])?;
    send_commit(controller, UNIHUB_COMMIT_ADDRESS)?;

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