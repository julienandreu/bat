// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
// This is not a library, so we don't need to worry about intra-doc links
#![allow(rustdoc::private_intra_doc_links)]

pub mod animation;
pub mod bootstrap;
pub mod camera;
pub mod entities;
pub mod input;
pub mod performance;
pub mod physics;
pub mod states;
