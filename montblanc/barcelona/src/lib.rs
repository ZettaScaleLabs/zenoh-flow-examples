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
use datatypes::{LENA_PORT, MEKONG_PORT};
use futures::prelude::*;
use futures::select;
use prost::Message;
use rand::random;
use zenoh_flow::prelude::*;

#[export_operator]
pub struct Barcelona {
    input_mekong: Input<data_types::TwistWithCovarianceStamped>,
    output_lena: Output<data_types::WrenchStamped>,
}

#[async_trait::async_trait]
impl Operator for Barcelona {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Self> {
        Ok(Self {
            input_mekong: inputs
                .take(MEKONG_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", MEKONG_PORT))
                .typed(|d| Ok(data_types::TwistWithCovarianceStamped::decode(d)?)),
            output_lena: outputs
                .take(LENA_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", LENA_PORT))
                .typed(|buf, v: &data_types::WrenchStamped| {
                    buf.resize(v.encoded_len(), 0);
                    Ok(v.encode(buf)?)
                }),
        })
    }
}

#[async_trait::async_trait]
impl Node for Barcelona {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg  = self.input_mekong.recv().fuse() => {
                if let Ok((msg, _ts)) = msg {
                    if let zenoh_flow::prelude::Message::Data(data) = msg {
                        let value = data_types::WrenchStamped {
                            header: Some(data.header.clone().unwrap_or(random())),
                            wrench: Some(data_types::Wrench {
                                force: data
                                    .twist
                                    .as_ref()
                                    .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                    .twist
                                    .as_ref()
                                    .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                    .linear
                                    .clone(),
                                torque: data
                                    .twist
                                    .as_ref()
                                    .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                    .twist
                                    .as_ref()
                                    .ok_or_else(|| zferror!(ErrorKind::Empty))?
                                    .angular
                                    .clone(),
                            }),
                        };
                        self.output_lena.send(value, None).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
