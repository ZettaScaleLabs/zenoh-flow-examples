include!(concat!(env!("OUT_DIR"), "/ros2.geometry.rs"));

use prost::Message;
use zenoh_flow::prelude::*;

impl ZFData for Vector3 {
    fn try_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.encode_to_vec())
    }

    fn try_deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Message::decode(bytes).unwrap())
    }
}
