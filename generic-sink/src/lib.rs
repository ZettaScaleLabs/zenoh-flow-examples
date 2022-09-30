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

struct GenericSink;

#[async_trait]
impl Sink for GenericSink {
    async fn setup(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut inputs: Inputs,
    ) -> Result<Option<Box<dyn AsyncIteration>>> {
        let file = match configuration {
            Some(c) => {
                let f = File::create(c["file"].as_str().unwrap()).unwrap();
                Some(Arc::new(Mutex::new(f)))
            }
            None => None,
        };

        let input = inputs.take_into_arc("Data").unwrap();

        Ok(Some(Box::new(move || {
            let file = file.clone();
            let input = Arc::clone(&input);
            async move {
                if let Ok(data) = input.recv_async().await {
                    match &file {
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
        })))
    }
}

export_sink!(register);

fn register() -> Result<Arc<dyn Sink>> {
    Ok(Arc::new(GenericSink) as Arc<dyn Sink>)
}
