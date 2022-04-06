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

use async_trait::async_trait;
use std::sync::atomic::{AtomicUsize, Ordering};
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::Configuration;
use zenoh_flow::{types::ZFResult, zenoh_flow_derive::ZFState, zf_empty_state, Data, State};
use zenoh_flow::{Node, Source};
use zenoh_flow_example_types::ZFUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, ZFState)]
struct CountSource;

#[async_trait]
impl Source for CountSource {
    async fn run(&self, _context: &mut zenoh_flow::Context, _state: &mut State) -> ZFResult<Data> {
        let d = ZFUsize(COUNTER.fetch_add(1, Ordering::AcqRel));
        zenoh_flow::async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok(Data::from::<ZFUsize>(d))
    }
}

impl Node for CountSource {
    fn initialize(&self, configuration: &Option<Configuration>) -> ZFResult<State> {
        if let Some(conf) = configuration {
            let initial = conf["initial"].as_u64().unwrap() as usize;
            COUNTER.store(initial, Ordering::SeqCst);
        }

        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(CountSource) as Arc<dyn Source>)
}
