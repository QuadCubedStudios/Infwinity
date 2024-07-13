use std::sync::Arc;

use smithay::wayland::socket::ListeningSocketSource;

use crate::{EventLoopHandle, Infwinity, InfwinityClientState};

impl Infwinity {
    pub(crate) fn init_socket(&mut self, event_loop: &EventLoopHandle) -> anyhow::Result<()> {
        // Estabish the unix socket we will listen on
        let socket = ListeningSocketSource::new_auto()?;

        eprintln!("Listening on: {:?}", socket.socket_name());

        // Listen for clients connecting to the socket
        event_loop.insert_source(socket, |client_stream, _meta, state| {
            eprintln!("Client joined: {:?}", client_stream.peer_addr());

            // And attach them to the server
            state
                .display_handle
                .insert_client(client_stream, Arc::new(InfwinityClientState::default()))
                .expect("Couldn't add client?");
        })?;

        Ok(())
    }
}
