use smithay::{
    delegate_xdg_shell,
    desktop::Window,
    reexports::wayland_server::protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    utils::Serial,
    wayland::{
        compositor::with_states,
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
    },
};

use crate::Infwinity;

/// Handles requests for constructing windows
impl XdgShellHandler for Infwinity {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        self.register_window(Window::new_wayland_window(surface));
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // TODO: Popups
    }

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
        // TODO: Popups
    }

    fn move_request(&mut self, _surface: ToplevelSurface, _seat: WlSeat, _serial: Serial) {
        // TODO: Window movement
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        // TODO Popups
    }
}

impl Infwinity {
    pub(crate) fn on_commit_xdg(&mut self, surface: &WlSurface) {
        // Check if this surface defines a window
        if let Some(window) = self
            .space
            .elements()
            .find(|w| w.toplevel().unwrap().wl_surface() == surface)
            .cloned()
        {
            // For the first commit, Wayland requires the server to issue the first configure message
            let initial_configure_sent = with_states(surface, |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .initial_configure_sent
            });

            if !initial_configure_sent {
                window.toplevel().unwrap().send_configure();
            }

            // Tell the window that its surface has been committed
            window.on_commit();
        }
    }
}

delegate_xdg_shell!(Infwinity);
