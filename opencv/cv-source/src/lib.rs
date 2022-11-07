mod config;

use config::{Config, DEFAULT_RESOLUTION};

use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use opencv::{
    self as cv,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use std::time::Duration;
use zenoh_flow::prelude::*;

struct VideoState {
    capture: VideoCapture,
    delay: u64,
    width: i32,
    height: i32,
}

impl VideoState {
    fn new(config: Config) -> Result<Self> {
        let Config {
            resolution,
            path,
            delay,
        } = config;

        let capture = if let Some(p) = &path {
            VideoCapture::from_file(p, CAP_ANY)
        } else {
            VideoCapture::new(0, CAP_ANY)
        }?;
        assert!(VideoCapture::is_opened(&capture)?, "Cannot open {:?}", path);

        let (width, height) = {
            let res: Vec<_> = resolution
                .unwrap_or_else(|| DEFAULT_RESOLUTION.to_string())
                .split('x')
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            assert!(res.len() == 2);
            (res[0], res[1])
        };

        Ok(Self {
            capture,
            delay,
            width,
            height,
        })
    }

    pub fn get_frame(&mut self) -> Result<Vec<u8>> {
        let frame = {
            let mut frame = Mat::default();
            self.capture.read(&mut frame)?;
            frame
        };

        let resized = {
            let mut resized = cv::core::Mat::default();
            cv::imgproc::resize(
                &frame,
                &mut resized,
                cv::core::Size::new(self.width, self.height),
                0.0,
                0.0,
                cv::imgproc::INTER_LINEAR,
            )?;
            resized
        };

        let mut buf = cv::types::VectorOfu8::new();
        let encode_options = opencv::types::VectorOfi32::new();
        opencv::imgcodecs::imencode(".jpg", &resized, &mut buf, &encode_options).unwrap();
        Ok(buf.into())
    }
}

struct VideoSourceFactory;

struct VideoSource {
    output: Output,
    state: Arc<Mutex<VideoState>>,
}

#[async_trait]
impl Node for VideoSource {
    async fn iteration(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        self.output
            .send_async(Data::from(state.get_frame()?), None)
            .await?;
        async_std::task::sleep(Duration::from_millis(state.delay)).await;
        Ok(())
    }
}

#[async_trait]
impl SourceFactoryTrait for VideoSourceFactory {
    async fn new_source(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let config = configuration
            .clone()
            .map_or_else(Config::default, |cfg| serde_json::from_value(cfg).unwrap());

        let output = outputs
            .take("Frame")
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let state = Arc::new(Mutex::new(VideoState::new(config)?));
        Ok(Some(Arc::new(VideoSource { output, state })))
    }
}

export_source_factory!(register);

fn register() -> Result<Arc<dyn SourceFactoryTrait>> {
    Ok(Arc::new(VideoSourceFactory) as Arc<dyn SourceFactoryTrait>)
}
