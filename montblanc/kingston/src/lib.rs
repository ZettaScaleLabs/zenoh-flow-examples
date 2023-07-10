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

use datatypes::YAMUNA_PORT;
use prost::Message;
use rand::random;
use std::time::Duration;
use zenoh_flow::prelude::*;

#[export_source]
pub struct Kingston {
    output: Output<datatypes::data_types::Vector3>,
}

#[async_trait::async_trait]
impl Node for Kingston {
    async fn iteration(&self) -> Result<()> {
        async_std::task::sleep(Duration::from_millis(100)).await;
        let value: datatypes::data_types::Vector3 = random();
        self.output.send(value, None).await
    }
}

#[async_trait::async_trait]
impl Source for Kingston {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            output: outputs
                .take(YAMUNA_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", YAMUNA_PORT))
                .typed(|buf, v: &datatypes::data_types::Vector3| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
        })
    }
}
