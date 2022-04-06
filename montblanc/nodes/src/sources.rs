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

use async_trait::async_trait;
use zenoh_flow::{Configuration, Context, Data, Node, Source, State, ZFResult};

use rand::random;

// Cordoba SOURCE
#[derive(Debug)]
pub struct Cordoba;

#[async_trait]
impl Source for Cordoba {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let data: f32 = random::<f32>() * 1000000.0;
        let value = datatypes::data_types::Float32 { value: data };

        Ok(Data::from::<datatypes::data_types::Float32>(value))
    }
}

impl Node for Cordoba {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Portsmouth SOURCE
#[derive(Debug)]
pub struct Portsmouth;

#[async_trait]
impl Source for Portsmouth {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let value = datatypes::data_types::String {
            value: datatypes::random_string(256),
        };

        Ok(Data::from::<datatypes::data_types::String>(value))
    }
}

impl Node for Portsmouth {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Freeport SOURCE
#[derive(Debug)]
pub struct Freeport;

#[async_trait]
impl Source for Freeport {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let data: i64 = random::<i64>();
        let value = datatypes::data_types::Int64 { value: data };

        Ok(Data::from::<datatypes::data_types::Int64>(value))
    }
}

impl Node for Freeport {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Madelin SOURCE
#[derive(Debug)]
pub struct Madelin;

#[async_trait]
impl Source for Madelin {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let data: i32 = random::<i32>();
        let value = datatypes::data_types::Int32 { value: data };

        Ok(Data::from::<datatypes::data_types::Int32>(value))
    }
}

impl Node for Madelin {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Delhi SOURCE
#[derive(Debug)]
pub struct Delhi;

#[async_trait]
impl Source for Delhi {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let value: datatypes::data_types::Image = random();

        Ok(Data::from::<datatypes::data_types::Image>(value))
    }
}

impl Node for Delhi {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Hebron SOURCE
#[derive(Debug)]
pub struct Hebron;

#[async_trait]
impl Source for Hebron {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let value: datatypes::data_types::Quaternion = random();

        Ok(Data::from::<datatypes::data_types::Quaternion>(value))
    }
}

impl Node for Hebron {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Kingston SOURCE
#[derive(Debug)]
pub struct Kingston;

#[async_trait]
impl Source for Kingston {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let value: datatypes::data_types::Vector3 = random();

        Ok(Data::from::<datatypes::data_types::Vector3>(value))
    }
}

impl Node for Kingston {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}
