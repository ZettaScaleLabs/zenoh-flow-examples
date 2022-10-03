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

use crate::{
    AMAZON_PORT, CHENAB_PORT, COLUMBIA_PORT, DANUBE_PORT, GANGES_PORT, NILE_PORT, YAMUNA_PORT,
};
use async_trait::async_trait;
use rand::random;
use std::{sync::Arc, time::Duration};
use zenoh_flow::prelude::*;
use zenoh_flow::zfresult::ZFResult;

// Cordoba SOURCE
#[derive(Debug)]
pub struct Cordoba;

#[async_trait]
impl Source for Cordoba {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_amazon = outputs.take_into_arc(AMAZON_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_amazon = Arc::clone(&output_amazon);

            async move {
                // Send every 100ms
                async_std::task::sleep(Duration::from_millis(100)).await;

                let data: f32 = random::<f32>() * 1000000.0;
                let value = datatypes::data_types::Float32 { value: data };
                let amazon_data = Data::from(value);
                output_amazon.send_async(amazon_data, None).await
            }
        })))
    }
}

// Portsmouth SOURCE
#[derive(Debug)]
pub struct Portsmouth;

#[async_trait]
impl Source for Portsmouth {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_danube = outputs.take_into_arc(DANUBE_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_danube = Arc::clone(&output_danube);

            async move {
                // Send every 200ms
                async_std::task::sleep(Duration::from_millis(200)).await;

                let value = datatypes::data_types::String {
                    value: datatypes::random_string(256),
                };
                let danube_data = Data::from(value);
                output_danube.send_async(danube_data, None).await
            }
        })))
    }
}

// Freeport SOURCE
#[derive(Debug)]
pub struct Freeport;

#[async_trait]
impl Source for Freeport {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_ganges = outputs.take_into_arc(GANGES_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_ganges = Arc::clone(&output_ganges);
            async move {
                // Send every 50ms
                async_std::task::sleep(Duration::from_millis(50)).await;

                let data: i64 = random::<i64>();
                let value = datatypes::data_types::Int64 { value: data };
                let ganges_data = Data::from(value);
                output_ganges.send_async(ganges_data, None).await
            }
        })))
    }
}

// Madelin SOURCE
#[derive(Debug)]
pub struct Madelin;

#[async_trait]
impl Source for Madelin {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_nile = outputs.take_into_arc(NILE_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_nile = Arc::clone(&output_nile);
            async move {
                // Send every 10ms
                async_std::task::sleep(Duration::from_millis(10)).await;

                let data: i32 = random::<i32>();
                let value = datatypes::data_types::Int32 { value: data };
                let nile_data = Data::from(value);
                output_nile.send_async(nile_data, None).await
            }
        })))
    }
}

// Delhi SOURCE
#[derive(Debug)]
pub struct Delhi;

#[async_trait]
impl Source for Delhi {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_columbia = outputs.take_into_arc(COLUMBIA_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_columbia = Arc::clone(&output_columbia);
            async move {
                // Send every 1s
                async_std::task::sleep(Duration::from_millis(1000)).await;
                let value: datatypes::data_types::Image = random();
                let columbia_data = Data::from(value);
                output_columbia.send_async(columbia_data, None).await
            }
        })))
    }
}

// Hebron SOURCE
#[derive(Debug)]
pub struct Hebron;

#[async_trait]
impl Source for Hebron {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_chenab = outputs.take_into_arc(CHENAB_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_chenab = Arc::clone(&output_chenab);
            async move {
                // Send every 100ms
                async_std::task::sleep(Duration::from_millis(100)).await;
                let value: datatypes::data_types::Quaternion = random();
                let chenab_data = Data::from(value);
                output_chenab.send_async(chenab_data, None).await
            }
        })))
    }
}

// Kingston SOURCE
#[derive(Debug)]
pub struct Kingston;

#[async_trait]
impl Source for Kingston {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let output_yamuna = outputs.take_into_arc(YAMUNA_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let output_yamuna = Arc::clone(&output_yamuna);
            async move {
                // Send every 100ms
                async_std::task::sleep(Duration::from_millis(100)).await;
                let value: datatypes::data_types::Vector3 = random();
                let yamuna_data = Data::from(value);
                output_yamuna.send_async(yamuna_data, None).await
            }
        })))
    }
}
