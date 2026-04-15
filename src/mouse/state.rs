#[cfg(feature = "serde")]
use serde::{
  Deserialize,
  Serialize,
};

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
  /// The [type] name of the corresponding mouse event.
  ///
  /// This is either `"mousedown"` or `"mouseup"`.
  ///
  /// [type]: https://w3c.github.io/pointerevents/#mouse-event-types
  pub const fn event_type(self) -> &'static str {
    match self {
      Self::Down => "mousedown",
      Self::Up => "mouseup",
    }
  }

  /// True if the button is pressed down.
  pub const fn is_down(self) -> bool {
    matches!(self, Self::Down)
  }

  /// True if the button is released.
  pub const fn is_up(self) -> bool {
    matches!(self, Self::Up)
  }
}
