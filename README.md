# Uni-Sync
### A Synchronization Tool for Lian Li Uni Controllers

This tool allows you to configure the synchronization settings for SLV1 and SLV2 Uni Fan controllers on any OS. No more booting into Windows to enable PWM or LED synchronization. 
- Enable/Disable Motherboard PWM Sync
- Enable/Disable LED ARGB Header Sync
- Manually Set Fan Speeds

## Installation

Uni-Sync requires [Rust](https://www.rust-lang.org/learn/get-started) to build. Please install before proceeding.

### 1. Configure Settings

Uni-Sync is configured by modifying a few lines of code within the ```src/main.rs``` file. 

#### a. VENDOR_ID & PRODUCT_ID

**- LINUX -**

These two fields represent the ID of the controller. This will likely be different for the SLV2 controller and will need to be changed. You can locate the ID for your controller by utilizing the ```lsusb``` command in Linux. Example output:
```
0cf2:a100 ENE Technology, Inc. LianLi-UNI FAN-SL-v1.8
```
In this example, **0cf2** is the VENDOR_ID and **a100** is the PRODUCT_ID.

**- WINDOWS -**

Alternatively, you can get this information by locating the controller within Device Manager.

#### b. SYNC_LEDS

A boolean field used to switch on/off RGB synchronization based on the ARGB header on your motherboard.

Default: **true**

Supported Values: **true**, **false**

#### c. FAN_SPEED

An enum field used to configure the Fan Speed configuration for each channel on the controller. The following options are:
- **PWM**: Will set channel to listen to PWM signal from motherboard.
- **Quiet**: Will set channel to around 800 RPM.
- **HighSpeed**: Will set channel to around 1500 RPM.
- **FullSpeed**: Will set channel to around 1900 RPM.

### 2. Run Install Script

#### a. Windows

Run install.bat. This will build the application and move it into your startup folder located under ```%USERPROFILE%\Start Menu\Programs\Startup```

#### b. Linux

Run install.sh. Please note, this will set up a service named uni-sync and will execute once on system startup.
