pub mod button;
pub use button::*;
use {
  crate::{
    ButtonState,
    PointerType,
    mouse::MouseEvent,
  },
  dpi::PhysicalPosition,
  keyboard_types::Modifiers,
};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PointerEvent {
  Button {
    /// The type of pointer that fired this event.
    pointer: PointerType,
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
    /// Whether the button is pressed or released.
    state: ButtonState,
    /// Logical button value.
    button: PointerButton,
    /// Flags for pressed modifier keys.
    modifiers: Modifiers,
    /// Whether this event is a double click/tap.
    is_double: bool,
  },
  Move {
    /// The type of pointer that fired this event.
    pointer: PointerType,
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
  Enter {
    /// The type of pointer that fired this event.
    pointer: PointerType,
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
  Leave {
    /// The type of pointer that fired this event.
    pointer: PointerType,
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
}

impl PointerEvent {
  pub fn mouse_event(self) -> Option<MouseEvent> {
    self.try_into().ok()
  }
}
