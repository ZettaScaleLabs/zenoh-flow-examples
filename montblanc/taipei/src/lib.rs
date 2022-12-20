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

use datatypes::{COLORADO_PORT, COLUMBIA_PORT};
use zenoh_flow::prelude::*;

#[export_operator]
pub struct Taipei {
    input: InputRaw,
    output: OutputRaw,
}

#[async_trait::async_trait]
impl Operator for Taipei {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input: inputs
                .take_raw(COLUMBIA_PORT)
                .expect(&format!("No Input called '{}' found", COLUMBIA_PORT)),
            output: outputs
                .take_raw(COLORADO_PORT)
                .expect(&format!("No Output called '{}' found", COLORADO_PORT)),
        })
    }
}

#[async_trait::async_trait]
impl Node for Taipei {
    async fn iteration(&self) -> Result<()> {
        self.output.forward(self.input.recv().await?).await
    }
}
