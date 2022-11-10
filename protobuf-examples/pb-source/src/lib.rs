use async_trait::async_trait;
use async_std::sync::Arc;
use std::time::Duration;
use zenoh_flow::prelude::*;
use protobuf_types::ros2::geometry::Vector3;

struct PbSource {
    output: Output,
}


#[async_trait]
impl Node for PbSource {
    async fn iteration(&self) -> Result<()> {
        self.output
            .send_async(Data::from(Vector3 {
                x: 0.,
                y: 1.,
                z: 2.
            }), None)
            .await?;
        async_std::task::sleep(Duration::from_millis(100)).await;
        Ok(())
    }
}

struct PbSourceFactory;

#[async_trait]
impl SourceFactoryTrait for PbSourceFactory {
    async fn new_source(
        &self,
        _context: &mut Context,
        _configuration: &Option<Configuration>,
        mut outputs: Outputs,
    ) -> Result<Option<Arc<dyn Node>>> {
        let output = outputs
            .take("Frame")
            .ok_or_else(|| zferror!(ErrorKind::NotFound))?;
        Ok(Some(Arc::new(PbSource { output })))
    }
}

export_source_factory!(register);

fn register() -> Result<Arc<dyn SourceFactoryTrait>> {
    Ok(Arc::new(PbSourceFactory) as Arc<dyn SourceFactoryTrait>)
}
