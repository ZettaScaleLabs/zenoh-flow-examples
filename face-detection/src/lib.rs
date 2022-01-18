use std::collections::HashMap;
use zenoh_flow::async_std::sync::{Arc, Mutex};
use zenoh_flow::{
    default_input_rule, default_output_rule, runtime::message::DataMessage,
    zenoh_flow_derive::ZFState, zf_spin_lock, Data, Node, Operator, PortId, State, ZFError,
    ZFResult,
};
use zenoh_flow::{Configuration, LocalDeadlineMiss};

use opencv::{core, imgproc, objdetect, prelude::*, types};

#[derive(Debug)]
struct FaceDetection;

static INPUT: &str = "Frame";
static OUTPUT: &str = "Frame";

#[derive(ZFState, Clone)]
struct FDState {
    pub face: Arc<Mutex<objdetect::CascadeClassifier>>,
    pub encode_options: Arc<Mutex<opencv::types::VectorOfi32>>,
}

// because of opencv
impl std::fmt::Debug for FDState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FDState:...",)
    }
}

impl FDState {
    fn new(configuration: &Option<Configuration>) -> Self {
        let default_neural_network = &"haarcascades/haarcascade_frontalface_alt.xml".to_owned();
        let neural_network = if let Some(configuration) = configuration {
            configuration["neural-network"]
                .as_str()
                .unwrap_or(default_neural_network)
        } else {
            default_neural_network
        };

        let xml = core::find_file(neural_network, true, false).unwrap();
        let face = objdetect::CascadeClassifier::new(&xml).unwrap();
        let encode_options = opencv::types::VectorOfi32::new();

        Self {
            face: Arc::new(Mutex::new(face)),
            encode_options: Arc::new(Mutex::new(encode_options)),
        }
    }
}

impl Operator for FaceDetection {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<zenoh_flow::PortId, zenoh_flow::InputToken>,
    ) -> ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        dyn_state: &mut State,
        inputs: &mut HashMap<zenoh_flow::PortId, DataMessage>,
    ) -> ZFResult<HashMap<PortId, Data>> {
        let mut results: HashMap<zenoh_flow::PortId, Data> = HashMap::new();

        let state = dyn_state.try_get::<FDState>()?;

        let mut face = zf_spin_lock!(state.face);
        let encode_options = zf_spin_lock!(state.encode_options);

        let mut input_value = inputs
            .remove(INPUT)
            .ok_or_else(|| ZFError::InvalidData("No data".to_string()))?;
        let data = input_value
            .get_inner_data()
            .try_as_bytes()?
            .as_ref()
            .clone();

        // Decode Image
        let mut frame = opencv::imgcodecs::imdecode(
            &opencv::types::VectorOfu8::from_iter(data),
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();

        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
        let mut reduced = Mat::default();
        imgproc::resize(
            &gray,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0,
            },
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR,
        )
        .unwrap();
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30,
            },
            core::Size {
                width: 0,
                height: 0,
            },
        )
        .unwrap();
        for face in faces {
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0f64, 255f64, -1f64, -1f64),
                10,
                1,
                0,
            )
            .unwrap();
        }

        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpg", &frame, &mut buf, &encode_options).unwrap();

        results.insert(OUTPUT.into(), Data::from_bytes(buf.into()));
        drop(face);

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<zenoh_flow::PortId, Data>,
        _deadlinemiss: Option<LocalDeadlineMiss>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, zenoh_flow::NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for FaceDetection {
    fn initialize(&self, configuration: &Option<Configuration>) -> ZFResult<State> {
        Ok(State::from(FDState::new(configuration)))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Also generated by macro
zenoh_flow::export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(FaceDetection) as Arc<dyn Operator>)
}
