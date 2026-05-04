//! Multi-channel identity fusion. Combines face + gait + reid + voice
//! per-track with a weighted sum; emits IdentitySource::Fused above a
//! configurable threshold. Single-channel matches still emit individually.
//!
//! Stub — fill once at least two of the four channels are wired.

pub struct Fusion;
