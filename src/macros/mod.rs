#[macro_export]
macro_rules! enum_extend {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $( $(#[$variant_meta:meta])* $variant:ident = $value:expr, )* $(,)?
        },
        $value_type:ty,
        $err_type:ty,
        $err_info:tt
    ) => {
        #[repr($value_type)]
        $(#[$meta])*
        $vis enum $name {
            $( $(#[$variant_meta])* $variant = $value, )*
        }

        impl Into<$value_type> for $name {
            #[inline(always)]
            fn into(self) -> $value_type {
                match self {
                    $( $(#[$variant_meta])* Self::$variant => $value, )*
                }
            }
        }

        impl TryFrom<$value_type> for $name {
            type Error = $err_type;
            fn try_from(val: $value_type) -> Result<Self, Self::Error> {
                match val {
                    $( $(#[$variant_meta])* $value => Ok(Self::$variant), )*
                    _ => Err(Self::Error::$err_info((val))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, thiserror::Error)]
    pub enum Error {
        #[error("invalid value: {0}")]
        InvalidValue(u8),
    }

    #[test]
    fn test_enum_extend() {
        enum_extend!(
            #[derive(Debug, PartialEq)]
            enum A {
                T1 = 0x01,
                T2 = 0x02,
            }, u8, Error, InvalidValue);

        assert_eq!(<A as TryFrom<u8>>::try_from(3), Err(Error::InvalidValue(3)));
        assert_eq!(<A as TryFrom<u8>>::try_from(1), Ok(A::T1));
    }
}
