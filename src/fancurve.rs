use lm_sensors::{self, Initializer};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct FanCurve {
    pub temperatures: Vec<f32>,
    pub speeds: Vec<usize>,
    pub sensor: String, // Chip and feature names, e.g., "coretemp-isa-0000/temp1"
}

pub fn read_fan_curve(file_name: &str) -> Result<FanCurve, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_name)?;
    let fan_curve: FanCurve = serde_json::from_str(&content)?;
    Ok(fan_curve)
}

pub fn calculate_fan_speed(fan_curve: &FanCurve, temperature: f32) -> usize {
    for i in 1..fan_curve.temperatures.len() {
        if temperature <= fan_curve.temperatures[i] {
            let t1 = fan_curve.temperatures[i - 1];
            let t2 = fan_curve.temperatures[i];
            let s1 = fan_curve.speeds[i - 1] as f32;
            let s2 = fan_curve.speeds[i] as f32;

            // Linear interpolation
            return ((s1 + (s2 - s1) * (temperature - t1) / (t2 - t1)) as usize).min(100);
        }
    }
    *fan_curve.speeds.last().unwrap_or(&100)
}

pub fn get_current_temperature(sensor: &str) -> Result<f32, Box<dyn std::error::Error>> {
    // Initialize LM sensors library.
    let sensors = Initializer::default().initialize()?;

    let parts: Vec<&str> = sensor.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid sensor format. Use 'chip/feature'".into());
    }
    let chip_name = parts[0];
    let feature_name = parts[1];

    for chip in sensors.chip_iter(None) {
        if chip.to_string() == chip_name {
            for feature in chip.feature_iter() {
                if let Some(Ok(name)) = feature.name() {
                    if name == feature_name {
                        for sub_feature in feature.sub_feature_iter() {
                            if let Ok(lm_sensors::Value::TemperatureInput(temp)) =
                                sub_feature.value()
                            {
                                return Ok(temp as f32);
                            }
                        }
                    }
                }
            }
        }
    }

    Err(format!("Sensor '{}' not found", sensor).into())
}

pub fn list_available_sensors() -> Vec<String> {
    let mut sensors = Vec::new();

    // Initialize LM sensors library.
    if let Ok(lm_sensors) = Initializer::default().initialize() {
        for chip in lm_sensors.chip_iter(None) {
            let chip_name = chip.to_string();
            for feature in chip.feature_iter() {
                if let Some(Ok(name)) = feature.name() {
                    sensors.push(format!("{}/{}", chip_name, name));
                }
            }
        }
    }
    sensors
}
