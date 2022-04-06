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

use std::convert::TryInto;
use zenoh_flow::zenoh_flow_derive::{ZFData, ZFState};
use zenoh_flow::{Deserializable, ZFData, ZFError, ZFResult};
// We may want to provide some "built-in" types
pub mod ros2;

#[derive(Debug, Clone, ZFData)]
pub struct ZFString(pub String);

impl ZFData for ZFString {
    fn try_serialize(&self) -> zenoh_flow::ZFResult<Vec<u8>> {
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
    fn try_deserialize(bytes: &[u8]) -> ZFResult<ZFString>
    where
        Self: Sized,
    {
        Ok(ZFString(
            String::from_utf8(bytes.to_vec()).map_err(|_| ZFError::DeseralizationError)?,
        ))
    }
}

#[derive(Debug, Clone, ZFData)]
pub struct ZFUsize(pub usize);

impl ZFData for ZFUsize {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        Ok(self.0.to_ne_bytes().to_vec())
    }
}

impl Deserializable for ZFUsize {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        let value =
            usize::from_ne_bytes(bytes.try_into().map_err(|_| ZFError::DeseralizationError)?);
        Ok(ZFUsize(value))
    }
}

#[derive(Debug, Clone, ZFState)]
pub struct ZFEmptyState;

#[derive(Debug, Clone, ZFData)]
pub struct ZFBytes(pub Vec<u8>);

impl ZFData for ZFBytes {
    fn try_serialize(&self) -> ZFResult<Vec<u8>> {
        Ok(self.0.clone())
    }
}

impl Deserializable for ZFBytes {
    fn try_deserialize(bytes: &[u8]) -> ZFResult<Self>
    where
        Self: Sized,
    {
        Ok(ZFBytes(bytes.into()))
    }
}
