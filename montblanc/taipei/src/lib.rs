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

use datatypes::data_types;
use datatypes::{COLORADO_PORT, COLUMBIA_PORT};
use prost::Message;
use zenoh_flow::prelude::*;

#[export_operator]
pub struct Taipei {
    input: Input<data_types::Image>,
    output: Output<data_types::Image>,
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
                .take(COLUMBIA_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", COLUMBIA_PORT))
                .typed(|buf| Ok(data_types::Image::decode(buf)?)),
            output: outputs
                .take(COLORADO_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", COLORADO_PORT))
                .typed(|buf, v: &data_types::Image| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
        })
    }
}

#[async_trait::async_trait]
impl Node for Taipei {
    async fn iteration(&self) -> Result<()> {
        let (msg, _ts) = self.input.recv().await?;
        if let zenoh_flow::prelude::Message::Data(data) = msg {
            self.output.send(data, None).await?;
        }
        Ok(())
    }
}
