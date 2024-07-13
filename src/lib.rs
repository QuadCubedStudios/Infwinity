use std::time::{Duration, Instant};

use smithay::{
    desktop::{Space, Window},
    input::SeatState,
    output::Output,
    reexports::{
        calloop::{generic::Generic, Interest, LoopSignal, Mode, PostAction},
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
use util::CanvasCoords;

mod backends;
mod util;
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

    space: Space<Window>,

    loop_signal: LoopSignal,
    display_handle: DisplayHandle,
    start_time: Instant,
}

/// Type alias for our compositor's event loop
pub type EventLoop<'a> = smithay::reexports::calloop::EventLoop<'a, Infwinity>;
/// Type alias for our compositor's event loop
pub type EventLoopHandle<'a> = smithay::reexports::calloop::LoopHandle<'a, Infwinity>;

impl Infwinity {
    /// Creates a new instance of the compositor
    pub fn new(event_loop: &mut EventLoop) -> anyhow::Result<Self> {
        // NOTE: As much as possible, make this function call out to other init functions for readability
        let loop_signal = event_loop.get_signal();
        let event_loop = event_loop.handle();

        let display_handle = Self::create_display(&event_loop)?;

        // Construct the compositor and all of its state data first
        let mut compositor = Self {
            compositor_state: CompositorState::new::<Self>(&display_handle),
            seat_state: SeatState::new(),
            shm_state: ShmState::new::<Self>(&display_handle, vec![]),
            xdg_shell_state: XdgShellState::new::<Self>(&display_handle),

            space: Space::default(),

            loop_signal,
            display_handle,
            start_time: Instant::now(),
        };

        // Then handle all initialization steps
        compositor.init_winit(&event_loop)?;
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

    /// Registers the provided output to the canvas
    ///
    /// 
    fn register_output(&mut self, output: &Output) {
        // Add the output to our display
        let _global = output.create_global::<Self>(&self.display_handle);

        // And map it to the center of our space
        // TODO: Check this calculation for HiDPI
        let offset = output
            .current_mode()
            .map_or((0, 0).into(), |x| {
                x.size.to_logical(output.current_scale().integer_scale())
            })
            .downscale(2);

        self.space
            .map_output(output, CanvasCoords::center() - offset);
    }

    /// Adds a window to our canvas
    /// 
    /// The provided window will be positioned at canvas origin
    fn register_window(&mut self, window: Window) {
        // Place this window centered at the canvas origin
        let offset = window.geometry().size.downscale(2);

        self.space
            .map_element(window, CanvasCoords::center() - offset, false);
    }

    /// To be called after a frame has been rendered on the provided output
    fn after_frame_rendered(&mut self, output: &Output) {
        // Temporarily shift the output around, to demonstrate canvas panning
        let t = self.start_time.elapsed().as_secs_f32();
        let (x, y) = t.sin_cos();

        let mut offset = output
            .current_mode()
            .map_or((0, 0).into(), |x| {
                x.size.to_logical(output.current_scale().integer_scale())
            })
            .downscale(2);

        offset.w += (x * 100.0).round() as i32;
        offset.h += (y * 100.0).round() as i32;

        self.space
            .map_output(output, CanvasCoords::center() - offset);

        // Go through every window in the space, and let them know
        self.space.elements().for_each(|window| {
            window.send_frame(
                output,
                self.start_time.elapsed(),
                Some(Duration::ZERO),
                |_, _| Some(output.clone()),
            )
        });

        self.space.refresh();
        let _ = self.display_handle.flush_clients();
    }

    pub fn shutdown(&self) {
        self.loop_signal.stop()
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
