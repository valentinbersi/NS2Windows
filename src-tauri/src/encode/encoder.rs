use crate::data::output_data::OutputData;
use crate::encode::ds4_encoder::Ds4Encoder;
use crate::encode::xbox_encoder::XboxEncoder;
use vigem_rust::controller::ds4::Ds4ReportEx;
use vigem_rust::X360Report;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Encoder {
    ds4_encoder: Ds4Encoder,
    xbox_encoder: XboxEncoder,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            ds4_encoder: Ds4Encoder,
            xbox_encoder: XboxEncoder,
        }
    }

    pub fn encode_xbox(&self, data: &OutputData) -> X360Report {
        self.xbox_encoder.encode(data)
    }

    pub fn encode_ps4(&self, data: &OutputData) -> Ds4ReportEx {
        self.ds4_encoder.encode(data)
    }
}
