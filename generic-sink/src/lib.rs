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

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use zenoh_flow::prelude::*;

struct GenericSink {
    input: Input,
    file: Option<Arc<Mutex<File>>>
}

#[async_trait]
impl Node for GenericSink {
    async fn iteration(&self) -> Result<()> {
        if let Ok(data) = self.input.recv_async().await {
            match &self.file {
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
    }

}

struct GenericSinkFactory;

#[async_trait]
impl SinkFactoryTrait for GenericSinkFactory {
    async fn new_sink(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut inputs: Inputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let file = match configuration {
            Some(c) => {
                let f = File::create(c["file"].as_str().unwrap()).unwrap();
                Some(Arc::new(Mutex::new(f)))
            }
            None => None,
        };
        Ok(Some(Arc::new(GenericSink {
            input: inputs.take("Data").ok_or_else(|| zferror!(ErrorKind::NotFound))?,
            file
        })))
    }
}


export_sink_factory!(register);

fn register() -> Result<Arc<dyn SinkFactoryTrait>> {
   Ok(Arc::new(GenericSinkFactory) as Arc<dyn SinkFactoryTrait>)
}
