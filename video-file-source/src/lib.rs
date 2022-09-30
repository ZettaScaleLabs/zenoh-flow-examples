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

use async_std::sync::Mutex;
use async_trait::async_trait;
use opencv::{core, prelude::*, videoio};
use std::sync::Arc;
use zenoh_flow::prelude::*;

#[derive(Debug)]
struct VideoSource;

struct VideoSourceState {
    pub camera: videoio::VideoCapture,
    pub encode_options: opencv::types::VectorOfi32,
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
            camera,
            encode_options: opencv::types::VectorOfi32::new(),
            source_file: source_file.to_string(),
            delay,
        }
    }

    pub fn capture(&mut self) -> Result<Vec<u8>> {
        let mut frame = core::Mat::default();
        match self.camera.read(&mut frame) {
            Ok(false) => {
                self.camera =
                    videoio::VideoCapture::from_file(&self.source_file, videoio::CAP_ANY).unwrap(); // 0 is the default camera
                let opened = videoio::VideoCapture::is_opened(&self.camera).unwrap();
                if !opened {
                    panic!("Unable to open default camera!");
                }
                self.camera
                    .read(&mut frame)
                    .map_err(|e| zferror!(ErrorKind::IOError, "{}", e))?;
            }
            Ok(true) => (),
            Err(_) => {
                self.camera =
                    videoio::VideoCapture::from_file(&self.source_file, videoio::CAP_ANY).unwrap(); // 0 is the default camera
                let opened = videoio::VideoCapture::is_opened(&self.camera).unwrap();
                if !opened {
                    panic!("Unable to open default camera!");
                }
                self.camera
                    .read(&mut frame)
                    .map_err(|e| zferror!(ErrorKind::IOError, "{}", e))?;
            }
        };

        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpg", &frame, &mut buf, &self.encode_options).unwrap();
        Ok(buf.into())
    }
}

#[async_trait]
impl Source for VideoSource {
    async fn setup(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Box<dyn AsyncIteration>>> {
        let state = Arc::new(Mutex::new(VideoSourceState::new(configuration)));

        let output = outputs.take_into_arc("Frame").unwrap();

        Ok(Some(Box::new(move || {
            let state = state.clone();
            let output = output.clone();

            async move {
                let delay: u64;
                let frame: Vec<u8>;
                {
                    let mut state = state.lock().await;
                    delay = state.delay;
                    frame = state.capture()?;
                }

                output.send_async(Data::from(frame), None).await?;
                async_std::task::sleep(std::time::Duration::from_millis(delay)).await;
                Ok(())
            }
        })))
    }
}

zenoh_flow::export_source!(register);

fn register() -> Result<Arc<dyn Source>> {
    Ok(Arc::new(VideoSource) as Arc<dyn Source>)
}
