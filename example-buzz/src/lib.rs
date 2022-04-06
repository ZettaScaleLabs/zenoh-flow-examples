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

use std::collections::HashMap;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::runtime::message::DataMessage;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::{
    default_input_rule, default_output_rule, export_operator, types::ZFResult, InputToken, Node,
    NodeOutput, Operator, State,
};
use zenoh_flow::{Configuration, LocalDeadlineMiss};
use zenoh_flow::{Context, Data, ZFError};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct BuzzOperator;

#[derive(Debug, ZFState)]
struct BuzzState {
    buzzword: String,
}

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_INPUT_STR: &str = "Str";
static LINK_ID_OUTPUT_STR: &str = "Str";

impl Operator for BuzzOperator {
    fn input_rule(
        &self,
        _context: &mut Context,
        state: &mut State,
        tokens: &mut HashMap<zenoh_flow::PortId, InputToken>,
    ) -> ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut Context,
        dyn_state: &mut State,
        inputs: &mut HashMap<zenoh_flow::PortId, DataMessage>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results = HashMap::<zenoh_flow::PortId, Data>::with_capacity(1);

        let state = dyn_state.try_get::<BuzzState>()?;

        let mut input_fizz = inputs
            .remove(LINK_ID_INPUT_STR)
            .ok_or_else(|| ZFError::InvalidData("No data".to_string()))?;

        let fizz = input_fizz.get_inner_data().try_get::<ZFString>()?;

        let mut input_value = inputs
            .remove(LINK_ID_INPUT_INT)
            .ok_or_else(|| ZFError::InvalidData("No data".to_string()))?;

        let value = input_value.get_inner_data().try_get::<ZFUsize>()?;

        let mut buzz = fizz.clone();
        if value.0 % 3 == 0 {
            buzz.0.push_str(&state.buzzword);
        }

        results.insert(
            LINK_ID_OUTPUT_STR.into(),
            Data::from::<ZFString>(buzz.clone()),
        );

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut Context,
        state: &mut State,
        outputs: HashMap<zenoh_flow::PortId, Data>,
        _deadlinemiss: Option<LocalDeadlineMiss>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for BuzzOperator {
    fn initialize(&self, configuration: &Option<Configuration>) -> ZFResult<State> {
        let state = match configuration {
            Some(config) => match config["buzzword"].as_str() {
                Some(buzzword) => BuzzState {
                    buzzword: buzzword.to_string(),
                },
                None => BuzzState {
                    buzzword: "Buzz".to_string(),
                },
            },
            None => BuzzState {
                buzzword: "Buzz".to_string(),
            },
        };
        Ok(State::from(state))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(BuzzOperator) as Arc<dyn Operator>)
}
