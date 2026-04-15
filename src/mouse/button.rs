use crate::PointerButton;

/// Identifies a button of a mouse controller.
///
/// ## Platform-specific
///
/// The first three buttons should be supported on all platforms.
/// [`Self::Back`] and [`Self::Forward`] are supported on most platforms
/// (when using a compatible mouse).
///
/// - **Android, iOS:** Currently not supported.
/// - **Orbital:** Only left/right/middle buttons are supported at this time.
/// - **Web, Windows:** Supports left/right/middle/back/forward buttons.
/// - **Wayland:** Supports buttons 0..=15.
/// - **macOS:** Supports all button variants.
/// - **X11:** Technically supports further buttons than this (0..=250).
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum MouseButton {
  /// The primary (usually left) button
  Left = 0,
  /// The secondary (usually right) button
  Right = 1,
  /// The tertiary (usually middle) button
  Middle = 2,
  /// The first side button, frequently assigned a back function
  Back = 3,
  /// The second side button, frequently assigned a forward function
  Forward = 4,
  /// The sixth button
  Button6 = 5,
  /// The seventh button
  Button7 = 6,
  /// The eighth button
  Button8 = 7,
  /// The ninth button
  Button9 = 8,
  /// The tenth button
  Button10 = 9,
  /// The eleventh button
  Button11 = 10,
  /// The twelfth button
  Button12 = 11,
  /// The thirteenth button
  Button13 = 12,
  /// The fourteenth button
  Button14 = 13,
  /// The fifteenth button
  Button15 = 14,
  /// The sixteenth button
  Button16 = 15,
  Button17 = 16,
  Button18 = 17,
  Button19 = 18,
  Button20 = 19,
  Button21 = 20,
  Button22 = 21,
  Button23 = 22,
  Button24 = 23,
  Button25 = 24,
  Button26 = 25,
  Button27 = 26,
  Button28 = 27,
  Button29 = 28,
  Button30 = 29,
  Button31 = 30,
  Button32 = 31,
}

impl From<MouseButton> for PointerButton {
  fn from(button: MouseButton) -> Self {
    match button {
      MouseButton::Left => PointerButton::Primary,
      MouseButton::Right => PointerButton::Secondary,
      MouseButton::Middle => PointerButton::Tertiary,
      MouseButton::Back => PointerButton::Back,
      MouseButton::Forward => PointerButton::Forward,
      MouseButton::Button6 => PointerButton::Eraser,
      MouseButton::Button7 => PointerButton::Other(7),
      MouseButton::Button8 => PointerButton::Other(8),
      MouseButton::Button9 => PointerButton::Other(9),
      MouseButton::Button10 => PointerButton::Other(10),
      MouseButton::Button11 => PointerButton::Other(11),
      MouseButton::Button12 => PointerButton::Other(12),
      MouseButton::Button13 => PointerButton::Other(13),
      MouseButton::Button14 => PointerButton::Other(14),
      MouseButton::Button15 => PointerButton::Other(15),
      MouseButton::Button16 => PointerButton::Other(16),
      MouseButton::Button17 => PointerButton::Other(17),
      MouseButton::Button18 => PointerButton::Other(18),
      MouseButton::Button19 => PointerButton::Other(19),
      MouseButton::Button20 => PointerButton::Other(20),
      MouseButton::Button21 => PointerButton::Other(21),
      MouseButton::Button22 => PointerButton::Other(22),
      MouseButton::Button23 => PointerButton::Other(23),
      MouseButton::Button24 => PointerButton::Other(24),
      MouseButton::Button25 => PointerButton::Other(25),
      MouseButton::Button26 => PointerButton::Other(26),
      MouseButton::Button27 => PointerButton::Other(27),
      MouseButton::Button28 => PointerButton::Other(28),
      MouseButton::Button29 => PointerButton::Other(29),
      MouseButton::Button30 => PointerButton::Other(30),
      MouseButton::Button31 => PointerButton::Other(31),
      MouseButton::Button32 => PointerButton::Other(32),
    }
  }
}

impl From<PointerButton> for MouseButton {
  fn from(button: PointerButton) -> Self {
    match button {
      PointerButton::Primary => MouseButton::Left,
      PointerButton::Secondary => MouseButton::Right,
      PointerButton::Tertiary => MouseButton::Middle,
      PointerButton::Back => MouseButton::Back,
      PointerButton::Forward => MouseButton::Forward,
      PointerButton::Eraser => MouseButton::Button6,
      PointerButton::Other(n) => match n {
        7 => MouseButton::Button7,
        8 => MouseButton::Button8,
        9 => MouseButton::Button9,
        10 => MouseButton::Button10,
        11 => MouseButton::Button11,
        12 => MouseButton::Button12,
        13 => MouseButton::Button13,
        14 => MouseButton::Button14,
        15 => MouseButton::Button15,
        16 => MouseButton::Button16,
        17 => MouseButton::Button17,
        18 => MouseButton::Button18,
        19 => MouseButton::Button19,
        20 => MouseButton::Button20,
        21 => MouseButton::Button21,
        22 => MouseButton::Button22,
        23 => MouseButton::Button23,
        24 => MouseButton::Button24,
        25 => MouseButton::Button25,
        26 => MouseButton::Button26,
        27 => MouseButton::Button27,
        28 => MouseButton::Button28,
        29 => MouseButton::Button29,
        30 => MouseButton::Button30,
        31 => MouseButton::Button31,
        _ => unimplemented!("no mouse button for pointer button `{button:?}`"),
      },
    }
  }
}
