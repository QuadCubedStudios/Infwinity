use std::sync::Arc;

use smithay::{
    reexports::{
        calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction},
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    wayland::{compositor::CompositorClientState, socket::ListeningSocketSource},
};

/// Backend-agnostic compositor state
pub struct InfwinityState {
    display_handle: DisplayHandle,
}

impl InfwinityState {
    pub fn new(event_loop: &mut EventLoop<Self>) -> anyhow::Result<Self> {
        // Create the display
        let display: Display<Self> = Display::new()?;
        let display_handle = display.handle();

        // Estabish the unix socket we will listen on
        let socket = ListeningSocketSource::new_auto()?;

        eprintln!("Listening on: {:?}", socket.socket_name());

        // Listen for clients connecting to the socket, and attach them to the server
        event_loop
            .handle()
            .insert_source(socket, |client_stream, _meta, state| {
                eprintln!("Client joined: {:?}", client_stream.peer_addr());
                state
                    .display_handle
                    .insert_client(client_stream, Arc::new(InfwinityClientState::default()))
                    .expect("Couldn't add client?");
            })?;

        // And listen for any messages from clients
        event_loop.handle().insert_source(
            Generic::new(display, Interest::READ, Mode::Level),
            |_ready, display, state| {
                eprintln!("Client events received");

                // SAFETY: display is not dropped
                unsafe {
                    // Dispatch all received messages to the event loop
                    display.get_mut().dispatch_clients(state)?;
                    // Temporarily flush client messages right away
                    // FIXME: This should be done when a frame redraw is issued
                    display.get_mut().flush_clients().expect("Failed to flush?");
                }

                Ok(PostAction::Continue)
            },
        )?;

        Ok(Self { display_handle })
    }
}

#[derive(Debug, Default)]
struct InfwinityClientState {
    client_state: CompositorClientState,
}

impl ClientData for InfwinityClientState {
    fn initialized(&self, client_id: ClientId) {
        eprintln!("Client Initialized: {client_id:?}");
    }

    fn disconnected(&self, client_id: ClientId, reason: DisconnectReason) {
        eprintln!("Client Disconnected: {client_id:?} {reason:?}");
    }
}
