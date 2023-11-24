use crate::maloja_backend::{direct::DirectBackend, hos::HOSBackend};

pub enum MalojaBackend {
    Direct(DirectBackend),
    HOS(HOSBackend),
}
