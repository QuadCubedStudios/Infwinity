use smithay::{
    input::SeatState,
    reexports::{
        calloop::{generic::Generic, Interest, Mode, PostAction},
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        shell::xdg::XdgShellState,
        shm::ShmState,
    },
};

mod backends;
mod wayland;

/// The compositor itself
///
/// This struct maintains the compositor's state, which is affected
/// by the event loop
pub struct Infwinity {
    compositor_state: CompositorState,
    seat_state: SeatState<Self>,
    shm_state: ShmState,
    xdg_shell_state: XdgShellState,

    display_handle: DisplayHandle,
}

/// Type alias for our compositor's event loop
pub type EventLoop<'a> = smithay::reexports::calloop::EventLoop<'a, Infwinity>;
/// Type alias for our compositor's event loop
pub type EventLoopHandle<'a> = smithay::reexports::calloop::LoopHandle<'a, Infwinity>;

impl Infwinity {
    /// Creates a new instance of the compositor
    pub fn new(event_loop: &mut EventLoop) -> anyhow::Result<Self> {
        // NOTE: As much as possible, make this function call out to other init functions for readability
        let event_loop = event_loop.handle();

        let display_handle = Self::create_display(&event_loop)?;

        // Construct the compositor and all of its state data first
        let mut compositor = Self {
            compositor_state: CompositorState::new::<Self>(&display_handle),
            seat_state: SeatState::new(),
            shm_state: ShmState::new::<Self>(&display_handle, vec![]),
            xdg_shell_state: XdgShellState::new::<Self>(&display_handle),

            display_handle,
        };

        // Then handle all initialization steps
        compositor.init_socket(&event_loop)?;

        Ok(compositor)
    }

    fn create_display(event_loop: &EventLoopHandle) -> anyhow::Result<DisplayHandle> {
        // Create the display, which serves as the main entry point to Wayland
        let display: Display<Self> = Display::new()?;
        let display_handle = display.handle();

        // And listen for any messages from clients
        event_loop.insert_source(
            Generic::new(display, Interest::READ, Mode::Level),
            |_ready, display, state| {
                log::info!("Client events received");

                // SAFETY: display is not dropped
                unsafe {
                    // Dispatch all received messages to the event loop
                    display.get_mut().dispatch_clients(state)?;
                }

                Ok(PostAction::Continue)
            },
        )?;

        Ok(display_handle)
    }

    pub fn on_frame_done(&mut self) {
        self.display_handle
            .flush_clients()
            .expect("Failed to flush buffers?");
    }
}

#[derive(Debug, Default)]
struct InfwinityClientState {
    client_state: CompositorClientState,
}

impl ClientData for InfwinityClientState {
    fn initialized(&self, client_id: ClientId) {
        log::info!("Client Initialized: {client_id:?}");
    }

    fn disconnected(&self, client_id: ClientId, reason: DisconnectReason) {
        log::info!("Client Disconnected: {client_id:?} {reason:?}");
    }
}
