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
use zenoh_flow::async_std::sync::{Arc, Mutex};
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::Sink;
use zenoh_flow::{export_sink, types::ZFResult, Node};
use zenoh_flow::{AsyncIteration, Configuration, Inputs};

use std::fs::File;
use std::io::Write;

struct GenericSink;

#[derive(ZFState, Clone, Debug)]
struct SinkState {
    pub file: Option<Arc<Mutex<File>>>,
}

impl SinkState {
    pub fn new(configuration: &Option<Configuration>) -> Self {
        let file = match configuration {
            Some(c) => {
                let f = File::create(c["file"].as_str().unwrap()).unwrap();
                Some(Arc::new(Mutex::new(f)))
            }
            None => None,
        };
        Self { file }
    }
}

#[async_trait]
impl Sink for GenericSink {
    async fn setup(
        &self,
        configuration: &Option<Configuration>,
        inputs: Inputs,
    ) -> Arc<dyn AsyncIteration> {
        let state = SinkState::new(configuration);
        let input = inputs.get("Data").unwrap()[0].clone();
        Arc::new(async move || {
            if let Ok(data) = input.recv().await {
                match &state.file {
                    None => {
                        println!("#######");
                        println!("Example Generic Sink Received -> {:?}", data);
                        println!("#######");
                    }
                    Some(f) => {
                        let mut guard = f.lock().await;
                        writeln!(&mut guard, "#######").unwrap();
                        writeln!(&mut guard, "Example Generic Sink Received -> {:?}", data)
                            .unwrap();
                        writeln!(&mut guard, "#######").unwrap();
                        guard.sync_all().unwrap();
                    }
                }
            }
            Ok(())
        })
    }
}

#[async_trait]
impl Node for GenericSink {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

export_sink!(register);

fn register() -> ZFResult<Arc<dyn Sink>> {
    Ok(Arc::new(GenericSink) as Arc<dyn Sink>)
}
