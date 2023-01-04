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

use async_std::{fs::File, io::WriteExt, sync::Mutex};
use datatypes::ARKANSAS_PORT;
use zenoh_flow::prelude::*;

static OUT_FILE: &str = "/tmp/montblanc.out";

#[export_sink]
pub struct Arequipa {
    input: Input<datatypes::data_types::String>,
    file: Mutex<File>,
}

#[async_trait::async_trait]
impl Node for Arequipa {
    async fn iteration(&self) -> Result<()> {
        let (message, _) = self.input.recv().await?;

        if let Message::Data(data) = message {
            let mut file = self.file.lock().await;
            file.write_all(data.value.as_bytes())
                .await
                .map_err(|e| zferror!(ErrorKind::IOError, "{:?}", e))?;
            return file
                .flush()
                .await
                .map_err(|e| zferror!(ErrorKind::IOError, "{:?}", e).into());
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Sink for Arequipa {
    async fn new(
        _context: Context,
        _configuration: Option<Configuration>,
        mut inputs: Inputs,
    ) -> Result<Self> {
        Ok(Self {
            input: inputs
                .take(ARKANSAS_PORT)
                .unwrap_or_else(|| panic!("No Input called '{}' found", ARKANSAS_PORT)),
            file: Mutex::new(
                File::create(OUT_FILE)
                    .await
                    .unwrap_or_else(|_| panic!("Could not create {}", OUT_FILE)),
            ),
        })
    }
}
