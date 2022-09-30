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

struct ManualSource;

#[async_trait]
impl Source for ManualSource {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Box<dyn AsyncIteration>>> {
        let output = outputs.take_into_arc("Int").unwrap();

        Ok(Some(Box::new(move || {
            let output = Arc::clone(&output);
            async move {
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

                output.send_async(Data::from(ZFUsize(value)), None).await?;
                async_std::task::sleep(Duration::from_millis(500)).await;
                Ok(())
            }
        })))
    }
}

zenoh_flow::export_source!(register);

fn register() -> Result<Arc<dyn Source>> {
    Ok(Arc::new(ManualSource) as Arc<dyn Source>)
}
