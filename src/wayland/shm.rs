use smithay::{delegate_shm, reexports::wayland_server::protocol::wl_buffer::WlBuffer, wayland::{buffer::BufferHandler, shm::{ShmHandler, ShmState}}};

use crate::Infwinity;

impl ShmHandler for Infwinity {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl BufferHandler for Infwinity {
    fn buffer_destroyed(&mut self, _buffer: &WlBuffer) {}
}

delegate_shm!(Infwinity);