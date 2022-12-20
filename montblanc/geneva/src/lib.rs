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
use datatypes::{ARKANSAS_PORT, CONGO_PORT, DANUBE_PORT, PARANA_PORT, TAGUS_PORT};
use futures::prelude::*;
use futures::select;
use rand::random;
use std::sync::Arc;
use zenoh_flow::prelude::*;

#[derive(Debug, Clone)]
struct GenevaState {
    danube_last_val: data_types::String,
    parana_last_val: data_types::String,
    tagus_last_val: data_types::Pose,
    congo_last_val: data_types::Twist,
}

#[export_operator]
pub struct Geneva {
    input_parana: Input<data_types::String>,
    input_danube: Input<data_types::String>,
    input_tagus: Input<data_types::Pose>,
    input_congo: Input<data_types::Twist>,
    output_arkansas: Output<data_types::String>,
    state: Arc<Mutex<GenevaState>>,
}

#[async_trait::async_trait]
impl Operator for Geneva {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_parana: inputs
                .take(PARANA_PORT)
                .expect(&format!("No Input called '{}' found", PARANA_PORT)),
            input_danube: inputs
                .take(DANUBE_PORT)
                .expect(&format!("No Input called '{}' found", DANUBE_PORT)),
            input_tagus: inputs
                .take(TAGUS_PORT)
                .expect(&format!("No Input called '{}' found", TAGUS_PORT)),
            input_congo: inputs
                .take(CONGO_PORT)
                .expect(&format!("No Input called '{}' found", CONGO_PORT)),
            output_arkansas: outputs
                .take(ARKANSAS_PORT)
                .expect(&format!("No Output called '{}' found", ARKANSAS_PORT)),
            state: Arc::new(Mutex::new(GenevaState {
                danube_last_val: data_types::String {
                    value: datatypes::random_string(1),
                },
                parana_last_val: data_types::String {
                    value: datatypes::random_string(1),
                },
                tagus_last_val: random(),
                congo_last_val: random(),
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Geneva {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_danube.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.danube_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_tagus.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.tagus_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_congo.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.congo_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_parana.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {

                    let mut guard_state = self.state.lock().await;
                    guard_state.parana_last_val = (*inner_data).clone();

                    let value = data_types::String {
                        value: format!(
                            "{}-{}",
                            guard_state.parana_last_val.value, guard_state.danube_last_val.value
                        ),
                    };

                    self.output_arkansas.send(value, None).await?;
                }
            }
        }
        Ok(())
    }
}
