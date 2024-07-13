use smithay::{delegate_output, wayland::output::OutputHandler};

use crate::Infwinity;

mod compositor;
mod seat;
mod shm;
mod socket;
mod xdg;

// Simpler handlers go here

impl OutputHandler for Infwinity {}
delegate_output!(Infwinity);
