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

struct BuzzOperator {
    word: String,
    input_value: Input,
    input_fizz: Input,
    output_buzz: Output,
}

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_INPUT_STR: &str = "Str";
static LINK_ID_OUTPUT_STR: &str = "Str";

#[async_trait]
impl Node for BuzzOperator {
    async fn iteration(&self) -> Result<()> {
        let value = match self.input_value.recv_async().await.unwrap() {
            Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFUsize>()?.clone(),
            _ => bail!(ErrorKind::InvalidData, "No data"),
        };

        let fizz = match self.input_fizz.recv_async().await.unwrap() {
            Message::Data(mut msg) => msg.get_inner_data().try_get::<ZFString>()?.clone(),
            _ => bail!(ErrorKind::InvalidData, "No data"),
        };

        let mut buzz = fizz.clone();
        if value.0 % 3 == 0 {
            buzz.0.push_str(&self.word);
        }

        self.output_buzz.send_async(Data::from(buzz), None).await?;
        Ok(())
    }
}

struct BuzzOperatorFactory;

#[async_trait]
impl OperatorFactoryTrait for BuzzOperatorFactory {
    async fn new_operator(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let mut word = "Buzz".to_string();
        if let Some(configuration) = configuration {
            if let Some(buzzword) = configuration["buzzword"].as_str() {
                word = buzzword.to_string();
            }
        }
        let input_fizz = inputs
            .take(LINK_ID_INPUT_STR)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let input_value = inputs
            .take(LINK_ID_INPUT_INT)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let output_buzz = outputs
            .take(LINK_ID_OUTPUT_STR)
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        Ok(Some(Arc::new(BuzzOperator {
            word,
            input_value,
            input_fizz,
            output_buzz,
        })))
    }
}

export_operator_factory!(register);

fn register() -> Result<Arc<dyn OperatorFactoryTrait>> {
    Ok(Arc::new(BuzzOperatorFactory) as Arc<dyn OperatorFactoryTrait>)
}
