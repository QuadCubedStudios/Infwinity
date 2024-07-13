use std::time::Duration;

use anyhow::Result;
use smithay::reexports::calloop::EventLoop;

use infwinity::Infwinity;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Debug).expect("Failed to initialize logger?");

    // Create the overall event loop that the compositor spins on
    let mut event_loop: EventLoop<Infwinity> = EventLoop::try_new()?;

    // Prepare the compositor instance
    let mut compositor_state = Infwinity::new(&mut event_loop)?;

    // Keep the event loop running for the lifetime of the application
    // Other modules will add their own event handlers to this loop
    event_loop.run(
        Some(Duration::from_millis(16)),
        &mut compositor_state,
        |_| {},
    )?;

    Ok(())
}
