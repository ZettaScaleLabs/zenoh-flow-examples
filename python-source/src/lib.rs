use async_trait::async_trait;
use pyo3::types::{PyDict, PyList}; //, PyBool, PyString};
use pyo3::{prelude::*, types::PyModule};
use std::fs;
use std::path::Path;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::Configuration;
use zenoh_flow::{Data, Node, Source, State, ZFError, ZFResult};
use zenoh_flow_example_types::ZFUsize;

#[derive(ZFState, Clone)]
struct PythonState {
    pub module: Arc<Py<PyModule>>,
    pub py_state: Arc<Py<PyAny>>,
}
unsafe impl Send for PythonState {}
unsafe impl Sync for PythonState {}

impl std::fmt::Debug for PythonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PythonState").finish()
    }
}

#[derive(Debug)]
struct PythonSource;

#[async_trait]
impl Source for PythonSource {
    async fn run(&self, _context: &mut zenoh_flow::Context, state: &mut State) -> ZFResult<Data> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let current_state = state.try_get::<PythonState>()?;

        let source_class = current_state
            .module
            .getattr(py, "PySource")
            .map_err(|e| ZFError::InvalidData(e.to_string()))?;

        let value: usize = source_class
            .call_method(
                py,
                "run",
                (
                    source_class.clone(),
                    "",
                    current_state.py_state.as_ref().clone(),
                ),
                None,
            )
            .map_err(|e| ZFError::InvalidData(e.to_string()))?
            .extract(py)
            .map_err(|e| ZFError::InvalidData(e.to_string()))?;
        Ok(Data::from::<ZFUsize>(ZFUsize(value)))
    }
}

impl Node for PythonSource {
    fn initialize(&self, configuration: &Option<Configuration>) -> ZFResult<State> {
        pyo3::prepare_freethreaded_python();
        let gil = Python::acquire_gil();
        let py = gil.python();
        match configuration {
            Some(configuration) => {
                let script_file_path = Path::new(
                    configuration["python-script"]
                        .as_str()
                        .ok_or(ZFError::InvalidState)?,
                );
                let mut config = configuration.clone();
                config["python-script"].take();

                let py_config = into_py(py, config);

                let code = read_file(script_file_path);
                let module = PyModule::from_code(py, &code, "source.py", "source")
                    .map_err(|_| ZFError::InvalidState)?;

                let node_class = module.getattr("Node").map_err(|_| ZFError::InvalidState)?;
                let state: Py<PyAny> = node_class
                    .call_method1("initialize", (node_class, py_config))
                    .map_err(|_| ZFError::InvalidState)?
                    .into();

                Ok(State::from(PythonState {
                    module: Arc::new(module.into()),
                    py_state: Arc::new(state),
                }))
            }
            None => Err(ZFError::InvalidState),
        }
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Also generated by macro
zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(PythonSource) as Arc<dyn Source>)
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn into_py(py: Python, value: serde_json::Value) -> PyObject {
    match value {
        serde_json::Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for v in arr {
                py_list.append(into_py(py, v)).unwrap();
            }
            py_list.to_object(py)
        }
        serde_json::Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                py_dict.set_item(k, into_py(py, v)).unwrap();
            }
            py_dict.to_object(py)
        }
        // serde_json::Value::Bool(b) => PyBool::new(py,b).to_object(py),
        // serde_json::Value::Number(n) => n.as_u64().unwrap().to_object(py),
        // serde_json::Value::String(s) => PyString::new(py,&s).to_object(py),
        // serde_json::Value::Null => py.None(),
        serde_json::Value::Bool(b) => b.to_object(py),
        serde_json::Value::Number(n) => n.as_u64().unwrap().to_object(py),
        serde_json::Value::String(s) => s.to_object(py),
        serde_json::Value::Null => py.None(),
    }
}
