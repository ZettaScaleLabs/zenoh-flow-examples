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
use opencv::core;
use zenoh_flow::async_std::sync::{Arc, Mutex};
use zenoh_flow::{
    zenoh_flow_derive::ZFState, zf_spin_lock, Data, Node, Operator, Streams, ZFError, ZFResult,
};
use zenoh_flow::{AsyncIteration, Configuration, Context, Inputs, Message, Outputs};

static INPUT1: &str = "Frame1";
static INPUT2: &str = "Frame2";
static OUTPUT: &str = "Frame";

#[derive(ZFState, Clone)]
struct FrameConcatState {
    pub encode_options: Arc<Mutex<opencv::types::VectorOfi32>>,
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
            encode_options: Arc::new(Mutex::new(opencv::types::VectorOfi32::new())),
        }
    }

    fn concat(&self, top: Vec<u8>, bottom: Vec<u8>) -> Vec<u8> {
        let encode_options = zf_spin_lock!(self.encode_options);

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
        opencv::imgcodecs::imencode(".jpg", &frame, &mut buf, &encode_options).unwrap();

        buf.into()
    }
}

struct FrameConcat;

#[async_trait]
impl Node for FrameConcat {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

#[async_trait]
impl Operator for FrameConcat {
    async fn setup(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut inputs: Inputs,
        mut outputs: Outputs,
    ) -> ZFResult<Option<Arc<dyn AsyncIteration>>> {
        let state = FrameConcatState::new();

        let input_top = inputs.take(INPUT1).unwrap();
        let input_bottom = inputs.take(INPUT2).unwrap();
        let output_frame = outputs.take(OUTPUT).unwrap();

        Ok(Some(Arc::new(move || async move {
            let top = match input_top.recv_async().await.unwrap() {
                Message::Data(mut msg) => Ok(msg.get_inner_data().try_as_bytes()?.as_ref().clone()),
                _ => Err(ZFError::InvalidData("No data".to_string())),
            }?;

            let bottom = match input_bottom.recv_async().await.unwrap() {
                Message::Data(mut msg) => Ok(msg.get_inner_data().try_as_bytes()?.as_ref().clone()),
                _ => Err(ZFError::InvalidData("No data".to_string())),
            }?;

            let frame = state.concat(top, bottom);
            output_frame.send_async(Data::from_bytes(frame), None).await
        })))
    }
}

zenoh_flow::export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(FrameConcat) as Arc<dyn Operator>)
}
