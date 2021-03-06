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

use serde_big_array::BigArray;
use zenoh_flow::serde::{Deserialize, Serialize};
use zenoh_flow::zenoh_flow_derive::ZFData;
use zenoh_flow::{Deserializable, ZFData, ZFError, ZFResult};

#[derive(Serialize, Deserialize, PartialEq, Default, ZFData, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ZFData for Vector3 {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Vector3 {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

impl ZFData for Twist {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Twist {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct TwistWithCovariance {
    pub twist: Twist,
    #[serde(with = "BigArray")]
    pub covariance: [f64; 36],
}

impl ZFData for TwistWithCovariance {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for TwistWithCovariance {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ZFData for Point {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Point {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl ZFData for Quaternion {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Quaternion {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

impl ZFData for Pose {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for Pose {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, ZFData)]
pub struct PoseWithCovariance {
    pub pose: Pose,
    #[serde(with = "BigArray")]
    pub covariance: [f64; 36],
}

impl ZFData for PoseWithCovariance {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        bincode::serialize(self).map_err(|_| ZFError::SerializationError)
    }
}

impl Deserializable for PoseWithCovariance {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value = bincode::deserialize::<Self>(bytes).map_err(|_| ZFError::SerializationError)?;
        Ok(value)
    }
}
