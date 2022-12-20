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

use datatypes::GANGES_PORT;
use rand::random;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[export_source]
pub struct Freeport {
    output: Output<datatypes::data_types::Int64>,
}

#[async_trait::async_trait]
impl Node for Freeport {
    async fn iteration(&self) -> Result<()> {
        async_std::task::sleep(Duration::from_millis(50)).await;
        let data: i64 = random::<i64>();
        let value = datatypes::data_types::Int64 { value: data };
        self.output.send(value, None).await
    }
}

#[async_trait::async_trait]
impl Source for Freeport {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            output: outputs
                .take(GANGES_PORT)
                .expect(&format!("No Output called '{}' found", GANGES_PORT)),
        })
    }
}
