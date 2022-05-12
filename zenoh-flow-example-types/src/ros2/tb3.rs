// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale zenoh team, <zenoh@zettascale.tech>
//

use zenoh_flow::serde::{Deserialize, Serialize};
use zenoh_flow::zenoh_flow_derive::ZFData;
use zenoh_flow::{Deserializable, ZFData, ZFError, ZFResult};

use super::sensors::{BatteryState, JointState, MagneticField, IMU};
use hls_lfcd_lds_driver::LaserReading;
pub(crate) const LINEAR_SCALING_FACTOR: f64 = 0.20;
pub(crate) const ANGULAR_SCALING_FACTOR: f64 = 2.60;

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum BumperState {
    Forward = 1,
    Backward = 2,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ButtonState {
    Button0 = 1,
    Button1 = 2,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MotorErrors {
    LeftMotor = 1,
    RightMotor = 2,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MotorTorque {
    On = 1,
    Off = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct SensorState {
    pub bumper: u8,
    pub cliff: f32,
    pub sonar: f32,
    pub illumination: f32,
    pub led: u8,
    pub button: u8,
    pub torque: bool,
    pub left_encoder: i32,
    pub right_encoder: i32,
    pub battery: f32,
}

impl ZFData for SensorState {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for SensorState {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Sounds {
    Off = 0,
    On = 1,
    LowBattery = 2,
    Error = 3,
    Button1 = 4,
    Button2 = 5,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct Sound {
    pub value: Sounds,
}

impl ZFData for Sound {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Sound {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Default, Debug, ZFData)]
pub struct VersionInfo {
    pub hardware: String, // <yyyy>.<mm>.<dd>        : hardware version of Turtlebot3 (ex. 2017.05.23)
    pub firmware: String, // <major>.<minor>.<patch> : firmware version of OpenCR
    pub software: String, // <major>.<minor>.<patch> : software version of Turtlebot3 ROS packages
}

impl ZFData for VersionInfo {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for VersionInfo {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct RobotInformation {
    pub battery: BatteryState,
    pub imu: IMU,
    pub magnetic_field: MagneticField,
    pub joint_state: JointState,
    pub sensor_state: SensorState,
}

impl ZFData for RobotInformation {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for RobotInformation {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}



#[derive(Debug, ZFData, Serialize, Deserialize, Clone)]
pub struct LaserScan(pub LaserReading);

impl ZFData for LaserScan {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for LaserScan {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<LaserScan>
    where
        Self: Sized,
    {
        Ok(bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?)
    }
}