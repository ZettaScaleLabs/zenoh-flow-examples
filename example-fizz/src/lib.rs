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
use async_trait::async_trait;
use std::sync::Arc;
use zenoh_flow::zfresult::ZFResult;
use zenoh_flow::{bail, prelude::*};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct FizzOperator;

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_STR: &str = "Str";

#[async_trait]
impl Operator for FizzOperator {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Box<dyn AsyncIteration>>> {
        let input_value = inputs.take_into_arc(LINK_ID_INPUT_INT).unwrap();
        let output_value = outputs.take_into_arc(LINK_ID_OUTPUT_INT).unwrap();
        let output_fizz = outputs.take_into_arc(LINK_ID_OUTPUT_STR).unwrap();

        Ok(Some(Box::new(move || {
            let c_input = Arc::clone(&input_value);
            let c_output_val = Arc::clone(&output_value);
            let c_output_fizz = Arc::clone(&output_fizz);

            async move {
                let mut fizz = ZFString::from("");

                let value = match c_input.recv_async().await.unwrap() {
                    Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFUsize>()?.clone(),
                    _ => bail!(ErrorKind::InvalidData, "No data"),
                };

                if value.0 % 2 == 0 {
                    fizz = ZFString::from("Fizz");
                }

                c_output_val.send_async(Data::from(value), None).await?;
                c_output_fizz.send_async(Data::from(fizz), None).await
            }
        })))
    }
}

export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(FizzOperator) as Arc<dyn Operator>)
}
