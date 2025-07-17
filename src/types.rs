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

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
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

/// encodings from python
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Encoding {
    Ascii,
    Base64,
    Big5,
    Big5HkScs,
    Bz2,
    Cp037,
    Cp1026,
    Cp1125,
    Cp1140,
    Cp1250,
    Cp1251,
    Cp1252,
    Cp1253,
    Cp1254,
    Cp1255,
    Cp1256,
    Cp1257,
    Cp1258,
    Cp273,
    Cp424,
    Cp437,
    Cp500,
    Cp775,
    Cp850,
    Cp852,
    Cp855,
    Cp857,
    Cp858,
    Cp860,
    Cp861,
    Cp862,
    Cp863,
    Cp864,
    Cp865,
    Cp866,
    Cp869,
    Cp932,
    Cp949,
    Cp950,
    EucJis2004,
    EucJisx0213,
    EucJp,
    EucKr,
    Gb18030,
    Gb2312,
    Gbk,
    Hex,
    HpRoman8,
    Hz,
    Iso2022Jp,
    Iso2022Jp1,
    Iso2022Jp2,
    Iso2022Jp2004,
    Iso2022Jp3,
    Iso2022JpExt,
    Iso2022Kr,
    Iso8859_10,
    Iso8859_11,
    Iso8859_13,
    Iso8859_14,
    Iso8859_15,
    Iso8859_16,
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    Johab,
    Koi8R,
    Kz1048,
    Latin1,
    MacCyrillic,
    MacGreek,
    MacIceland,
    MacLatin2,
    MacRoman,
    MacTurkish,
    Mbcs,
    Ptcp154,
    Quopri,
    Rot13,
    ShiftJis,
    ShiftJis2004,
    ShiftJisx0213,
    Tis620,
    Utf16,
    Utf16be,
    Utf16le,
    Utf32,
    Utf32be,
    Utf32le,
    Utf7,
    #[default]
    Utf8,
    UU,
    Zlib,
}
