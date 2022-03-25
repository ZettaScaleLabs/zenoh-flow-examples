//
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

use async_trait::async_trait;
use zenoh_flow::{Configuration, Context, Node, Sink, State, ZFResult};

// Latency SINK
pub struct Arequipa;

#[async_trait]
impl Sink for Arequipa {
    async fn run(
        &self,
        _context: &mut Context,
        _state: &mut State,
        mut input: zenoh_flow::runtime::message::DataMessage,
    ) -> zenoh_flow::ZFResult<()> {
        let data = input
            .get_inner_data()
            .try_get::<datatypes::data_types::String>()?;
        println!("Arequipa: Received data {}", data.value);
        Ok(())
    }
}

impl Node for Arequipa {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zenoh_flow::zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}
