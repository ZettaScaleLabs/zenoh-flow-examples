//
// Copyright (c) 2017, 2021 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//

use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc, usize};
use zenoh_flow::{
    zf_data, zf_empty_state, Context, Node, SerDeData, Source, State, ZFError, ZFResult,
};
use zenoh_flow_example_types::ZFUsize;

struct ManualSource;

#[async_trait]
impl Source for ManualSource {
    async fn run(
        &self,
        _context: &mut Context,
        _state: &mut Box<dyn State>,
    ) -> ZFResult<SerDeData> {
        println!("> Please input a number: ");
        let mut number = String::new();
        zenoh_flow::async_std::io::stdin()
            .read_line(&mut number)
            .await
            .expect("Could not read number.");

        let value: usize = match number.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err(ZFError::GenericError),
        };

        Ok(zf_data!(ZFUsize(value)))
    }
}

impl Node for ManualSource {
    fn initialize(&self, _configuration: &Option<HashMap<String, String>>) -> Box<dyn State> {
        zf_empty_state!()
    }

    fn clean(&self, _state: &mut Box<dyn State>) -> ZFResult<()> {
        Ok(())
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(ManualSource) as Arc<dyn Source>)
}
