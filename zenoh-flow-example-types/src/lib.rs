//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use zenoh_flow::prelude::*;
use zenoh_flow::zenoh_flow_derive::ZFData;
// We may want to provide some "built-in" types
pub mod ros2;

#[derive(Debug, Clone, ZFData)]
pub struct ZFString(pub String);

impl ZFData for ZFString {
    fn try_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.0.as_bytes().to_vec())
    }
}

impl From<String> for ZFString {
    fn from(s: String) -> Self {
        ZFString(s)
    }
}

impl From<&str> for ZFString {
    fn from(s: &str) -> Self {
        ZFString(s.to_owned())
    }
}

impl Deserializable for ZFString {
    fn try_deserialize(bytes: &[u8]) -> Result<ZFString>
    where
        Self: Sized,
    {
        Ok(ZFString(String::from_utf8(bytes.to_vec()).map_err(
            |e| zferror!(ErrorKind::DeseralizationError, e),
        )?))
    }
}

#[derive(Debug, Clone, ZFData)]
pub struct ZFUsize(pub usize);

impl ZFData for ZFUsize {
    fn try_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.0.to_ne_bytes().to_vec())
    }
}

impl Deserializable for ZFUsize {
    fn try_deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let value = usize::from_ne_bytes(
            bytes
                .try_into()
                .map_err(|e| zferror!(ErrorKind::DeseralizationError, e))?,
        );
        Ok(ZFUsize(value))
    }
}

#[derive(Debug, Clone)]
pub struct ZFEmptyState;

#[derive(Debug, Clone, ZFData)]
pub struct ZFBytes(pub Vec<u8>);

impl ZFData for ZFBytes {
    fn try_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.0.clone())
    }
}

impl Deserializable for ZFBytes {
    fn try_deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(ZFBytes(bytes.into()))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, ZFData)]
pub struct GamepadInput {
    pub left_trigger: f32,
    pub right_trigger: f32,
    pub left_stick_x: f32,
}

impl Default for GamepadInput {
    fn default() -> Self {
        Self {
            left_trigger: 0.0,
            right_trigger: 0.0,
            left_stick_x: 0.0,
        }
    }
}

impl ZFData for GamepadInput {
    fn try_serialize(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)
            .map_err(|e| zferror!(ErrorKind::SerializationError, "{}", e))?)
    }
}

impl Deserializable for GamepadInput {
    fn try_deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let value =
            bincode::deserialize(bytes).map_err(|e| zferror!(ErrorKind::DeseralizationError, e))?;
        Ok(value)
    }
}

impl From<&GamepadInput> for crate::ros2::geometry::Twist {
    fn from(gamepad_input: &GamepadInput) -> Self {
        // left trigger indicates going backward
        // right trigger indicates going forward
        let linear_x = (gamepad_input.right_trigger as f64 - gamepad_input.left_trigger as f64)
            * crate::ros2::tb3::LINEAR_SCALING_FACTOR;

        // left stick x indicates going left / right.
        // However, it feels more natural if the values are swapped, hence the minus in front.
        let angular_z =
            -gamepad_input.left_stick_x as f64 * crate::ros2::tb3::ANGULAR_SCALING_FACTOR;

        Self {
            linear: crate::ros2::geometry::Vector3 {
                x: linear_x,
                y: 0.0,
                z: 0.0,
            },
            angular: crate::ros2::geometry::Vector3 {
                x: 0.0,
                y: 0.0,
                z: angular_z,
            },
        }
    }
}
