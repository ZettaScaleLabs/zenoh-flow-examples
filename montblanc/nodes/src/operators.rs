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

use datatypes::data_types;
use rand::random;
use std::collections::HashMap;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::{
    default_input_rule, default_output_rule, zf_empty_state, Configuration, Data, InputToken,
    LocalDeadlineMiss, Node, NodeOutput, Operator, PortId, State, ZFError, ZFResult,
};

// Lyon OPERATOR

#[derive(Debug)]
pub struct Lyon;

impl Operator for Lyon {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let data = inputs
            .get_mut(crate::AMAZON_PORT)
            .ok_or(ZFError::Empty)?
            .get_inner_data()
            .clone();

        results.insert(crate::TIGRIS_PORT.into(), data);
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Lyon {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Hamburg OPERATOR

#[derive(Debug)]
pub struct Hamburg;

#[derive(ZFState, Debug, Clone)]
struct HamburgState {
    ganges_last_val: i64,
    nile_last_val: i32,
    tigris_last_val: f32,
}

impl Operator for Hamburg {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<HamburgState>()?;

        if let Some(tigris_value) = inputs.get_mut(crate::TIGRIS_PORT) {
            let inner_value = tigris_value
                .get_inner_data()
                .try_get::<data_types::Float32>()?;
            my_state.tigris_last_val = inner_value.value;
        }

        if let Some(ganges_value) = inputs.get_mut(crate::GANGES_PORT) {
            let inner_value = ganges_value
                .get_inner_data()
                .try_get::<data_types::Int64>()?;
            my_state.ganges_last_val = inner_value.value;
        }

        if let Some(nile_value) = inputs.get_mut(crate::NILE_PORT) {
            let inner_value = nile_value.get_inner_data().try_get::<data_types::Int32>()?;
            my_state.nile_last_val = inner_value.value;
        }

        if let Some(danube_value) = inputs.get_mut(crate::DANUBE_PORT) {
            let inner_value = danube_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            let new_value = data_types::String {
                value: format!(
                    "{}-{}-{}-{}",
                    inner_value.value,
                    my_state.tigris_last_val,
                    my_state.ganges_last_val,
                    my_state.nile_last_val
                ),
            };

            let data = Data::from::<data_types::String>(new_value);
            results.insert(crate::PARANA_PORT.into(), data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Hamburg {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = HamburgState {
            ganges_last_val: 0i64,
            nile_last_val: 0i32,
            tigris_last_val: 0.0f32,
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Taipei OPERATOR

#[derive(Debug)]
pub struct Taipei;

impl Operator for Taipei {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        // being triggered each time you receive a value
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let data = inputs
            .get_mut(crate::COLUMBIA_PORT)
            .ok_or(ZFError::Empty)?
            .get_inner_data()
            .clone();

        results.insert(crate::COLORADO_PORT.into(), data);
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Taipei {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Osaka OPERATOR

#[derive(Debug)]
pub struct Osaka;

#[derive(ZFState, Debug, Clone)]
struct OsakaState {
    parana_last_val: data_types::String,
    columbia_last_val: data_types::Image,
    pointcloud2_data: data_types::PointCloud2,
    laserscan_data: data_types::LaserScan,
}

impl Operator for Osaka {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<OsakaState>()?;

        if let Some(parana_value) = inputs.get_mut(crate::PARANA_PORT) {
            let inner_value = parana_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            my_state.parana_last_val = inner_value.clone();
        }

        if let Some(columbia_value) = inputs.get_mut(crate::COLUMBIA_PORT) {
            let inner_value = columbia_value
                .get_inner_data()
                .try_get::<data_types::Image>()?;
            my_state.columbia_last_val = inner_value.clone();
        }

        if let Some(colorado_value) = inputs.get_mut(crate::DANUBE_PORT) {
            let _inner_value = colorado_value
                .get_inner_data()
                .try_get::<data_types::Image>()?;
            let salween_data =
                Data::from::<data_types::PointCloud2>(my_state.pointcloud2_data.clone());
            let godavari_data =
                Data::from::<data_types::LaserScan>(my_state.laserscan_data.clone());

            results.insert(crate::SALWEEN_PORT.into(), salween_data);
            results.insert(crate::GODAVARI_PORT.into(), godavari_data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Osaka {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = OsakaState {
            parana_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            columbia_last_val: random(),
            pointcloud2_data: random(),
            laserscan_data: random(),
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Tripoli OPERATOR

#[derive(Debug)]
pub struct Tripoli;

#[derive(ZFState, Debug, Clone)]
struct TripoliState {
    pointcloud2_data: data_types::PointCloud2,
}

impl Operator for Tripoli {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<TripoliState>()?;

        if let Some(godavari_value) = inputs.get_mut(crate::GODAVARI_PORT) {
            let _inner_value = godavari_value
                .get_inner_data()
                .try_get::<data_types::LaserScan>()?;
            let loire_data =
                Data::from::<data_types::PointCloud2>(my_state.pointcloud2_data.clone());

            results.insert(crate::LOIRE_PORT.into(), loire_data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Tripoli {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = TripoliState {
            pointcloud2_data: random(),
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Mandalay OPERATOR

#[derive(Debug)]
pub struct Mandalay;

#[derive(ZFState, Debug, Clone)]
struct MandalayState {
    danube_last_val: data_types::String,
    chanab_last_val: data_types::Quaternion,
    salween_last_val: data_types::PointCloud2,
    godavari_last_val: data_types::LaserScan,
    loire_last_val: data_types::PointCloud2,
    yamuna_last_val: data_types::Vector3,
    pointcloud2_data: data_types::PointCloud2,
    pose_data: data_types::Pose,
    img_data: data_types::Image,
}

impl Operator for Mandalay {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<MandalayState>()?;

        if let Some(danube_value) = inputs.get_mut(crate::DANUBE_PORT) {
            let inner_value = danube_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            my_state.danube_last_val = inner_value.clone();
        }

        if let Some(chanab_value) = inputs.get_mut(crate::CHANAB_PORT) {
            let inner_value = chanab_value
                .get_inner_data()
                .try_get::<data_types::Quaternion>()?;
            my_state.chanab_last_val = inner_value.clone();
        }

        if let Some(salween_value) = inputs.get_mut(crate::SALWEEN_PORT) {
            let inner_value = salween_value
                .get_inner_data()
                .try_get::<data_types::PointCloud2>()?;
            my_state.salween_last_val = inner_value.clone();
        }

        if let Some(godavari_value) = inputs.get_mut(crate::GODAVARI_PORT) {
            let inner_value = godavari_value
                .get_inner_data()
                .try_get::<data_types::LaserScan>()?;
            my_state.godavari_last_val = inner_value.clone();
        }

        if let Some(loire_value) = inputs.get_mut(crate::LOIRE_PORT) {
            let inner_value = loire_value
                .get_inner_data()
                .try_get::<data_types::PointCloud2>()?;
            my_state.loire_last_val = inner_value.clone();
        }

        if let Some(yamuna_value) = inputs.get_mut(crate::YAMUNA_PORT) {
            let inner_value = yamuna_value
                .get_inner_data()
                .try_get::<data_types::Vector3>()?;
            my_state.yamuna_last_val = inner_value.clone();
        }

        if false {
            let brazos_data =
                Data::from::<data_types::PointCloud2>(my_state.pointcloud2_data.clone());

            let tagus_data = Data::from::<data_types::Pose>(my_state.pose_data.clone());

            let missouri_data = Data::from::<data_types::Image>(my_state.img_data.clone());

            results.insert(crate::BRAZOS_PORT.into(), brazos_data);
            results.insert(crate::TAGUS_PORT.into(), tagus_data);
            results.insert(crate::MISSOURI_PORT.into(), missouri_data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Mandalay {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = MandalayState {
            danube_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            chanab_last_val: random(),
            salween_last_val: random(),
            godavari_last_val: random(),
            loire_last_val: random(),
            yamuna_last_val: random(),
            pointcloud2_data: random(),
            pose_data: random(),
            img_data: random(),
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Ponce OPERATOR

#[derive(Debug)]
pub struct Ponce;

#[derive(ZFState, Debug, Clone)]
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

impl Operator for Ponce {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<PonceState>()?;

        if let Some(danube_value) = inputs.get_mut(crate::DANUBE_PORT) {
            let inner_value = danube_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            my_state.danube_last_val = inner_value.clone();
        }

        if let Some(tagus_value) = inputs.get_mut(crate::TAGUS_PORT) {
            let inner_value = tagus_value.get_inner_data().try_get::<data_types::Pose>()?;
            my_state.tagus_last_val = inner_value.clone();
        }

        if let Some(missouri_value) = inputs.get_mut(crate::MISSOURI_PORT) {
            let inner_value = missouri_value
                .get_inner_data()
                .try_get::<data_types::Image>()?;
            my_state.missouri_last_val = inner_value.clone();
        }

        if let Some(loire_value) = inputs.get_mut(crate::LOIRE_PORT) {
            let inner_value = loire_value
                .get_inner_data()
                .try_get::<data_types::PointCloud2>()?;
            my_state.loire_last_val = inner_value.clone();
        }

        if let Some(yamuna_value) = inputs.get_mut(crate::YAMUNA_PORT) {
            let inner_value = yamuna_value
                .get_inner_data()
                .try_get::<data_types::Vector3>()?;
            my_state.yamuna_last_val = inner_value.clone();
        }

        if let Some(ohio_value) = inputs.get_mut(crate::OHIO_PORT) {
            let inner_value = ohio_value
                .get_inner_data()
                .try_get::<data_types::Float32>()?;
            my_state.ohio_last_val = inner_value.clone();
        }

        if let Some(volga_value) = inputs.get_mut(crate::VOLGA_PORT) {
            let inner_value = volga_value
                .get_inner_data()
                .try_get::<data_types::Float64>()?;
            my_state.volga_last_val = inner_value.clone();
        }

        if let Some(brazos_value) = inputs.get_mut(crate::BRAZOS_PORT) {
            let _inner_value = brazos_value
                .get_inner_data()
                .try_get::<data_types::PointCloud2>()?;

            let twist_data = Data::from::<data_types::Twist>(my_state.twist_data.clone());

            let twist_w_cov_data = Data::from::<data_types::TwistWithCovarianceStamped>(
                my_state.twist_w_cov_data.clone(),
            );

            results.insert(crate::CONGO_PORT.into(), twist_data);
            results.insert(crate::MEKONG_PORT.into(), twist_w_cov_data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Ponce {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = PonceState {
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
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Monaco OPERATOR

#[derive(Debug)]
pub struct Monaco;

impl Operator for Monaco {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let _data = inputs
            .get_mut(crate::CONGO_PORT)
            .ok_or(ZFError::Empty)?
            .get_inner_data()
            .try_get::<data_types::Twist>()?;

        let ohio_data = Data::from::<data_types::Float32>(data_types::Float32 { value: random() });

        results.insert(crate::OHIO_PORT.into(), ohio_data);
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Monaco {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Barcelona OPERATOR

#[derive(Debug)]
pub struct Barcelona;

impl Operator for Barcelona {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let mekong_data = inputs
            .get_mut(crate::MEKONG_PORT)
            .ok_or(ZFError::Empty)?
            .get_inner_data()
            .try_get::<data_types::TwistWithCovarianceStamped>()?;

        let wrench = data_types::WrenchStamped {
            header: Some(mekong_data.header.as_ref().ok_or(ZFError::Empty)?.clone()),
            wrench: Some(data_types::Wrench {
                force: mekong_data
                    .twist
                    .as_ref()
                    .ok_or(ZFError::Empty)?
                    .twist
                    .as_ref()
                    .ok_or(ZFError::Empty)?
                    .linear
                    .clone(),
                torque: mekong_data
                    .twist
                    .as_ref()
                    .ok_or(ZFError::Empty)?
                    .twist
                    .as_ref()
                    .ok_or(ZFError::Empty)?
                    .angular
                    .clone(),
            }),
        };

        let lena_data = Data::from::<data_types::WrenchStamped>(wrench);

        results.insert(crate::LENA_PORT.into(), lena_data);
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Barcelona {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Rotterdam OPERATOR

#[derive(Debug)]
pub struct Rotterdam;

impl Operator for Rotterdam {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let mekong_data = inputs
            .get_mut(crate::MEKONG_PORT)
            .ok_or(ZFError::Empty)?
            .get_inner_data()
            .try_get::<data_types::TwistWithCovarianceStamped>()?;

        let vec3s = data_types::Vector3Stamped {
            header: Some(mekong_data.header.as_ref().ok_or(ZFError::Empty)?.clone()),
            vector: mekong_data
                .twist
                .as_ref()
                .ok_or(ZFError::Empty)?
                .twist
                .as_ref()
                .ok_or(ZFError::Empty)?
                .linear
                .clone(),
        };

        let murray_data = Data::from::<data_types::Vector3Stamped>(vec3s);

        results.insert(crate::MURRAY_PORT.into(), murray_data);
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Rotterdam {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Georgetown OPERATOR

#[derive(Debug)]
pub struct Georgetown;

#[derive(ZFState, Debug, Clone)]
struct GeorgetownState {
    murray_last_val: data_types::Vector3Stamped,
    lena_last_val: data_types::WrenchStamped,

    f64_data: data_types::Float64,
}

impl Operator for Georgetown {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<GeorgetownState>()?;

        if let Some(murray_value) = inputs.get_mut(crate::MURRAY_PORT) {
            let inner_value = murray_value
                .get_inner_data()
                .try_get::<data_types::Vector3Stamped>()?;
            my_state.murray_last_val = inner_value.clone();
        }

        if let Some(lena_value) = inputs.get_mut(crate::LENA_PORT) {
            let inner_value = lena_value
                .get_inner_data()
                .try_get::<data_types::WrenchStamped>()?;
            my_state.lena_last_val = inner_value.clone();
        }

        if false {
            let volga_data = Data::from::<data_types::Float64>(my_state.f64_data.clone());

            results.insert(crate::VOLGA_PORT.into(), volga_data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Georgetown {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = GeorgetownState {
            murray_last_val: random(),
            lena_last_val: random(),
            f64_data: data_types::Float64 { value: random() },
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Geneva;

#[derive(ZFState, Debug, Clone)]
struct GenevaState {
    danube_last_val: data_types::String,
    parana_last_val: data_types::String,
    tagus_last_val: data_types::Pose,
    congo_last_val: data_types::Twist,
}

impl Operator for Geneva {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        _state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        for token in tokens.values() {
            match token {
                InputToken::Ready(_) => return Ok(true),
                InputToken::Pending => continue,
            }
        }
        Ok(true)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        let my_state = state.try_get::<GenevaState>()?;

        if let Some(danube_value) = inputs.get_mut(crate::DANUBE_PORT) {
            let inner_value = danube_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            my_state.danube_last_val = inner_value.clone();
        }

        if let Some(togus_value) = inputs.get_mut(crate::TAGUS_PORT) {
            let inner_value = togus_value.get_inner_data().try_get::<data_types::Pose>()?;
            my_state.tagus_last_val = inner_value.clone();
        }

        if let Some(congo_value) = inputs.get_mut(crate::CONGO_PORT) {
            let inner_value = congo_value
                .get_inner_data()
                .try_get::<data_types::Twist>()?;
            my_state.congo_last_val = inner_value.clone();
        }

        if let Some(parana_value) = inputs.get_mut(crate::PARANA_PORT) {
            let inner_value = parana_value
                .get_inner_data()
                .try_get::<data_types::String>()?;
            my_state.parana_last_val = inner_value.clone();

            let new_value = data_types::String {
                value: format!(
                    "{}-{}",
                    my_state.parana_last_val.value, my_state.danube_last_val.value
                ),
            };

            let data = Data::from::<data_types::String>(new_value);
            results.insert(crate::ARKANSAS_PORT.into(), data);
        }

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for Geneva {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        let my_state = GenevaState {
            danube_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            parana_last_val: data_types::String {
                value: datatypes::random_string(1),
            },
            tagus_last_val: random(),
            congo_last_val: random(),
        };
        Ok(State::from(my_state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}
