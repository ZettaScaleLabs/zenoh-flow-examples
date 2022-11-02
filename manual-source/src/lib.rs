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
use std::{sync::Arc, time::Duration, usize};
use zenoh_flow::{bail, prelude::*};
use zenoh_flow_example_types::ZFUsize;

struct ManualSource {
    output: Output,
}

#[async_trait]
impl Node for ManualSource {
    async fn iteration(&self) -> Result<()> {
        println!("> Please input a number: ");
        let mut number = String::new();
        async_std::io::stdin()
            .read_line(&mut number)
            .await
            .expect("Could not read number.");

        let value: usize = match number.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                bail!(ErrorKind::InvalidData, "Expected value, found naught");
            }
        };

        self.output
            .send_async(Data::from(ZFUsize(value)), None)
            .await?;
        async_std::task::sleep(Duration::from_millis(500)).await;
        Ok(())
    }
}

struct ManualSourceFactory;

#[async_trait]
impl SourceFactoryTrait for ManualSourceFactory {
    async fn new_source(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let output = outputs
            .take("Int")
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        Ok(Some(Arc::new(ManualSource { output })))
    }
}

export_source_factory!(register);

fn register() -> Result<Arc<dyn SourceFactoryTrait>> {
    Ok(Arc::new(ManualSourceFactory) as Arc<dyn SourceFactoryTrait>)
}
