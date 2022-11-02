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
use zenoh_flow::{bail, prelude::*};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct FizzOperator {
    input_value: Input,
    output_value: Output,
    output_fizz: Output,
}

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_INT: &str = "Int";
static LINK_ID_OUTPUT_STR: &str = "Str";

#[async_trait]
impl Node for FizzOperator {
    async fn iteration(&self) -> Result<()> {
        let value = match self.input_value.recv_async().await.unwrap() {
            Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFUsize>()?.clone(),
            _ => bail!(ErrorKind::InvalidData, "No data"),
        };

        let fizz = ZFString::from(if value.0 % 2 == 0 { "Fizz" } else { "" });

        self.output_value
            .send_async(Data::from(value), None)
            .await?;
        self.output_fizz.send_async(Data::from(fizz), None).await?;
        Ok(())
    }
}

struct FizzOperatorFactory;

#[async_trait]
impl OperatorFactoryTrait for FizzOperatorFactory {
    async fn new_operator(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let input_value = inputs
            .take(LINK_ID_INPUT_INT)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let output_value = outputs
            .take(LINK_ID_OUTPUT_INT)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let output_fizz = outputs
            .take(LINK_ID_OUTPUT_STR)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;

        Ok(Some(Arc::new(FizzOperator {
            input_value,
            output_value,
            output_fizz,
        })))
    }
}

export_operator_factory!(register);

fn register() -> Result<Arc<dyn OperatorFactoryTrait>> {
    Ok(Arc::new(FizzOperatorFactory) as Arc<dyn OperatorFactoryTrait>)
}
