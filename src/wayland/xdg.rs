use smithay::{
    delegate_xdg_shell,
    reexports::wayland_server::protocol::wl_seat::WlSeat,
    utils::Serial,
    wayland::shell::xdg::{
        PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
    },
};

use crate::Infwinity;

/// Handles requests for constructing windows
impl XdgShellHandler for Infwinity {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        // Called when a client wants a new standalone window
        log::info!("New top-level surface: {surface:?}");
    }

    fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState) {
        log::info!("New popup surface: {surface:?} {positioner:?}");
    }

    fn grab(&mut self, surface: PopupSurface, seat: WlSeat, serial: Serial) {
        todo!()
    }

    fn reposition_request(
        &mut self,
        surface: PopupSurface,
        positioner: PositionerState,
        token: u32,
    ) {
        todo!()
    }
}

delegate_xdg_shell!(Infwinity);
