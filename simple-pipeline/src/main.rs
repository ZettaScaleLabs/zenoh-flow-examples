//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale zenoh team, <zenoh@zettascale.tech>
//

use async_ctrlc::CtrlC;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use zenoh_flow::async_std::stream::StreamExt;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::model::{InputDescriptor, OutputDescriptor};
use zenoh_flow::runtime::dataflow::instance::DataflowInstance;
use zenoh_flow::runtime::dataflow::loader::{Loader, LoaderConfig};
use zenoh_flow::runtime::RuntimeContext;
use zenoh_flow::Configuration;
use zenoh_flow::{model::link::PortDescriptor, zf_empty_state};
use zenoh_flow::{Context, Data, Node, Sink, Source};
use zenoh_flow::{State, ZFResult};
use zenoh_flow_example_types::ZFUsize;

static SOURCE: &str = "Counter";

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct CountSource;

impl CountSource {
    fn new(configuration: Option<HashMap<String, String>>) -> Self {
        match configuration {
            Some(conf) => {
                let initial = conf.get("initial").unwrap().parse::<usize>().unwrap();
                COUNTER.store(initial, Ordering::SeqCst);
                CountSource {}
            }
            None => CountSource {},
        }
    }
}

#[async_trait]
impl Source for CountSource {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> zenoh_flow::ZFResult<Data> {
        let d = ZFUsize(COUNTER.fetch_add(1, Ordering::AcqRel));
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok(Data::from::<ZFUsize>(d))
    }
}

impl Node for CountSource {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

struct ExampleGenericSink;

#[async_trait]
impl Sink for ExampleGenericSink {
    async fn run(
        &self,
        _context: &mut Context,
        _state: &mut State,
        input: zenoh_flow::runtime::message::DataMessage,
    ) -> zenoh_flow::ZFResult<()> {
        println!("Example Generic Sink Received: {:?}", input);
        Ok(())
    }
}

impl Node for ExampleGenericSink {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let session = Arc::new(zenoh::open(zenoh::config::Config::default()).await.unwrap());
    let hlc = async_std::sync::Arc::new(uhlc::HLC::default());

    let ctx = RuntimeContext {
        session,
        hlc,
        loader: Arc::new(Loader::new(LoaderConfig::new())),
        runtime_name: String::from("local").into(),
        runtime_uuid: uuid::Uuid::new_v4(),
    };

    let mut zf_graph =
        zenoh_flow::runtime::dataflow::Dataflow::new(ctx.clone(), "simple-pipeline".into(), None);

    let source = Arc::new(CountSource::new(None));
    let sink = Arc::new(ExampleGenericSink {});

    zf_graph
        .try_add_static_source(
            "counter-source".into(),
            None,
            PortDescriptor {
                port_id: String::from(SOURCE).into(),
                port_type: String::from("int").into(),
            },
            source.initialize(&None).unwrap(),
            source,
        )
        .unwrap();

    zf_graph
        .try_add_static_sink(
            "generic-sink".into(),
            PortDescriptor {
                port_id: String::from(SOURCE).into(),
                port_type: String::from("int").into(),
            },
            sink.initialize(&None).unwrap(),
            sink,
        )
        .unwrap();

    zf_graph
        .try_add_link(
            OutputDescriptor {
                node: "counter-source".into(),
                output: String::from(SOURCE).into(),
            },
            InputDescriptor {
                node: "generic-sink".into(),
                input: String::from(SOURCE).into(),
            },
            None,
            None,
            None,
        )
        .unwrap();

    let mut instance = DataflowInstance::try_instantiate(zf_graph).unwrap();

    let nodes = instance.get_nodes();
    for id in &nodes {
        instance.start_node(id).await.unwrap()
    }

    let ctrlc = CtrlC::new().expect("Unable to create Ctrl-C handler");
    let mut stream = ctrlc.enumerate().take(1);
    stream.next().await;
    println!("Received Ctrl-C start teardown");

    for id in nodes {
        instance.stop_node(&id).await.unwrap()
    }
}
