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
use datatypes::{COLORADO_PORT, COLUMBIA_PORT, GODAVARI_PORT, PARANA_PORT, SALWEEN_PORT};
use futures::prelude::*;
use futures::select;
use rand::random;
use std::sync::Arc;
use zenoh_flow::prelude::*;

#[derive(Debug, Clone)]
struct OsakaState {
    parana_last_val: data_types::String,
    columbia_last_val: data_types::Image,
    _colorado_last_val: data_types::Image,
    pointcloud2_data: data_types::PointCloud2,
    laserscan_data: data_types::LaserScan,
}

#[export_operator]
pub struct Osaka {
    input_parana: Input<data_types::String>,
    input_columbia: Input<data_types::Image>,
    input_colorado: Input<data_types::Image>,
    output_salween: Output<data_types::PointCloud2>,
    output_godavari: Output<data_types::LaserScan>,
    state: Arc<Mutex<OsakaState>>,
}

#[async_trait::async_trait]
impl Operator for Osaka {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_parana: inputs
                .take(PARANA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", PARANA_PORT)),
            input_columbia: inputs
                .take(COLUMBIA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", COLUMBIA_PORT)),
            input_colorado: inputs
                .take(COLORADO_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", COLORADO_PORT)),
            output_salween: outputs
                .take(SALWEEN_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", SALWEEN_PORT)),
            output_godavari: outputs
                .take(GODAVARI_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", GODAVARI_PORT)),
            state: Arc::new(Mutex::new(OsakaState {
                parana_last_val: data_types::String {
                    value: datatypes::random_string(1),
                },
                columbia_last_val: random(),
                _colorado_last_val: random(),
                pointcloud2_data: random(),
                laserscan_data: random(),
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Osaka {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_parana.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.parana_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_columbia.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.columbia_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_colorado.recv().fuse() => {
                if let Ok((Message::Data(_inner_data),_)) = msg {

                    let guard_state = self.state.lock().await;

                    self.output_salween.send(guard_state.pointcloud2_data.clone(), None).await?;
                    self.output_godavari.send(guard_state.laserscan_data.clone(), None).await?;
                }
            }
        }
        Ok(())
    }
}
