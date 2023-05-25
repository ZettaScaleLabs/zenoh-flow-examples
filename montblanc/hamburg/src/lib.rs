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

use async_std::sync::Mutex;
use datatypes::data_types;
use datatypes::{DANUBE_PORT, GANGES_PORT, NILE_PORT, PARANA_PORT, TIGRIS_PORT};
use futures::prelude::*;
use futures::select;
use prost::Message as pMessage;
use std::sync::Arc;
use zenoh_flow::{anyhow, prelude::*};

#[derive(Debug, Clone)]
struct HamburgState {
    ganges_last_val: i64,
    nile_last_val: i32,
    tigris_last_val: f32,
}

#[export_operator]
pub struct Hamburg {
    input_tigris: Input<data_types::Float32>,
    input_ganges: Input<data_types::Int64>,
    input_nile: Input<data_types::Int32>,
    input_danube: Input<data_types::String>,
    output_parana: Output<data_types::String>,
    state: Arc<Mutex<HamburgState>>,
}

#[async_trait::async_trait]
impl Operator for Hamburg {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_tigris: inputs
                .take(TIGRIS_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", TIGRIS_PORT))
                .typed(|bytes| data_types::Float32::decode(bytes).map_err(|e| anyhow!(e))),
            input_ganges: inputs
                .take(GANGES_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", GANGES_PORT))
                .typed(|bytes| data_types::Int64::decode(bytes).map_err(|e| anyhow!(e))),
            input_nile: inputs
                .take(NILE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", NILE_PORT))
                .typed(|bytes| data_types::Int32::decode(bytes).map_err(|e| anyhow!(e))),
            input_danube: inputs
                .take(DANUBE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", DANUBE_PORT))
                .typed(|bytes| data_types::String::decode(bytes).map_err(|e| anyhow!(e))),
            output_parana: outputs
                .take(PARANA_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", PARANA_PORT))
                .typed(|buffer, data: &data_types::String| {
                    data.encode(buffer).map_err(|e| anyhow!(e))
                }),
            state: Arc::new(Mutex::new(HamburgState {
                ganges_last_val: 0i64,
                nile_last_val: 0i32,
                tigris_last_val: 0.0f32,
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Hamburg {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_tigris.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.tigris_last_val = inner_data.value;
                }
            },
            msg  = self.input_ganges.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.ganges_last_val = inner_data.value;
                }
            },
            msg  = self.input_nile.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.nile_last_val = inner_data.value;
                }
            },
            msg  = self.input_danube.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    let new_value = data_types::String {
                        value: format!("hamburg/parana:{}", inner_data.value)
                    };
                    self.output_parana.send(new_value, None).await?;
                }
            }
        }
        Ok(())
    }
}
