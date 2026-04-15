#[cfg(feature = "serde")]
use serde::{
  Deserialize,
  Serialize,
};
use {
  super::{
    button::MouseButton,
    state::ButtonState,
  },
  dpi::PhysicalPosition,
  keyboard_types::Modifiers,
};

/// Mouse events are issued for all pressed and released mouse buttons.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseEvent {
  /// The position of the mouse cursor.
  pub position: PhysicalPosition<i32>,
  /// Whether the button is pressed or released.
  pub state: ButtonState,
  /// Logical button value.
  pub button: MouseButton,
  /// Flags for pressed modifier keys.
  pub modifiers: Modifiers,
  /// Whether this event is a double click.
  pub is_double_click: bool,
}
