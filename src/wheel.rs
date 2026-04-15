use dpi::PhysicalPosition;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WheelEvent {
  delta: MouseScrollDelta,
}

/// Describes a difference in the mouse scroll wheel state.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseScrollDelta {
  /// Amount in lines or rows to scroll in the horizontal
  /// and vertical directions.
  ///
  /// Positive values indicate that the content that is being scrolled should move
  /// right and down (revealing more content left and up).
  LineDelta(f32, f32),

  /// Amount in pixels to scroll in the horizontal and
  /// vertical direction.
  ///
  /// Scroll events are expressed as a `PixelDelta` if
  /// supported by the device (eg. a touchpad) and
  /// platform.
  ///
  /// Positive values indicate that the content being scrolled should
  /// move right/down.
  ///
  /// For a 'natural scrolling' touch pad (that acts like a touch screen)
  /// this means moving your fingers right and down should give positive values,
  /// and move the content right and down (to reveal more things left and up).
  PixelDelta(PhysicalPosition<f64>),
}
