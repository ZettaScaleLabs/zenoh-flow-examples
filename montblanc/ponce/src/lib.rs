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
use datatypes::{
    BRAZOS_PORT, CONGO_PORT, DANUBE_PORT, LOIRE_PORT, MEKONG_PORT, MISSOURI_PORT, OHIO_PORT,
    TAGUS_PORT, VOLGA_PORT, YAMUNA_PORT,
};
use futures::prelude::*;
use futures::select;
use rand::random;
use std::sync::Arc;
use zenoh_flow::prelude::*;

#[derive(Debug, Clone)]
struct PonceState {
    danube_last_val: data_types::String,
    tagus_last_val: data_types::Pose,
    missouri_last_val: data_types::Image,
    loire_last_val: data_types::PointCloud2,
    yamuna_last_val: data_types::Vector3,

    ohio_last_val: data_types::Float32,
    volga_last_val: data_types::Float64,

    twist_data: data_types::Twist,
    twist_w_cov_data: data_types::TwistWithCovarianceStamped,
}

#[export_operator]
pub struct Ponce {
    input_danube: Input<data_types::String>,
    input_tagus: Input<data_types::Pose>,
    input_missouri: Input<data_types::Image>,
    input_loire: Input<data_types::PointCloud2>,
    input_yamuna: Input<data_types::Vector3>,
    input_ohio: Input<data_types::Float32>,
    input_volga: Input<data_types::Float64>,
    input_brazos: Input<data_types::PointCloud2>,
    output_congo: Output<data_types::Twist>,
    output_mekong: Output<data_types::TwistWithCovarianceStamped>,
    state: Arc<Mutex<PonceState>>,
}

#[async_trait::async_trait]
impl Operator for Ponce {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_danube: inputs
                .take(DANUBE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", DANUBE_PORT)),
            input_tagus: inputs
                .take(TAGUS_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", TAGUS_PORT)),
            input_missouri: inputs
                .take(MISSOURI_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", MISSOURI_PORT)),
            input_loire: inputs
                .take(LOIRE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", LOIRE_PORT)),
            input_yamuna: inputs
                .take(YAMUNA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", YAMUNA_PORT)),
            input_ohio: inputs
                .take(OHIO_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", OHIO_PORT)),
            input_volga: inputs
                .take(VOLGA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", VOLGA_PORT)),
            input_brazos: inputs
                .take(BRAZOS_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", BRAZOS_PORT)),

            output_congo: outputs
                .take(CONGO_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", CONGO_PORT)),
            output_mekong: outputs
                .take(MEKONG_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", MEKONG_PORT)),
            state: Arc::new(Mutex::new(PonceState {
                danube_last_val: data_types::String {
                    value: datatypes::random_string(1),
                },
                tagus_last_val: random(),
                missouri_last_val: random(),
                loire_last_val: random(),
                yamuna_last_val: random(),

                ohio_last_val: data_types::Float32 { value: random() },
                volga_last_val: data_types::Float64 { value: random() },

                twist_data: random(),
                twist_w_cov_data: random(),
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Ponce {
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
            msg = self.input_missouri.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.missouri_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_loire.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.loire_last_val = (*inner_data).clone();
                }
            },
            msg = self.input_yamuna.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.yamuna_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_ohio.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.ohio_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_volga.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    self.state.lock().await.volga_last_val = (*inner_data).clone();
                }
            },
            msg  = self.input_brazos.recv().fuse() => {
                if let Ok((Message::Data(_inner_data),_)) = msg {

                    let guard_state = self.state.lock().await;

                    self.output_congo.send(guard_state.twist_data.clone(), None).await?;
                    self.output_mekong.send(guard_state.twist_w_cov_data.clone(), None).await?;
                }
            }
        }
        Ok(())
    }
}
