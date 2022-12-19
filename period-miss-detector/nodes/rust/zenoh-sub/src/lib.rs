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

use flume::Receiver;
use zenoh::{prelude::r#async::*, subscriber::Subscriber};
use zenoh_flow::prelude::*;

#[export_source]
pub struct ZenohSub<'a> {
    output: Output<String>,
    subscriber: Subscriber<'a, Receiver<Sample>>,
}

#[async_trait::async_trait]
impl<'a> Node for ZenohSub<'a> {
    async fn iteration(&self) -> Result<()> {
        let name = self.subscriber.recv_async().await?;
        self.output.send(name.value.to_string(), None).await
    }
}

#[async_trait::async_trait]
impl<'a> Source for ZenohSub<'a> {
    async fn new(
        context: Context,
        _configuration: Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Self> {
        let subscriber = context
            .zenoh_session()
            .declare_subscriber("zf/period-miss-detector")
            .res()
            .await?;

        Ok(ZenohSub {
            output: outputs.take("out").expect("Could not find output 'out'"),
            subscriber,
        })
    }
}
