use smithay::utils::{Logical, Point};

type LogicalPoint = Point<i32, Logical>;

/// Utility functions to get canvas-local coordinates
///
/// These are taken relative to the canvas center, which is at the middle of the signed 32-bit positive range
pub struct CanvasCoords;

impl CanvasCoords {
    pub fn from(mut x: i32, mut y: i32) -> LogicalPoint {
        // Shift them relative to the center
        x += i32::MAX / 2;
        y += i32::MAX / 2;

        // And clamp it to the positive range
        Point::from((x.clamp(0, i32::MAX), y.clamp(0, i32::MAX)))        
    }

    pub fn center() -> LogicalPoint {
        Self::from(0, 0)
    }
}
