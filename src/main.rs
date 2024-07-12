use anyhow::Result;
use smithay::reexports::calloop::EventLoop;

mod state;

use self::state::InfwinityState;

fn main() -> Result<()> {
    // Create the overall event loop that the compositor spins on
    let mut event_loop: EventLoop<InfwinityState> = EventLoop::try_new()?;

    // Prepare the compositor's state
    let mut compositor_state = InfwinityState::new(&mut event_loop)?;

    // Keep the event loop running for the lifetime of the application
    // Other modules will add their own event handlers to this loop
    event_loop.run(None, &mut compositor_state, |_| {})?;

    Ok(())
}
