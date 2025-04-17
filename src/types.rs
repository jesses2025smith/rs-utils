/// Represents the byte order (endianness) used for interpreting binary data.
///
/// Byte order determines how multi-byte data types (e.g., integers, floating-point numbers)
/// are stored in memory. This enum provides variants for the most common byte orders:
///
/// - `Big`: Big-endian byte order, where the most significant byte is stored first.
/// - `Little`: Little-endian byte order, where the least significant byte is stored first.
/// - `Native`: The native byte order of the target platform, determined at compile time.
///
/// # Variants
///
/// - `Big`: Represents big-endian byte order.
/// - `Little`: Represents little-endian byte order. This is the default variant.
/// - `Native`: Represents the native byte order of the target platform. This variant is
///   conditionally compiled based on the target platform's endianness.
///
/// # Example
///
/// ```rust
/// use rsutil::types::ByteOrder;
///
/// #[cfg(target_endian = "little")]
/// assert!(ByteOrder::Native.is_little());
/// #[cfg(target_endian = "little")]
/// assert!(!ByteOrder::Native.is_big());
/// #[cfg(target_endian = "big")]
/// assert!(!ByteOrder::Native.is_little());
/// #[cfg(target_endian = "big")]
/// assert!(ByteOrder::Native.is_big());
/// assert!(ByteOrder::Native.is_native());
///
/// let order = ByteOrder::Little;
/// assert!(order.is_little());
/// assert!(!order.is_big());
/// #[cfg(target_endian = "little")]
/// assert!(order.is_native());
/// #[cfg(target_endian = "big")]
/// assert!(!order.is_native());
///
/// let order = ByteOrder::Big;
/// assert!(!order.is_little());
/// assert!(order.is_big());
/// #[cfg(target_endian = "little")]
/// assert!(!order.is_native());
/// #[cfg(target_endian = "big")]
/// assert!(order.is_native());
/// ```
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ByteOrder {
    Big,
    #[default]
    Little,
    #[cfg(target_endian = "little")]
    Native,
    #[cfg(target_endian = "big")]
    Native,
}

impl ByteOrder {
    pub fn is_little(&self) -> bool {
        match self {
            Self::Big => false,
            Self::Little => true,
            Self::Native => cfg!(target_endian = "little"),
        }
    }

    pub fn is_big(&self) -> bool {
        match self {
            Self::Big => true,
            Self::Little => false,
            Self::Native => cfg!(target_endian = "big"),
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            Self::Big => cfg!(target_endian = "big"),
            Self::Little => cfg!(target_endian = "little"),
            Self::Native => true,
        }
    }
}
