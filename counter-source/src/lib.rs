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
#![feature(async_closure)]

use async_trait::async_trait;
use std::sync::atomic::{AtomicUsize, Ordering};
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::{types::ZFResult, zenoh_flow_derive::ZFState, Data};
use zenoh_flow::{AsyncIteration, Configuration, Outputs};
use zenoh_flow::{Node, Source};
use zenoh_flow_example_types::ZFUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, ZFState)]
struct CountSource;

#[async_trait]
impl Source for CountSource {
    async fn setup(
        &self,
        configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        if let Some(conf) = configuration {
            let initial = conf["initial"].as_u64().unwrap() as usize;
            COUNTER.store(initial, Ordering::SeqCst);
        }

        let output = outputs.remove("Counter").unwrap();

        Ok(Arc::new(async move || {
            zenoh_flow::async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let d = Data::from(ZFUsize(COUNTER.fetch_add(1, Ordering::AcqRel)));
            output.send(d, Some(0u64)).await.unwrap();

            Ok(())
        }))
    }
}

#[async_trait]
impl Node for CountSource {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(CountSource) as Arc<dyn Source>)
}
