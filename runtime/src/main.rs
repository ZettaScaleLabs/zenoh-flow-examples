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

// TODO: this should become a deamon.

use async_ctrlc::CtrlC;
use async_std::prelude::StreamExt;
use clap::Parser;
use std::convert::TryFrom;
use std::fs::{File, *};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use zenoh::prelude::r#async::*;
use zenoh_flow::runtime::dataflow::loader::{Loader, LoaderConfig};
use zenoh_flow::runtime::RuntimeContext;
use zenoh_flow::Result;

#[derive(Debug, Parser)]
#[clap(name = "dpn")]
struct Opt {
    #[clap(short = 'g', long = "graph-file")]
    graph_file: String,
    #[clap(short = 'o', long = "out-file", default_value = "output.dot")]
    _outfile: String,
    #[clap(short = 'l', long = "loader_config")]
    loader_config: Option<String>,
    #[clap(short = 'r', long = "runtime")]
    runtime: String,
    #[clap(short = 'z', long = "zenoh-config")]
    zenoh_config: Option<String>,
}

fn _write_record_to_file(record: zenoh_flow::model::record::DataFlowRecord, filename: &str) {
    let path = Path::new(filename);
    let mut write_file = File::create(path).unwrap();
    write!(write_file, "{}", record.to_yaml().unwrap()).unwrap();
}

fn _write_flatten_to_file(
    descriptor: zenoh_flow::model::descriptor::FlattenDataFlowDescriptor,
    filename: &str,
) {
    let path = Path::new(filename);
    let mut write_file = File::create(path).unwrap();
    write!(write_file, "{}", descriptor.to_yaml().unwrap()).unwrap();
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let opt = Opt::parse();
    let yaml_df = read_to_string(opt.graph_file).unwrap();

    let loader_config = match opt.loader_config {
        Some(config) => {
            let yaml_conf = read_to_string(config).unwrap();
            serde_yaml::from_str::<LoaderConfig>(&yaml_conf).unwrap()
        }
        None => LoaderConfig::new(),
    };

    let z_config = match opt.zenoh_config {
        None => zenoh::config::Config::default(),
        Some(z_config) => get_zenoh_config(&z_config).unwrap(),
    };

    let session = Arc::new(zenoh::open(z_config).res().await.unwrap());

    let hlc = async_std::sync::Arc::new(uhlc::HLC::default());
    let loader = Arc::new(Loader::new(loader_config));

    let ctx = RuntimeContext {
        session,
        hlc,
        loader,
        runtime_name: opt.runtime.clone().into(),
        runtime_uuid: uuid::Uuid::new_v4(),
    };

    // loading the descriptor
    let df = zenoh_flow::model::descriptor::DataFlowDescriptor::from_yaml(&yaml_df).unwrap();

    let df = df.flatten().await.unwrap();

    _write_flatten_to_file(df.clone(), "flattened.yaml");

    // mapping to infrastructure
    let mapped = zenoh_flow::runtime::map_to_infrastructure(df, &opt.runtime)
        .await
        .unwrap();

    // creating record
    let dfr =
        zenoh_flow::model::record::DataFlowRecord::try_from((mapped, uuid::Uuid::nil())).unwrap();

    _write_record_to_file(dfr.clone(), "computed-record.yaml");

    // creating dataflow
    let dataflow = zenoh_flow::runtime::dataflow::Dataflow::try_new(ctx.clone(), dfr).unwrap();

    // instantiating
    let mut instance = zenoh_flow::runtime::dataflow::instance::DataflowInstance::try_instantiate(
        dataflow,
        ctx.hlc.clone(),
    )
    .unwrap();

    let mut sinks = instance.get_sinks();
    for id in sinks.drain(..) {
        instance.start_node(&id).await.unwrap()
    }

    let mut operators = instance.get_operators();
    for id in operators.drain(..) {
        instance.start_node(&id).await.unwrap()
    }

    let mut connectors = instance.get_connectors();
    for id in connectors.drain(..) {
        instance.start_node(&id).await.unwrap()
    }

    let sources = instance.get_sources();
    for id in &sources {
        instance.start_node(id).await.unwrap()
    }

    let ctrlc = CtrlC::new().expect("Unable to create Ctrl-C handler");
    let mut stream = ctrlc.enumerate().take(1);
    stream.next().await;
    log::trace!("Received Ctrl-C start teardown");

    // Stopping nodes
    let sources = instance.get_sources();
    for id in &sources {
        instance.stop_node(id).await.unwrap()
    }

    let mut sinks = instance.get_sinks();
    for id in sinks.drain(..) {
        instance.stop_node(&id).await.unwrap()
    }

    let mut operators = instance.get_operators();
    for id in operators.drain(..) {
        instance.stop_node(&id).await.unwrap()
    }

    let mut connectors = instance.get_connectors();
    for id in connectors.drain(..) {
        instance.stop_node(&id).await.unwrap()
    }

    log::trace!("Bye!");
}

fn get_zenoh_config(path: &str) -> Result<Config> {
    zenoh::config::Config::from_file(path)
}
