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
use async_trait::async_trait;
use datatypes::data_types;
use futures::prelude::*;
use futures::select;
use rand::random;
use std::sync::Arc;
use std::time::Duration;
use zenoh_flow::prelude::*;
use zenoh_flow::zfresult::ZFResult;

use crate::AMAZON_PORT;
use crate::ARKANSAS_PORT;
use crate::BRAZOS_PORT;
use crate::CHENAB_PORT;
use crate::COLORADO_PORT;
use crate::COLUMBIA_PORT;
use crate::CONGO_PORT;
use crate::DANUBE_PORT;
use crate::GANGES_PORT;
use crate::GODAVARI_PORT;
use crate::LENA_PORT;
use crate::LOIRE_PORT;
use crate::MEKONG_PORT;
use crate::MISSOURI_PORT;
use crate::MURRAY_PORT;
use crate::NILE_PORT;
use crate::OHIO_PORT;
use crate::PARANA_PORT;
use crate::SALWEEN_PORT;
use crate::TAGUS_PORT;
use crate::TIGRIS_PORT;
use crate::VOLGA_PORT;
use crate::YAMUNA_PORT;

// Lyon OPERATOR

#[derive(Debug)]
pub struct Lyon;

#[async_trait]
impl Operator for Lyon {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input = inputs.take_into_arc(AMAZON_PORT).unwrap();
        let output = outputs.take_into_arc(TIGRIS_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input = Arc::clone(&input);
            let output = Arc::clone(&output);
            async move {
                if let Ok(Message::Data(mut msg)) = input.recv_async().await {
                    output
                        .send_async(msg.get_inner_data().clone(), None)
                        .await
                        .unwrap();
                }
                Ok(())
            }
        })))
    }
}

// Hamburg OPERATOR

#[derive(Debug)]
pub struct Hamburg;

#[derive(Debug, Clone)]
struct HamburgState {
    ganges_last_val: i64,
    nile_last_val: i32,
    tigris_last_val: f32,
}

#[async_trait]
impl Operator for Hamburg {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let mut my_state = HamburgState {
            ganges_last_val: 0i64,
            nile_last_val: 0i32,
            tigris_last_val: 0.0f32,
        };

        let input_tigris = inputs.take_into_arc(TIGRIS_PORT).unwrap();
        let input_ganges = inputs.take_into_arc(GANGES_PORT).unwrap();
        let input_nile = inputs.take_into_arc(NILE_PORT).unwrap();
        let input_danube = inputs.take_into_arc(DANUBE_PORT).unwrap();
        let output_parana = outputs.take_into_arc(PARANA_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_tigris = Arc::clone(&input_tigris);
            let input_ganges = Arc::clone(&input_ganges);
            let input_nile = Arc::clone(&input_nile);
            let input_danube = Arc::clone(&input_danube);
            let output_parana = Arc::clone(&output_parana);

            async move {
                select! {
                    msg = input_tigris.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Float32>()?;
                            my_state.tigris_last_val = inner_data.value;
                        }
                    },
                    msg = input_ganges.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Int64>()?;
                            my_state.ganges_last_val = inner_data.value;
                        }
                    },
                    msg = input_nile.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Int32>()?;
                            my_state.nile_last_val = inner_data.value;
                        }
                    },
                    msg = input_danube.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            let new_value = data_types::String {
                                value: format!(
                                    "{}-{}-{}-{}",
                                    inner_data.value,
                                    my_state.tigris_last_val,
                                    my_state.ganges_last_val,
                                    my_state.nile_last_val
                                ),
                            };

                            let data = Data::from(new_value);
                            output_parana.send_async(data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Taipei OPERATOR

#[derive(Debug)]
pub struct Taipei;

#[async_trait]
impl Operator for Taipei {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input = inputs.take_into_arc(COLUMBIA_PORT).unwrap();
        let output = outputs.take_into_arc(COLORADO_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input = Arc::clone(&input);
            let output = Arc::clone(&output);

            async move {
                if let Ok(Message::Data(mut msg)) = input.recv_async().await {
                    output
                        .send_async(msg.get_inner_data().clone(), None)
                        .await
                        .unwrap();
                }
                Ok(())
            }
        })))
    }
}

// Osaka OPERATOR

#[derive(Debug)]
pub struct Osaka;

#[derive(Debug, Clone)]
struct OsakaState {
    parana_last_val: data_types::String,
    columbia_last_val: data_types::Image,
    _colorado_last_val: data_types::Image,
    pointcloud2_data: data_types::PointCloud2,
    laserscan_data: data_types::LaserScan,
}

#[async_trait]
impl Operator for Osaka {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(OsakaState {
            parana_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            columbia_last_val: random(),
            _colorado_last_val: random(),
            pointcloud2_data: random(),
            laserscan_data: random(),
        }));

        let input_parana = inputs.take_into_arc(PARANA_PORT).unwrap();
        let input_columbia = inputs.take_into_arc(COLUMBIA_PORT).unwrap();
        let input_colorado = inputs.take_into_arc(COLORADO_PORT).unwrap();
        let output_salween = outputs.take_into_arc(SALWEEN_PORT).unwrap();
        let output_godavari = outputs.take_into_arc(GODAVARI_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_parana = Arc::clone(&input_parana);
            let input_columbia = Arc::clone(&input_columbia);
            let input_colorado = Arc::clone(&input_colorado);
            let output_salween = Arc::clone(&output_salween);
            let output_godavari = Arc::clone(&output_godavari);

            let my_state = Arc::clone(&my_state);

            async move {
                let mut state = my_state.lock().await;
                select! {
                    msg = input_parana.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            state.parana_last_val = inner_data.clone();
                        }
                    },
                    msg = input_columbia.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Image>()?;
                            state.columbia_last_val = inner_data.clone();
                        }
                    },
                    msg = input_colorado.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let _inner_data = msg.get_inner_data().try_get::<data_types::Image>()?;
                            let salween_data = Data::from(state.pointcloud2_data.clone());
                            let godavari_data = Data::from(state.laserscan_data.clone());

                            output_salween.send_async(salween_data, None).await?;
                            output_godavari.send_async(godavari_data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Tripoli OPERATOR

#[derive(Debug)]
pub struct Tripoli;

#[derive(Debug, Clone)]
struct TripoliState {
    pointcloud2_data: data_types::PointCloud2,
    columbia_last_val: data_types::Image,
}

#[async_trait]
impl Operator for Tripoli {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(TripoliState {
            pointcloud2_data: random(),
            columbia_last_val: random(),
        }));

        let input_columbia = inputs.take_into_arc(COLUMBIA_PORT).unwrap();
        let input_godavari = inputs.take_into_arc(GODAVARI_PORT).unwrap();
        let output_loire = outputs.take_into_arc(LOIRE_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_columbia = Arc::clone(&input_columbia);
            let input_godavari = Arc::clone(&input_godavari);
            let output_loire = Arc::clone(&output_loire);
            let my_state = Arc::clone(&my_state);

            async move {
                let mut state = my_state.lock().await;
                select! {
                    msg = input_columbia.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Image>()?;
                            state.columbia_last_val = inner_data.clone();
                        }
                    },
                    msg = input_godavari.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let _inner_data = msg.get_inner_data().try_get::<data_types::LaserScan>()?;
                            let loire_data = Data::from(state.pointcloud2_data.clone());

                            output_loire.send_async(loire_data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Mandalay OPERATOR

#[derive(Debug)]
pub struct Mandalay;

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

#[async_trait]
impl Operator for Mandalay {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(MandalayState {
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
        }));

        let input_danube = inputs.take_into_arc(DANUBE_PORT).unwrap();
        let input_chenab = inputs.take_into_arc(CHENAB_PORT).unwrap();
        let input_salween = inputs.take_into_arc(SALWEEN_PORT).unwrap();
        let input_godavari = inputs.take_into_arc(GODAVARI_PORT).unwrap();
        let input_loire = inputs.take_into_arc(LOIRE_PORT).unwrap();
        let input_yamuna = inputs.take_into_arc(YAMUNA_PORT).unwrap();

        let output_brazos = outputs.take_into_arc(BRAZOS_PORT).unwrap();
        let output_tagus = outputs.take_into_arc(TAGUS_PORT).unwrap();
        let output_missouri = outputs.take_into_arc(MISSOURI_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_danube = Arc::clone(&input_danube);
            let input_chenab = Arc::clone(&input_chenab);
            let input_salween = Arc::clone(&input_salween);
            let input_godavari = Arc::clone(&input_godavari);
            let input_loire = Arc::clone(&input_loire);
            let input_yamuna = Arc::clone(&input_yamuna);

            let output_brazos = Arc::clone(&output_brazos);
            let output_tagus = Arc::clone(&output_tagus);
            let output_missouri = Arc::clone(&output_missouri);

            let state = Arc::clone(&my_state);
            async move {
                let mut my_state = state.lock().await;
                select! {
                    msg = input_danube.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            my_state.danube_last_val = inner_data.clone();
                        }
                    },
                    msg = input_chenab.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Quaternion>()?;
                            my_state.chenab_last_val = inner_data.clone();
                        }
                    },
                    msg = input_salween.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::PointCloud2>()?;
                            my_state.salween_last_val = inner_data.clone();
                        }
                    },
                    msg = input_godavari.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::LaserScan>()?;
                            my_state.godavari_last_val = inner_data.clone();
                        }
                    },
                    msg = input_loire.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::PointCloud2>()?;
                            my_state.loire_last_val = inner_data.clone();
                        }
                    },
                    msg = input_yamuna.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Vector3>()?;
                            my_state.yamuna_last_val = inner_data.clone();
                        }
                    },
                    // Output every 100ms
                    _ = async_std::task::sleep(Duration::from_millis(100)).fuse() => {
                        let brazos_data = Data::from(my_state.pointcloud2_data.clone());
                        let tagus_data = Data::from(my_state.pose_data.clone());
                        let missouri_data = Data::from(my_state.img_data.clone());

                        output_brazos.send_async(brazos_data, None).await?;
                        output_tagus.send_async(tagus_data, None).await?;
                        output_missouri.send_async(missouri_data, None).await?;
                    }
                }
                Ok(())
            }
        })))
    }
}

// Ponce OPERATOR

#[derive(Debug)]
pub struct Ponce;

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

#[async_trait]
impl Operator for Ponce {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(PonceState {
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
        }));

        let input_danube = inputs.take_into_arc(DANUBE_PORT).unwrap();
        let input_tagus = inputs.take_into_arc(TAGUS_PORT).unwrap();
        let input_missouri = inputs.take_into_arc(MISSOURI_PORT).unwrap();
        let input_loire = inputs.take_into_arc(LOIRE_PORT).unwrap();
        let input_yamuna = inputs.take_into_arc(YAMUNA_PORT).unwrap();
        let input_ohio = inputs.take_into_arc(OHIO_PORT).unwrap();
        let input_volga = inputs.take_into_arc(VOLGA_PORT).unwrap();
        let input_brazos = inputs.take_into_arc(BRAZOS_PORT).unwrap();

        let output_congo = outputs.take_into_arc(CONGO_PORT).unwrap();
        let output_mekong = outputs.take_into_arc(MEKONG_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_danube = Arc::clone(&input_danube);
            let input_tagus = Arc::clone(&input_tagus);
            let input_missouri = Arc::clone(&input_missouri);
            let input_loire = Arc::clone(&input_loire);
            let input_yamuna = Arc::clone(&input_yamuna);
            let input_ohio = Arc::clone(&input_ohio);
            let input_volga = Arc::clone(&input_volga);
            let input_brazos = Arc::clone(&input_brazos);

            let output_congo = Arc::clone(&output_congo);
            let output_mekong = Arc::clone(&output_mekong);

            let state = Arc::clone(&my_state);

            async move {
                let mut my_state = state.lock().await;
                select! {
                    msg = input_danube.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            my_state.danube_last_val = inner_data.clone();
                        }
                    },
                    msg = input_tagus.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Pose>()?;
                            my_state.tagus_last_val = inner_data.clone();
                        }
                    },
                    msg = input_missouri.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Image>()?;
                            my_state.missouri_last_val = inner_data.clone();
                        }
                    },
                    msg = input_loire.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::PointCloud2>()?;
                            my_state.loire_last_val = inner_data.clone();
                        }
                    },
                    msg = input_yamuna.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Vector3>()?;
                            my_state.yamuna_last_val = inner_data.clone();
                        }
                    },
                    msg = input_ohio.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Float32>()?;
                            my_state.ohio_last_val = inner_data.clone();
                        }
                    },
                    msg = input_volga.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Float64>()?;
                            my_state.volga_last_val = inner_data.clone();
                        }
                    },
                    msg = input_brazos.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let _inner_data = msg.get_inner_data().try_get::<data_types::PointCloud2>()?;

                            let twist_data = Data::from(my_state.twist_data.clone());

                            let twist_w_cov_data = Data::from(
                                my_state.twist_w_cov_data.clone(),
                            );

                            output_congo.send_async(twist_data, None).await?;
                            output_mekong.send_async(twist_w_cov_data, None).await?;

                        }
                    },
                }
                Ok(())
            }
        })))
    }
}

// Monaco OPERATOR

#[derive(Debug)]
pub struct Monaco;

#[async_trait]
impl Operator for Monaco {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input_congo = inputs.take_into_arc(CONGO_PORT).unwrap();
        let output_ohio = outputs.take_into_arc(OHIO_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_congo = Arc::clone(&input_congo);
            let output_ohio = Arc::clone(&output_ohio);

            async move {
                select! {
                    msg = input_congo.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let _inner_data = msg.get_inner_data().try_get::<data_types::Twist>()?;
                            let ohio_data = Data::from(data_types::Float32 { value: random() });
                            output_ohio.send_async(ohio_data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Barcelona OPERATOR

#[derive(Debug)]
pub struct Barcelona;

#[async_trait]
impl Operator for Barcelona {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input_mekong = inputs.take_into_arc(MEKONG_PORT).unwrap();
        let output_lena = outputs.take_into_arc(LENA_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_mekong = Arc::clone(&input_mekong);
            let output_lena = Arc::clone(&output_lena);

            async move {
                select! {
                    msg = input_mekong.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let mekong_data = msg.get_inner_data().try_get::<data_types::TwistWithCovarianceStamped>()?;

                            let wrench = data_types::WrenchStamped {
                                header: Some(mekong_data.header.as_ref().ok_or_else(|| zferror!(ErrorKind::Empty))?.clone()),
                                wrench: Some(data_types::Wrench {
                                    force: mekong_data
                                        .twist
                                        .as_ref()
                                        .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                        .twist
                                        .as_ref()
                                        .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                        .linear
                                        .clone(),
                                    torque: mekong_data
                                        .twist
                                        .as_ref()
                                        .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                        .twist
                                        .as_ref()
                                        .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                        .angular
                                        .clone(),
                                }),
                            };

                            let lena_data = Data::from(wrench);
                            output_lena.send_async(lena_data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Rotterdam OPERATOR

#[derive(Debug)]
pub struct Rotterdam;

#[async_trait]
impl Operator for Rotterdam {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input_mekong = inputs.take_into_arc(MEKONG_PORT).unwrap();
        let output_murray = outputs.take_into_arc(MURRAY_PORT).unwrap();

        let header_data: data_types::Header = random();

        Ok(Some(Box::new(move || {
            let input_mekong = Arc::clone(&input_mekong);
            let output_murray = Arc::clone(&output_murray);
            let header_data = header_data.clone();

            async move {
                select! {
                    msg = input_mekong.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let _mekong_data = msg.get_inner_data().try_get::<data_types::TwistWithCovarianceStamped>()?;

                            let vec3s = data_types::Vector3Stamped {
                                header: Some(header_data.clone()),
                                vector: random(),
                                // vector: mekong_data
                                //     .twist
                                //     .as_ref()
                                //     .ok_or(ZFError::Empty)?
                                //     .twist
                                //     .as_ref()
                                //     .ok_or(ZFError::Empty)?
                                //     .linear
                                //     .clone(),
                            };

                            let murray_data = Data::from(vec3s);
                            output_murray.send_async(murray_data, None).await?;
                        }
                    }
                }
                Ok(())
            }
        })))
    }
}

// Georgetown OPERATOR

#[derive(Debug)]
pub struct Georgetown;

#[derive(Debug, Clone)]
struct GeorgetownState {
    murray_last_val: data_types::Vector3Stamped,
    lena_last_val: data_types::WrenchStamped,

    f64_data: data_types::Float64,
}
#[async_trait]
impl Operator for Georgetown {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(GeorgetownState {
            murray_last_val: random(),
            lena_last_val: random(),
            f64_data: data_types::Float64 { value: random() },
        }));

        let input_murray = inputs.take_into_arc(MURRAY_PORT).unwrap();
        let input_lena = inputs.take_into_arc(LENA_PORT).unwrap();
        let output_volga = outputs.take_into_arc(VOLGA_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_murray = Arc::clone(&input_murray);
            let input_lena = Arc::clone(&input_lena);
            let output_volga = Arc::clone(&output_volga);

            let state = Arc::clone(&my_state);

            async move {
                let mut my_state = state.lock().await;
                select! {
                msg = input_murray.recv_async().fuse() => {
                    if let Ok(Message::Data(mut msg)) = msg {
                        let inner_data = msg.get_inner_data().try_get::<data_types::Vector3Stamped>()?;
                        my_state.murray_last_val = inner_data.clone();
                    }
                },
                msg = input_lena.recv_async().fuse() => {
                    if let Ok(Message::Data(mut msg)) = msg {
                        let inner_data = msg.get_inner_data().try_get::<data_types::WrenchStamped>()?;
                        my_state.lena_last_val = inner_data.clone();
                    }
                },
                // Output every 50ms
                _ = async_std::task::sleep(Duration::from_millis(50)).fuse() => {
                        let volga_data = Data::from(my_state.f64_data.clone());
                        output_volga.send_async(volga_data, None).await?;
                    }
                }
                Ok(())
            }
        })))
    }
}

#[derive(Debug)]
pub struct Geneva;

#[derive(Debug, Clone)]
struct GenevaState {
    danube_last_val: data_types::String,
    parana_last_val: data_types::String,
    tagus_last_val: data_types::Pose,
    congo_last_val: data_types::Twist,
}

#[async_trait]
impl Operator for Geneva {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let my_state = Arc::new(Mutex::new(GenevaState {
            danube_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            parana_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            tagus_last_val: random(),
            congo_last_val: random(),
        }));

        let input_parana = inputs.take_into_arc(PARANA_PORT).unwrap();
        let input_danube = inputs.take_into_arc(DANUBE_PORT).unwrap();
        let input_tagus = inputs.take_into_arc(TAGUS_PORT).unwrap();
        let input_congo = inputs.take_into_arc(CONGO_PORT).unwrap();

        let output_arkansas = outputs.take_into_arc(ARKANSAS_PORT).unwrap();

        Ok(Some(Box::new(move || {
            let input_parana = Arc::clone(&input_parana);
            let input_danube = Arc::clone(&input_danube);
            let input_tagus = Arc::clone(&input_tagus);
            let input_congo = Arc::clone(&input_congo);

            let output_arkansas = Arc::clone(&output_arkansas);

            let state = Arc::clone(&my_state);

            async move {
                let mut my_state = state.lock().await;
                select! {

                    msg = input_danube.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            my_state.danube_last_val = inner_data.clone();
                        }
                    },
                    msg = input_tagus.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Pose>()?;
                            my_state.tagus_last_val = inner_data.clone();
                        }
                    },
                    msg = input_congo.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::Twist>()?;
                            my_state.congo_last_val = inner_data.clone();
                        }
                    },
                    msg = input_parana.recv_async().fuse() => {
                        if let Ok(Message::Data(mut msg)) = msg {
                            let inner_data = msg.get_inner_data().try_get::<data_types::String>()?;
                            my_state.parana_last_val = inner_data.clone();

                            let new_value = data_types::String {
                                value: format!(
                                    "{}-{}",
                                    my_state.parana_last_val.value, my_state.danube_last_val.value
                                ),
                            };

                            let arkansas_data = Data::from(new_value);
                            output_arkansas.send_async(arkansas_data, None).await?;

                        }
                    },
                }
                Ok(())
            }
        })))
    }
}
