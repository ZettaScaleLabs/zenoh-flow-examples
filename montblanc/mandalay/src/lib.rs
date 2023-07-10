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
    BRAZOS_PORT, CHENAB_PORT, DANUBE_PORT, GODAVARI_PORT, LOIRE_PORT, MISSOURI_PORT, SALWEEN_PORT,
    TAGUS_PORT, YAMUNA_PORT,
};
use futures::prelude::*;
use futures::select;
use prost::Message;
use rand::random;
use std::sync::Arc;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[derive(Debug, Clone)]
struct MandalayState {
    danube_last_val: data_types::String,
    chenab_last_val: data_types::Quaternion,
    salween_last_val: data_types::PointCloud2,
    godavari_last_val: data_types::LaserScan,
    loire_last_val: data_types::PointCloud2,
    yamuna_last_val: data_types::Vector3,
    pointcloud2_data: data_types::PointCloud2,
    pose_data: data_types::Pose,
    img_data: data_types::Image,
}

#[export_operator]
pub struct Mandalay {
    input_danube: Input<data_types::String>,
    input_chenab: Input<data_types::Quaternion>,
    input_salween: Input<data_types::PointCloud2>,
    input_godavari: Input<data_types::LaserScan>,
    input_loire: Input<data_types::PointCloud2>,
    input_yamuna: Input<data_types::Vector3>,
    output_brazos: Output<data_types::PointCloud2>,
    output_tagus: Output<data_types::Pose>,
    output_missouri: Output<data_types::Image>,
    state: Arc<Mutex<MandalayState>>,
}

#[async_trait::async_trait]
impl Operator for Mandalay {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_danube: inputs
                .take(DANUBE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", DANUBE_PORT))
                .typed(|buf| Ok(data_types::String::decode(buf)?)),
            input_chenab: inputs
                .take(CHENAB_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", CHENAB_PORT))
                .typed(|buf| Ok(data_types::Quaternion::decode(buf)?)),
            input_salween: inputs
                .take(SALWEEN_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", SALWEEN_PORT))
                .typed(|buf| Ok(data_types::PointCloud2::decode(buf)?)),
            input_godavari: inputs
                .take(GODAVARI_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", GODAVARI_PORT))
                .typed(|buf| Ok(data_types::LaserScan::decode(buf)?)),
            input_loire: inputs
                .take(LOIRE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", LOIRE_PORT))
                .typed(|buf| Ok(data_types::PointCloud2::decode(buf)?)),
            input_yamuna: inputs
                .take(YAMUNA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", YAMUNA_PORT))
                .typed(|buf| Ok(data_types::Vector3::decode(buf)?)),
            output_brazos: outputs
                .take(BRAZOS_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", BRAZOS_PORT))
                .typed(|buf, v: &data_types::PointCloud2| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
            output_tagus: outputs
                .take(TAGUS_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", TAGUS_PORT))
                .typed(|buf, v: &data_types::Pose| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
            output_missouri: outputs
                .take(MISSOURI_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", MISSOURI_PORT))
                .typed(|buf, v: &data_types::Image| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
            state: Arc::new(Mutex::new(MandalayState {
                danube_last_val: data_types::String {
                    value: datatypes::random_string(1),
                },
                chenab_last_val: random(),
                salween_last_val: random(),
                godavari_last_val: random(),
                loire_last_val: random(),
                yamuna_last_val: random(),
                pointcloud2_data: random(),
                pose_data: random(),
                img_data: random(),
            })),
        })
    }
}

#[async_trait::async_trait]
impl Node for Mandalay {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg = self.input_danube.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.danube_last_val = (*inner_data).clone();
                }}
            },
            msg  = self.input_chenab.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.chenab_last_val = (*inner_data).clone();
                }}
            },
            msg = self.input_salween.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.salween_last_val = (*inner_data).clone();
                }}
            },
            msg  = self.input_godavari.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.godavari_last_val = (*inner_data).clone();
                }}
            },
            msg = self.input_loire.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.loire_last_val = (*inner_data).clone();
                }}
            },
            msg  = self.input_yamuna.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(inner_data) = msg {
                    self.state.lock().await.yamuna_last_val = (*inner_data).clone();
                }}
            },
            // Output every 100ms
            _ = async_std::task::sleep(Duration::from_millis(100)).fuse() => {

                let guard_state = self.state.lock().await;

                self.output_brazos.send(guard_state.pointcloud2_data.clone(), None).await?;
                self.output_tagus.send(guard_state.pose_data.clone(), None).await?;
                self.output_missouri.send(guard_state.img_data.clone(), None).await?;
            }

        }
        Ok(())
    }
}
