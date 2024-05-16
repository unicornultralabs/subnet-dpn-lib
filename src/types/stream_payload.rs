use dpn_proto::stream_payload::ProtoStreamPayload;
use log::info;
use prost::Message;

#[derive(Debug, Clone)]
pub struct StreamOrigin {
    pub origin_topic: String,
    /// for multiplex purpose, destination may process data sent by origin
    /// returned outcome is sent back along stream_id
    pub stream_id: u64,
}

#[derive(Debug, Clone)]
pub struct StreamPayload {
    pub origin: StreamOrigin,
    pub payload: Vec<u8>,
}

impl StreamPayload {
    pub fn stream_tx_id(&self) -> String {
        format!("{}:{}", self.origin.origin_topic, self.origin.stream_id)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let proto: ProtoStreamPayload = self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        binding.as_slice().to_owned()
    }

    pub fn from_bytes(bz: &[u8]) -> Self {
        let proto = ProtoStreamPayload::decode(bz).expect("decode proto stream payload failed");
        proto.into()
    }

    pub fn print_payload(&self, outgoing: bool) {
        if outgoing {
            info!(
                ">>>|out|>>> origin_topic={} stream_id={} len={}",
                self.origin.origin_topic[0..10].to_string(),
                self.origin.stream_id,
                self.payload.len(),
            );
        } else {
            info!(
                "<<<|inn|<<< origin_topic={} stream_id={} len={}",
                self.origin.origin_topic[0..10].to_string(),
                self.origin.stream_id,
                self.payload.len(),
            );
        }
    }
}

impl Into<ProtoStreamPayload> for StreamPayload {
    fn into(self) -> ProtoStreamPayload {
        ProtoStreamPayload {
            origin_topic: self.origin.origin_topic,
            stream_id: self.origin.stream_id,
            payload: self.payload,
        }
    }
}

impl Into<StreamPayload> for ProtoStreamPayload {
    fn into(self) -> StreamPayload {
        StreamPayload {
            origin: StreamOrigin {
                origin_topic: self.origin_topic,
                stream_id: self.stream_id,
            },
            payload: self.payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let bz: &[u8] = &[
            10, 68, 99, 95, 48, 120, 57, 55, 57, 55, 57, 101, 57, 56, 102, 57, 57, 102, 48, 98, 97,
            50, 102, 98, 54, 49, 98, 53, 99, 102, 48, 48, 102, 53, 53, 99, 48, 102, 51, 51, 100,
            50, 57, 52, 102, 53, 52, 57, 99, 52, 54, 98, 50, 99, 98, 54, 53, 57, 57, 100, 54, 48,
            99, 99, 100, 53, 100, 57, 100, 100, 16, 4, 26, 224, 1, 67, 79, 78, 78, 69, 67, 84, 32,
            118, 110, 101, 120, 112, 114, 101, 115, 115, 46, 110, 101, 116, 58, 52, 52, 51, 32, 72,
            84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32, 118, 110, 101, 120, 112,
            114, 101, 115, 115, 46, 110, 101, 116, 58, 52, 52, 51, 13, 10, 80, 114, 111, 120, 121,
            45, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97,
            108, 105, 118, 101, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 77,
            111, 122, 105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 77, 97, 99, 105, 110, 116, 111,
            115, 104, 59, 32, 73, 110, 116, 101, 108, 32, 77, 97, 99, 32, 79, 83, 32, 88, 32, 49,
            48, 95, 49, 53, 95, 55, 41, 32, 65, 112, 112, 108, 101, 87, 101, 98, 75, 105, 116, 47,
            53, 51, 55, 46, 51, 54, 32, 40, 75, 72, 84, 77, 76, 44, 32, 108, 105, 107, 101, 32, 71,
            101, 99, 107, 111, 41, 32, 67, 104, 114, 111, 109, 101, 47, 49, 50, 52, 46, 48, 46, 48,
            46, 48, 32, 83, 97, 102, 97, 114, 105, 47, 53, 51, 55, 46, 51, 54, 13, 10, 13, 10,
        ];
        let payload = StreamPayload::from_bytes(bz);
        let bz = payload.to_vec();
    }
}
