#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum PointerButton {
  Primary = 0,
  Secondary = 1,
  Tertiary = 2,
  Back = 3,
  Forward = 4,
  Eraser = 5,
  Other(u8) = 6,
}
