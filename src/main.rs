use anyhow::Result;
use smithay::reexports::calloop::EventLoop;

mod state;

use self::state::CompositorState;

fn main() -> Result<()> {
    // Create the overall event loop that the compositor spins on
    let mut event_loop: EventLoop<CompositorState> = EventLoop::try_new()?;

    // Prepare the compositor's state
    let mut compositor_state = CompositorState::new()?;

    // Keep the event loop running for the lifetime of the application
    // Other modules will add their own event handlers to this loop
    event_loop.run(None, &mut compositor_state, |_| {})?;

    Ok(())
}
