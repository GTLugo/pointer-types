#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod mouse;
pub mod pointer;
pub mod wheel;

pub use pointer::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PointerType {
  #[default]
  Mouse,
  Stylus,
  Touch,
}

/// Describes the state a button is in.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ButtonState {
  /// The button is pressed down.
  ///
  /// Often emitted in a [mousedown] event, see also [the MDN documentation][mdn] on that.
  ///
  /// [mousedown]: https://w3c.github.io/pointerevents/#mousedown
  /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
  #[default]
  Down,
  /// The button is not pressed / was just released.
  ///
  /// Often emitted in a [mouseup] event, see also [the MDN documentation][mdn] on that.
  ///
  /// [mouseup]: https://w3c.github.io/pointerevents/#mouseup
  /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
  Up,
}

impl ButtonState {
  /// True if the button is pressed down.
  pub const fn is_down(self) -> bool {
    matches!(self, Self::Down)
  }

  /// True if the button is released.
  pub const fn is_up(self) -> bool {
    matches!(self, Self::Up)
  }
}
