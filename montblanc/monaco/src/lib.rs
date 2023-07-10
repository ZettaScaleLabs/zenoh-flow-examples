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
use datatypes::{CONGO_PORT, OHIO_PORT};
use futures::prelude::*;
use futures::select;
use prost::Message;
use rand::random;
use zenoh_flow::prelude::*;

#[export_operator]
pub struct Monaco {
    input_congo: Input<data_types::Twist>,
    output_ohio: Output<data_types::Float32>,
}

#[async_trait::async_trait]
impl Operator for Monaco {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_congo: inputs
                .take(CONGO_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", CONGO_PORT))
                .typed(|buf| Ok(data_types::Twist::decode(buf)?)),
            output_ohio: outputs
                .take(OHIO_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", OHIO_PORT))
                .typed(|buf, v: &data_types::Float32| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
        })
    }
}

#[async_trait::async_trait]
impl Node for Monaco {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg  = self.input_congo.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                if let zenoh_flow::prelude::Message::Data(_inner_data) = msg {
                    let value = data_types::Float32 { value: random() };
                    self.output_ohio.send(value, None).await?;
                }
            }}
        }
        Ok(())
    }
}
