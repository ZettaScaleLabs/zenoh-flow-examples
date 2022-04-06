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

use super::geometry::{Quaternion, Vector3};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum PowerSupplyStatus {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum PowerSupplyHealth {
    Unknown,
    Good,
    Overheat,
    Dead,
    OverVoltage,
    UnspecFailure,
    Cold,
    WatchdogTimerExpired,
    SafetyTimerExpired,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum PowerSupplyTechnology {
    Unknown,
    NiHM,
    LiON,
    LiPO,
    LiFe,
    NiCD,
    LiMN,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct BatteryState {
    pub voltage: f32,
    pub temperature: f32,
    pub current: f32,
    pub charge: f32,
    pub capacity: f32,
    pub design_capacity: f32,
    pub percentage: f32,
    pub power_supply_status: PowerSupplyStatus,
    pub power_supply_health: PowerSupplyHealth,
    pub power_supply_technology: PowerSupplyTechnology,
    pub present: bool,
    pub cell_voltage: Vec<f32>,
    pub cell_temperature: Vec<f32>,
    pub location: String,
    pub serial_number: String,
}

impl ZFData for BatteryState {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for BatteryState {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct MagneticField {
    pub magnetic_field: Vector3,
    pub magnetic_filed_covariance: [f64; 9],
}

impl ZFData for MagneticField {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for MagneticField {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct IMU {
    pub orientation: Quaternion,
    pub orientation_covariance: [f64; 9],
    pub angular_velocity: Vector3,
    pub angualar_velocity_covariance: [f64; 9],
    pub linear_acceleration: Vector3,
    pub linear_acceleration_covariance: [f64; 9],
}

impl ZFData for IMU {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for IMU {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct JointState {
    pub name: Vec<String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub effort: Vec<f64>,
}

impl ZFData for JointState {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for JointState {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}
