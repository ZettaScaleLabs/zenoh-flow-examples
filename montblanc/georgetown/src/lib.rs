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
use datatypes::{LENA_PORT, MURRAY_PORT, VOLGA_PORT};
use futures::prelude::*;
use futures::select;
use rand::random;
use std::sync::Arc;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[derive(Debug, Clone)]
struct GeorgetownState {
    murray_last_val: data_types::Vector3Stamped,
    lena_last_val: data_types::WrenchStamped,

    f64_data: data_types::Float64,
}

#[export_operator]
pub struct Georgetown {
    input_murray: Input<data_types::Vector3Stamped>,
    input_lena: Input<data_types::WrenchStamped>,
    output_volga: Output<data_types::Float64>,
    state: Arc<Mutex<GeorgetownState>>,
}

#[async_trait::async_trait]
impl Operator for Georgetown {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_murray: inputs
                .take(MURRAY_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", MURRAY_PORT)),
            input_lena: inputs
                .take(LENA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", LENA_PORT)),
            output_volga: outputs
                .take(VOLGA_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", VOLGA_PORT)),
            state: Arc::new(Mutex::new(GeorgetownState {
                murray_last_val: random(),
                lena_last_val: random(),
                f64_data: data_types::Float64 { value: random() },
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Georgetown {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_murray.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.murray_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_lena.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.lena_last_val = (*inner_data).clone();
                }
            },
            // Output every 50ms
            _ = async_std::task::sleep(Duration::from_millis(50)).fuse() => {
                let guard_state = self.state.lock().await;
                self.output_volga.send(guard_state.f64_data.clone(), None).await?;
            }
        }
        Ok(())
    }
}
