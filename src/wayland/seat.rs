use smithay::{delegate_seat, input::{SeatHandler, SeatState}, reexports::wayland_server::protocol::wl_surface::WlSurface};

use crate::Infwinity;

impl SeatHandler for Infwinity {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(Infwinity);