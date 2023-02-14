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

use datatypes::DANUBE_PORT;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[export_source]
pub struct Portsmouth {
    output: Output<datatypes::data_types::String>,
}

#[async_trait::async_trait]
impl Node for Portsmouth {
    async fn iteration(&self) -> Result<()> {
        async_std::task::sleep(Duration::from_millis(200)).await;
        let value = datatypes::data_types::String {
            value: "portsmouth/danube".into(),
        };
        self.output.send(value, None).await
    }
}

#[async_trait::async_trait]
impl Source for Portsmouth {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            output: outputs
                .take(DANUBE_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", DANUBE_PORT)),
        })
    }
}
