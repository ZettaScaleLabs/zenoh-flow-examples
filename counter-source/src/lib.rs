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
//#![feature(async_closure)]

use async_trait::async_trait;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use zenoh_flow::prelude::*;
use zenoh_flow_example_types::ZFUsize;
use zenoh_flow::types::{Configuration, Context, Data, Outputs, Streams};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct CountSource {
    output: Output,
}

#[async_trait]
impl Node for CountSource {
    async fn iteration(&self) -> Result<()> {
        COUNTER.fetch_add(1, Ordering::AcqRel);
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        self.output.send_async(Data::from(ZFUsize(COUNTER.load(Ordering::Relaxed))), None).await?;
        Ok(())
    }
}

struct CountSourceFactory;

#[async_trait]
impl SourceFactoryTrait for CountSourceFactory {
    async fn new_source(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        if let Some(conf) = configuration {
            let initial = conf["initial"].as_u64().unwrap() as usize;
            COUNTER.store(initial, Ordering::SeqCst);
        }

        Ok(Some(Arc::new(CountSource {
            output: outputs.take("Counter").ok_or_else(|| zferror!(ErrorKind::NotFound))?,
        })))
    }
}


export_source_factory!(register);

fn register() -> Result<Arc<dyn SourceFactoryTrait>> {
   Ok(Arc::new(CountSourceFactory) as Arc<dyn SourceFactoryTrait>)
}
