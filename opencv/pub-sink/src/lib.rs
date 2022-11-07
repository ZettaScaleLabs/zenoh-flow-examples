use async_std::sync::Arc;
use async_trait::async_trait;
use zenoh::prelude::r#async::*;
use zenoh_flow::prelude::*;

mod config;

use config::Config;

struct PubSink<'a> {
    input: Input,
    session: Arc<Session>,
    key_expr: KeyExpr<'a>,
}

#[async_trait]
impl<'a> Node for PubSink<'a> {
    async fn iteration(&self) -> Result<()> {
        if let Ok(msg) = self.input.recv_async().await {
            let data = match msg {
                Message::Data(mut data) => data.get_inner_data().try_as_bytes()?.as_ref().clone(),
                _ => {
                    panic!()
                }
            };
            self.session
                .put(self.key_expr.clone(), data)
                .congestion_control(CongestionControl::Block)
                .res()
                .await?;
        }
        Ok(())
    }
}

struct PubSinkFactory;

#[async_trait]
impl SinkFactoryTrait for PubSinkFactory {
    async fn new_sink(
        &self,
        _context: &mut Context,
        configuration: &Option<Configuration>,
        mut inputs: Inputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let config = configuration
            .clone()
            .map_or_else(Config::default, |cfg| serde_json::from_value(cfg).unwrap());

        let session = Arc::new(zenoh::open(config.zenoh_config).res().await?);
        let input = inputs
            .take("Data")
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        let key_expr = session
            .declare_keyexpr(config.key_expr)
            .res()
            .await?
            .into_owned();

        Ok(Some(Arc::new(PubSink {
            input,
            session,
            key_expr,
        })))
    }
}

export_sink_factory!(register);

fn register() -> Result<Arc<dyn SinkFactoryTrait>> {
    Ok(Arc::new(PubSinkFactory) as Arc<dyn SinkFactoryTrait>)
}
