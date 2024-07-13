use smithay::{delegate_compositor, reexports::wayland_server::{protocol::wl_surface::WlSurface, Client}, wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState}};

use crate::{Infwinity, InfwinityClientState};

impl CompositorHandler for Infwinity {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<InfwinityClientState>().expect("Missing client data?").client_state
    }

    fn commit(&mut self, _surface: &WlSurface) {
        todo!()
    }
}

delegate_compositor!(Infwinity);