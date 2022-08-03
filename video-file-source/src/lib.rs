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
use opencv::{core, prelude::*, videoio};
use zenoh_flow::async_std::sync::{Arc, Mutex};
use zenoh_flow::{
    types::{ZFError, ZFResult},
    zenoh_flow_derive::ZFState,
    zf_spin_lock, Data, Node, Source,
};
use zenoh_flow::{AsyncIteration, Configuration, Outputs};

#[derive(Debug)]
struct VideoSource;

#[derive(ZFState, Clone)]
struct VideoSourceState {
    pub camera: Arc<Mutex<videoio::VideoCapture>>,
    pub encode_options: Arc<Mutex<opencv::types::VectorOfi32>>,
    pub delay: u64,
    pub source_file: String,
}

// because of opencv
impl std::fmt::Debug for VideoSourceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "VideoState: file:{:?} delay:{:?}",
            self.source_file, self.delay
        )
    }
}

impl VideoSourceState {
    fn new(configuration: &Option<Configuration>) -> Self {
        // Configuration is mandatory
        let configuration = configuration.as_ref().unwrap();

        let source_file = configuration["file"].as_str().unwrap();
        let delay = match configuration["fps"].as_f64() {
            Some(fps) => {
                let delay: f64 = 1f64 / fps;
                (delay * 1000f64) as u64
            }
            None => 40,
        };

        let camera = videoio::VideoCapture::from_file(source_file, videoio::CAP_ANY).unwrap(); // 0 is the default camera
        let opened = videoio::VideoCapture::is_opened(&camera).unwrap();
        if !opened {
            panic!("Unable to open default camera!");
        }
        Self {
            camera: Arc::new(Mutex::new(camera)),
            encode_options: Arc::new(Mutex::new(opencv::types::VectorOfi32::new())),
            source_file: source_file.to_string(),
            delay,
        }
    }

    pub fn capture(&self) -> ZFResult<Vec<u8>> {
        let mut cam = zf_spin_lock!(self.camera);
        let encode_options = zf_spin_lock!(self.encode_options);

        let mut frame = core::Mat::default();
        match cam.read(&mut frame) {
            Ok(false) => {
                *cam =
                    videoio::VideoCapture::from_file(&self.source_file, videoio::CAP_ANY).unwrap(); // 0 is the default camera
                let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
                if !opened {
                    panic!("Unable to open default camera!");
                }
                cam.read(&mut frame)
                    .map_err(|e| ZFError::IOError(format!("{}", e)))?;
            }
            Ok(true) => (),
            Err(_) => {
                *cam =
                    videoio::VideoCapture::from_file(&self.source_file, videoio::CAP_ANY).unwrap(); // 0 is the default camera
                let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
                if !opened {
                    panic!("Unable to open default camera!");
                }
                cam.read(&mut frame)
                    .map_err(|e| ZFError::IOError(format!("{}", e)))?;
            }
        };

        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpg", &frame, &mut buf, &encode_options).unwrap();
        Ok(buf.into())
    }
}
#[async_trait]
impl Node for VideoSource {
    async fn finalize(&self) -> ZFResult<()> {
        Ok(())
    }
}

#[async_trait]
impl Source for VideoSource {
    async fn setup(
        &self,
        configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> ZFResult<Arc<dyn AsyncIteration>> {
        let state = VideoSourceState::new(configuration);

        let output = outputs.remove("Frame").unwrap();

        Ok(Arc::new(async move || {
            zenoh_flow::async_std::task::sleep(std::time::Duration::from_millis(state.delay)).await;
            let frame = state.capture()?;
            output.send(Data::from_bytes(frame), None).await
        }))
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(VideoSource) as Arc<dyn Source>)
}
