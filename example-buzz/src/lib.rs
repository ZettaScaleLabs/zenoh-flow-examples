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
use zenoh_flow::{bail, prelude::*};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct BuzzOperator;

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
    ) -> Result<Option<Box<dyn AsyncIteration>>> {
        let mut word = "Buzz".to_string();
        if let Some(configuration) = configuration {
            if let Some(buzzword) = configuration["buzzword"].as_str() {
                word = buzzword.to_string();
            }
        }

        let input_fizz = inputs.take_into_arc(LINK_ID_INPUT_STR).unwrap();
        let input_value = inputs.take_into_arc(LINK_ID_INPUT_INT).unwrap();
        let output_buzz = outputs.take_into_arc(LINK_ID_OUTPUT_STR).unwrap();

        Ok(Some(Box::new(move || {
            let input_fizz = Arc::clone(&input_fizz);
            let input_value = Arc::clone(&input_value);
            let output_buzz = Arc::clone(&output_buzz);
            let word = word.clone();

            async move {
                let value = match input_value.recv_async().await.unwrap() {
                    Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFUsize>()?.clone(),
                    _ => bail!(ErrorKind::InvalidData, "No data"),
                };

                let fizz = match input_fizz.recv_async().await.unwrap() {
                    Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFString>()?.clone(),
                    _ => bail!(ErrorKind::InvalidData, "No data"),
                };

                let mut buzz = fizz.clone();
                if value.0 % 3 == 0 {
                    buzz.0.push_str(&word);
                }

                output_buzz.send_async(Data::from(buzz), None).await
            }
        })))
    }
}

export_operator!(register);

fn register() -> Result<Arc<dyn Operator>> {
    Ok(Arc::new(BuzzOperator) as Arc<dyn Operator>)
}
