#![no_std]

use self::mouse::event::MouseEvent;
#[cfg(feature = "std")]
extern crate std;

pub mod mouse;

pub enum PointerEvent {
  Mouse(MouseEvent),
}
