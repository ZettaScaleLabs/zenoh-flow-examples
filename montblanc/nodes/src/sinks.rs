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
use std::sync::Arc;
use zenoh_flow::{
    AsyncIteration, Configuration, Context, Inputs, Message, Node, Sink, Streams, ZFResult,
};

use crate::ARKANSAS_PORT;

// Latency SINK
pub struct Arequipa;

#[async_trait]
impl Sink for Arequipa {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
    ) -> ZFResult<Option<Arc<dyn AsyncIteration>>> {
        let input_arkansas = inputs.take(ARKANSAS_PORT).unwrap();

        Ok(Some(Arc::new(async move || {
            if let Ok(Message::Data(mut msg)) = input_arkansas.recv_async().await {
                let data = msg
                    .get_inner_data()
                    .try_get::<datatypes::data_types::String>()?;
                println!("Arequipa: Received data {}", data.value);
            }
            Ok(())
        })))
    }
}

#[async_trait]
impl Node for Arequipa {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}
