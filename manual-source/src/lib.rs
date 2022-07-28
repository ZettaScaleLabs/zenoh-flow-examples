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
use std::{sync::Arc, usize};
use zenoh_flow::{AsyncIteration, Configuration, Data, Node, Outputs, Source, ZFError, ZFResult};
use zenoh_flow_example_types::ZFUsize;

struct ManualSource;

#[async_trait]
impl Source for ManualSource {
    async fn setup(
        &self,
        _configuration: &Option<Configuration>,
        outputs: Outputs,
    ) -> Arc<dyn AsyncIteration> {
        let output = outputs.get("Int").unwrap()[0].clone();

        Arc::new(async move || {
            println!("> Please input a number: ");
            let mut number = String::new();
            zenoh_flow::async_std::io::stdin()
                .read_line(&mut number)
                .await
                .expect("Could not read number.");

            let value: usize = match number.trim().parse() {
                Ok(value) => value,
                Err(_) => return Err(ZFError::GenericError),
            };

            output.send(Data::from(ZFUsize(value)), None).await
        })
    }
}

#[async_trait]
impl Node for ManualSource {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(ManualSource) as Arc<dyn Source>)
}
