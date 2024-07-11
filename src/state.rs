use smithay::reexports::wayland_server::{Display, DisplayHandle};

/// Backend-agnostic compositor state
pub struct CompositorState {}

impl CompositorState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}