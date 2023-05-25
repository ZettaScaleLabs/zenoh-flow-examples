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

use datatypes::{AMAZON_PORT, TIGRIS_PORT};
use zenoh_flow::prelude::*;

#[export_operator]
pub struct Lyon {
    input: InputRaw,
    output: OutputRaw,
}

#[async_trait::async_trait]
impl Operator for Lyon {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input: inputs
                .take(AMAZON_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", AMAZON_PORT))
                .raw(),
            output: outputs
                .take(TIGRIS_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", TIGRIS_PORT))
                .raw(),
        })
    }
}

#[async_trait::async_trait]
impl Node for Lyon {
    async fn iteration(&self) -> Result<()> {
        self.output.forward(self.input.recv().await?).await
    }
}
