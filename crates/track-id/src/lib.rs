//! track-id — identity overlay for flightsuite tracks.
//!
//! Consumes `DetectedObject` events from flight-vision (anonymous bbox +
//! track id), runs face / gait / ReID / voice models against the cropped
//! frame regions, and emits `IdentifiedTrack` events with attached
//! identity + confidence.
//!
//! Backend: tract-onnx for portability across SBCs (Pi, Jetson via CPU).
//! For Jetson with TensorRT acceleration, swap to ort + CUDA EP — the
//! trait surface is identical.
//!
//! Status: scaffold. Modules below are placeholders to be filled in as
//! the desktop visionsystem versions are matured.

pub mod face;
pub mod gait;
pub mod reid;
pub mod voice;
pub mod fusion;
pub mod types;

pub use types::{IdentifiedTrack, IdentitySource};
