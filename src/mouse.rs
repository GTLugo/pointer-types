pub mod button;
pub use button::*;
#[cfg(feature = "serde")]
use serde::{
  Deserialize,
  Serialize,
};
use {
  crate::{
    ButtonState,
    PointerEvent,
    PointerType,
  },
  dpi::PhysicalPosition,
  keyboard_types::Modifiers,
};

/// Mouse events are issued for all pressed and released mouse buttons.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseEvent {
  Button {
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
    /// Whether the button is pressed or released.
    state: ButtonState,
    /// Logical button value.
    button: MouseButton,
    /// Flags for pressed modifier keys.
    modifiers: Modifiers,
    /// Whether this event is a double click.
    is_double_click: bool,
  },
  Move {
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
  Enter {
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
  Leave {
    /// The position relative to the top-left of the client area.
    position: PhysicalPosition<f64>,
  },
}

impl From<MouseEvent> for PointerEvent {
  fn from(event: MouseEvent) -> Self {
    match event {
      MouseEvent::Button {
        position,
        state,
        button,
        modifiers,
        is_double_click,
      } => Self::Button {
        pointer: PointerType::Mouse,
        position,
        state,
        button: button.into(),
        modifiers,
        is_double: is_double_click,
      },
      MouseEvent::Move { position } => Self::Move {
        pointer: PointerType::Mouse,
        position,
      },
      MouseEvent::Enter { position } => Self::Enter {
        pointer: PointerType::Mouse,
        position,
      },
      MouseEvent::Leave { position } => Self::Leave {
        pointer: PointerType::Mouse,
        position,
      },
    }
  }
}

impl TryFrom<PointerEvent> for MouseEvent {
  type Error = ();

  fn try_from(event: PointerEvent) -> Result<Self, Self::Error> {
    Ok(match event {
      PointerEvent::Button {
        pointer: PointerType::Mouse,
        position,
        state,
        button,
        modifiers,
        is_double: is_double_click,
      } => Self::Button {
        position,
        state,
        button: button.into(),
        modifiers,
        is_double_click,
      },
      PointerEvent::Move {
        pointer: PointerType::Mouse,
        position,
      } => Self::Move { position },
      PointerEvent::Enter {
        pointer: PointerType::Mouse,
        position,
      } => Self::Enter { position },
      PointerEvent::Leave {
        pointer: PointerType::Mouse,
        position,
      } => Self::Leave { position },
      _ => return Err(()),
    })
  }
}
