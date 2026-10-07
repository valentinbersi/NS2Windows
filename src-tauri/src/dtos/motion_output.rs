use crate::data::profile_kind::ProfileKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MotionOutput {
    ViGEm,
    CemuHook,
}

impl MotionOutput {
    pub fn resolve(selection: Option<Self>, kind: ProfileKind) -> Result<Self, String> {
        let output = selection.unwrap_or(match kind {
            ProfileKind::Ps4 => Self::ViGEm,
            ProfileKind::Xbox360 => Self::CemuHook,
        });
        if kind == ProfileKind::Xbox360 && output == Self::ViGEm {
            return Err("Xbox 360 profiles require CemuHook for motion output.".into());
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_xbox_enforcement() {
        assert_eq!(
            MotionOutput::resolve(None, ProfileKind::Ps4),
            Ok(MotionOutput::ViGEm)
        );
        assert_eq!(
            MotionOutput::resolve(None, ProfileKind::Xbox360),
            Ok(MotionOutput::CemuHook)
        );
        assert!(MotionOutput::resolve(Some(MotionOutput::ViGEm), ProfileKind::Xbox360).is_err());
        assert_eq!(
            MotionOutput::resolve(Some(MotionOutput::CemuHook), ProfileKind::Ps4),
            Ok(MotionOutput::CemuHook)
        );
    }
}
