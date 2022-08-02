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
#![feature(async_closure)]

use async_trait::async_trait;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::AsyncIteration;
use zenoh_flow::Configuration;
use zenoh_flow::Inputs;
use zenoh_flow::Message;
use zenoh_flow::Outputs;
use zenoh_flow::{export_operator, types::ZFResult, Data, Node, Operator, ZFError};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct FizzOperator;

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_STR: &str = "Str";

#[async_trait]
impl Node for FizzOperator {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

#[async_trait]
impl Operator for FizzOperator {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Arc<dyn AsyncIteration> {
        let input_value = inputs.remove(LINK_ID_INPUT_INT).unwrap();
        let output_value = outputs.remove(LINK_ID_OUTPUT_INT).unwrap();
        let output_fizz = outputs.remove(LINK_ID_OUTPUT_STR).unwrap();

        Arc::new(async move || {
            let mut fizz = ZFString::from("");

            let value = match input_value.recv().await.unwrap() {
                (_, Message::Data(mut msg)) => {
                    Ok(msg.get_inner_data().try_get::<ZFUsize>()?.clone())
                }
                (_, _) => Err(ZFError::InvalidData("No data".to_string())),
            }?;

            if value.0 % 2 == 0 {
                fizz = ZFString::from("Fizz");
            }

            output_value.send(Data::from(value), None).await?;
            output_fizz.send(Data::from(fizz), None).await
        })
    }
}

export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(FizzOperator) as Arc<dyn Operator>)
}
