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

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use zenoh_flow::{AsyncIteration, Configuration, Data, Node, Outputs, Source, ZFResult, Streams};

use rand::random;

use crate::{
    AMAZON_PORT, CHENAB_PORT, COLUMBIA_PORT, DANUBE_PORT, GANGES_PORT, NILE_PORT, YAMUNA_PORT,
};

// Cordoba SOURCE
#[derive(Debug)]
pub struct Cordoba;

#[async_trait]
impl Source for Cordoba {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_amazon = outputs.take(AMAZON_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 100ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(100)).await;

            let data: f32 = random::<f32>() * 1000000.0;
            let value = datatypes::data_types::Float32 { value: data };
            let amazon_data = Data::from::<datatypes::data_types::Float32>(value);
            output_amazon.send_async(amazon_data, None).await
        }))
    }
}

#[async_trait]
impl Node for Cordoba {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Portsmouth SOURCE
#[derive(Debug)]
pub struct Portsmouth;

#[async_trait]
impl Source for Portsmouth {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_danube = outputs.take(DANUBE_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 200ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(200)).await;

            let value = datatypes::data_types::String {
                value: datatypes::random_string(256),
            };
            let danube_data = Data::from(value);
            output_danube.send_async(danube_data, None).await
        }))
    }
}

#[async_trait]
impl Node for Portsmouth {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Freeport SOURCE
#[derive(Debug)]
pub struct Freeport;

#[async_trait]
impl Source for Freeport {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_ganges = outputs.take(GANGES_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 50ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(50)).await;

            let data: i64 = random::<i64>();
            let value = datatypes::data_types::Int64 { value: data };
            let ganges_data = Data::from(value);
            output_ganges.send_async(ganges_data, None).await
        }))
    }
}
#[async_trait]
impl Node for Freeport {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Madelin SOURCE
#[derive(Debug)]
pub struct Madelin;

#[async_trait]
impl Source for Madelin {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_nile = outputs.take(NILE_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 10ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(10)).await;

            let data: i32 = random::<i32>();
            let value = datatypes::data_types::Int32 { value: data };
            let nile_data = Data::from(value);
            output_nile.send_async(nile_data, None).await
        }))
    }
}
#[async_trait]
impl Node for Madelin {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Delhi SOURCE
#[derive(Debug)]
pub struct Delhi;

#[async_trait]
impl Source for Delhi {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_columbia = outputs.take(COLUMBIA_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 1s
            zenoh_flow::async_std::task::sleep(Duration::from_millis(1000)).await;
            let value: datatypes::data_types::Image = random();
            let columbia_data = Data::from(value);
            output_columbia.send_async(columbia_data, None).await
        }))
    }
}

#[async_trait]
impl Node for Delhi {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Hebron SOURCE
#[derive(Debug)]
pub struct Hebron;

#[async_trait]
impl Source for Hebron {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_chenab = outputs.take(CHENAB_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 100ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(100)).await;
            let value: datatypes::data_types::Quaternion = random();
            let chenab_data = Data::from(value);
            output_chenab.send_async(chenab_data, None).await
        }))
    }
}
#[async_trait]
impl Node for Hebron {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

// Kingston SOURCE
#[derive(Debug)]
pub struct Kingston;

#[async_trait]
impl Source for Kingston {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let output_yamuna = outputs.take(YAMUNA_PORT).unwrap();

        Ok(Arc::new(async move || {
            // Send every 100ms
            zenoh_flow::async_std::task::sleep(Duration::from_millis(100)).await;
            let value: datatypes::data_types::Vector3 = random();
            let yamuna_data = Data::from(value);
            output_yamuna.send_async(yamuna_data, None).await
        }))
    }
}

#[async_trait]
impl Node for Kingston {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}
