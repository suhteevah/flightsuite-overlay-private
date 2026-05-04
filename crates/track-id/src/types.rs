use flight_vision::types::{BBox, Track};
use serde::{Deserialize, Serialize};

/// Which biometric channel produced an identity claim.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySource {
    Face,
    Gait,
    Reid,
    Voice,
    /// Multi-channel agreement above a fusion threshold.
    Fused,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentifiedTrack {
    pub track: Track,
    pub identity: Option<String>,
    pub similarity: f32,
    pub source: IdentitySource,
}
