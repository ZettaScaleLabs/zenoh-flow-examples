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
use opencv::core;
use std::sync::Arc;
use zenoh_flow::{bail, prelude::*, zfresult::ZFError};

static INPUT1: &str = "Frame1";
static INPUT2: &str = "Frame2";
static OUTPUT: &str = "Frame";

struct FrameConcatState {
    pub encode_options: opencv::types::VectorOfi32,
}

// because of opencv
impl std::fmt::Debug for FrameConcatState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ConcatState:...",)
    }
}

impl FrameConcatState {
    fn new() -> Self {
        Self {
            encode_options: opencv::types::VectorOfi32::new(),
        }
    }

    fn concat(&mut self, top: Vec<u8>, bottom: Vec<u8>) -> Vec<u8> {
        // Decode Image
        let frame1 = opencv::imgcodecs::imdecode(
            &opencv::types::VectorOfu8::from_iter(top),
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();

        let frame2 = opencv::imgcodecs::imdecode(
            &opencv::types::VectorOfu8::from_iter(bottom),
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();

        let mut frame = core::Mat::default();

        // concat frames
        core::vconcat2(&frame1, &frame2, &mut frame).unwrap();

        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpg", &frame, &mut buf, &self.encode_options).unwrap();

        buf.into()
    }
}

struct FrameConcat;

#[async_trait]
impl Operator for FrameConcat {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> Result<Option<Box<dyn AsyncIteration>>> {
        let state = Arc::new(Mutex::new(FrameConcatState::new()));

        let input_top = inputs.take_into_arc(INPUT1).unwrap();
        let input_bottom = inputs.take_into_arc(INPUT2).unwrap();
        let output_frame = outputs.take_into_arc(OUTPUT).unwrap();

        Ok(Some(Box::new(move || {
            let state = state.clone();

            let input_top = input_top.clone();
            let input_bottom = input_bottom.clone();
            let output_frame = output_frame.clone();

            async move {
                let top = match input_top.recv_async().await.unwrap() {
                    Message::Data(mut msg) => Ok::<Vec<u8>, ZFError>(
                        msg.get_inner_data().try_as_bytes()?.as_ref().clone(),
                    ),
                    _ => bail!(ErrorKind::InvalidData, "No data"),
                }?;

                let bottom = match input_bottom.recv_async().await.unwrap() {
                    Message::Data(mut msg) => Ok::<Vec<u8>, ZFError>(
                        msg.get_inner_data().try_as_bytes()?.as_ref().clone(),
                    ),
                    _ => bail!(ErrorKind::InvalidData, "No data"),
                }?;
                let buffer: Vec<u8>;

                {
                    buffer = state.lock().await.concat(top, bottom);
                }

                output_frame.send_async(Data::from(buffer), None).await
            }
        })))
    }
}

zenoh_flow::export_operator!(register);

fn register() -> Result<Arc<dyn Operator>> {
    Ok(Arc::new(FrameConcat) as Arc<dyn Operator>)
}
