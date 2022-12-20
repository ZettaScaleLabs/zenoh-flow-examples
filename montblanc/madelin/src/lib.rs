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

use datatypes::NILE_PORT;
use rand::random;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[export_source]
pub struct Madelin {
    output: Output<datatypes::data_types::Int32>,
}

#[async_trait::async_trait]
impl Node for Madelin {
    async fn iteration(&self) -> Result<()> {
        async_std::task::sleep(Duration::from_millis(10)).await;
        let data: i32 = random::<i32>();
        let value = datatypes::data_types::Int32 { value: data };
        self.output.send(value, None).await
    }
}

#[async_trait::async_trait]
impl Source for Madelin {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            output: outputs
                .take(NILE_PORT)
                .expect(&format!("No Output called '{}' found", NILE_PORT)),
        })
    }
}
