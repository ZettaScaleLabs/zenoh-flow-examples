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
//#![feature(async_closure)]

use async_trait::async_trait;
use std::sync::Arc;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::{export_operator, types::ZFResult, Node, Operator, Streams};
use zenoh_flow::{AsyncIteration, Configuration, Inputs, Message, Outputs};
use zenoh_flow::{Context, Data, ZFError};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct BuzzOperator;

#[derive(Debug, ZFState, Clone)]
struct BuzzState {
    buzzword: String,
}

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_INPUT_STR: &str = "Str";
static LINK_ID_OUTPUT_STR: &str = "Str";

#[async_trait]
impl Operator for BuzzOperator {
    async fn setup(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Arc<dyn AsyncIteration>>> {
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

        let input_fizz = inputs.take(LINK_ID_INPUT_STR).unwrap();
        let input_value = inputs.take(LINK_ID_INPUT_INT).unwrap();
        let output_buzz = outputs.take(LINK_ID_OUTPUT_STR).unwrap();

        Ok(Some(Arc::new(move || async move {
            let value = match input_value.recv_async().await.unwrap() {
                Message::Data(mut msg) => Ok(msg.get_inner_data().try_get::<ZFUsize>()?.clone()),
                _ => Err(ZFError::InvalidData("No data".to_string())),
            }?;

            let fizz = match input_fizz.recv_async().await.unwrap() {
                Message::Data(mut msg) => Ok(msg.get_inner_data().try_get::<ZFString>()?.clone()),
                _ => Err(ZFError::InvalidData("No data".to_string())),
            }?;

            let mut buzz = fizz.clone();
            if value.0 % 3 == 0 {
                buzz.0.push_str(&state.buzzword);
            }

            output_buzz.send_async(Data::from(buzz), None).await
        })))
    }
}

#[async_trait]
impl Node for BuzzOperator {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(BuzzOperator) as Arc<dyn Operator>)
}
