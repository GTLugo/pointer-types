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
/// - **X11:** Technically supports further buttons than this (0..=250), these are emitted in
///   `ButtonSource::Unknown`.
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
