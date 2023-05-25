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

use datatypes::data_types::{self, Vector3Stamped};
use datatypes::{MEKONG_PORT, MURRAY_PORT};
use futures::prelude::*;
use futures::select;
use prost::Message as pMessage;
use rand::random;
use zenoh_flow::{anyhow, prelude::*};

#[export_operator]
pub struct Rotterdam {
    input_mekong: Input<data_types::TwistWithCovarianceStamped>,
    output_murray: Output<data_types::Vector3Stamped>,
}

#[async_trait::async_trait]
impl Operator for Rotterdam {
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
                .typed(|bytes| {
                    data_types::TwistWithCovarianceStamped::decode(bytes).map_err(|e| anyhow!(e))
                }),
            output_murray: outputs
                .take(MURRAY_PORT)
                .unwrap_or_else(|| panic!("No Output called '{}' found", MURRAY_PORT))
                .typed(|buffer, data: &Vector3Stamped| data.encode(buffer).map_err(|e| anyhow!(e))),
        })
    }
}

#[async_trait::async_trait]
impl Node for Rotterdam {
    async fn iteration(&self) -> Result<()> {
        select! {
            msg  = self.input_mekong.recv().fuse() => {
                if let Ok((Message::Data(inner_data),_)) = msg {
                    let value = data_types::Vector3Stamped {
                        header: Some(inner_data.header.clone().unwrap_or(random())),
                        vector: inner_data
                            .twist
                            .as_ref()
                            .ok_or_else(|| zferror!(ErrorKind::Empty))?
                            .twist
                            .as_ref()
                            .ok_or_else(|| zferror!(ErrorKind::Empty))?
                            .linear
                            .clone(),
                    };
                    self.output_murray.send(value, None).await?;
                }
            }
        }
        Ok(())
    }
}
