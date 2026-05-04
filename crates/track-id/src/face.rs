//! Face identity via ArcFace — wraps `vs_perceive::face_id::ArcFace` so the
//! existing desktop integration drops in unchanged.

pub use vs_perceive::face_id::ArcFace;
pub use vs_perceive::enrollment::EnrollmentSet;
