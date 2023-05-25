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

use std::sync::Arc;

use async_std::sync::Mutex;
use datatypes::data_types::{self, PointCloud2};
use datatypes::{COLUMBIA_PORT, GODAVARI_PORT, LOIRE_PORT};
use futures::prelude::*;
use futures::select;
use prost::Message as pMessage;
use rand::random;
use zenoh_flow::{anyhow, prelude::*};

#[derive(Debug, Clone)]
struct TripoliState {
    pointcloud2_data: data_types::PointCloud2,
    columbia_last_val: data_types::Image,
}

#[export_operator]
pub struct Tripoli {
    input_columbia: Input<data_types::Image>,
    input_godavari: Input<data_types::LaserScan>,
    output_loire: Output<data_types::PointCloud2>,
    state: Arc<Mutex<TripoliState>>,
}

#[async_trait::async_trait]
impl Operator for Tripoli {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_columbia: inputs
                .take(COLUMBIA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", COLUMBIA_PORT))
                .typed(|bytes| data_types::Image::decode(bytes).map_err(|e| anyhow!(e))),
            input_godavari: inputs
                .take(GODAVARI_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", GODAVARI_PORT))
                .typed(|bytes| data_types::LaserScan::decode(bytes).map_err(|e| anyhow!(e))),
            output_loire: outputs
                .take(LOIRE_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", LOIRE_PORT))
                .typed(|buffer, data: &PointCloud2| data.encode(buffer).map_err(|e| anyhow!(e))),
            state: Arc::new(Mutex::new(TripoliState {
                pointcloud2_data: random(),
                columbia_last_val: random(),
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Tripoli {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_columbia.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.columbia_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_godavari.recv().fuse() => {
                if let Ok((Message::Data(_inner_data),_)) = msg {

                    let guard_state = self.state.lock().await;

                    self.output_loire.send(guard_state.pointcloud2_data.clone(), None).await?;

                }
            }
        }
        Ok(())
    }
}
