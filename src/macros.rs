pub(crate) mod exports {
    pub use std::fmt;

    #[cfg(feature = "serde")]
    pub use serde::de::{self, Visitor};
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
}

macro_rules! impl_traits {
    ($hex: ident) => {
        impl std::str::FromStr for $hex {
            type Err = std::num::ParseIntError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse_str(s)
            }
        }

        impl std::convert::TryFrom<&str> for $hex {
            type Error = std::num::ParseIntError;

            #[inline]
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::parse_str(s)
            }
        }

        impl fmt::Debug for $hex {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl fmt::Display for $hex {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl From<$hex> for String {
            #[inline]
            fn from(value: $hex) -> String {
                value.to_string()
            }
        }

        #[cfg(feature = "serde")]
        impl Serialize for $hex {
            #[inline]
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.to_string())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $hex {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct HexVisitor;

                impl<'de> Visitor<'de> for HexVisitor {
                    type Value = $hex;

                    #[inline]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("hex string, e.g. 00ab127e6")
                    }

                    #[inline]
                    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        value.parse().map_err(E::custom)
                    }
                }

                deserializer.deserialize_str(HexVisitor)
            }
        }
    };
}

macro_rules! impl_hex {
    ($(#[$meta: meta])*
     pub struct $hex: ident ( $alias: ty );) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $hex($alias);

        impl $hex {
            /// Creates a null hex (i.e., `0`).
            #[allow(dead_code)]
            #[inline]
            pub fn new() -> Self {
                Self(0)
            }

            /// Creates a random hex. This is equivalent to
            /// [`Self::with_rng(rand::thread_rng())`](`Self::with_rng()`).
            #[allow(dead_code)]
            #[cfg(feature = "rand")]
            pub fn rand() -> Self {
                Self::with_rng(rand::thread_rng())
            }

            /// Creates a random hex.
            #[allow(dead_code)]
            #[cfg(feature = "rand")]
            #[inline]
            pub fn with_rng(mut rng: impl rand::Rng) -> Self {
                Self::from(rng.gen())
            }

            /// Gets the interior value.
            #[allow(dead_code)]
            #[inline]
            pub fn get(self) -> $alias {
                self.0
            }

            #[allow(dead_code)]
            #[inline]
            pub fn from(n: $alias) -> Self {
                Self(n)
            }

            #[allow(dead_code)]
            #[inline]
            fn parse_str(s: &str) -> Result<Self, std::num::ParseIntError> {
                <$alias>::from_str_radix(s, 16).map(Self)
            }
        }

        /// Creates a null hex (i.e., `0`).
        impl Default for $hex {
            #[inline]
            fn default() -> Self {
                Self(0)
            }
        }

        impl_traits! { $hex }
    };
}

macro_rules! impl_nonzero_hex {
    ($(#[$meta: meta])*
     pub struct $hex: ident ( $alias: ty => $nonzero: ty );) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $hex($nonzero);

        impl $hex {
            /// Creates a random hex. This is equivalent to
            /// [`Self::with_rng(rand::thread_rng())`](`Self::with_rng()`).
            #[allow(dead_code)]
            #[cfg(feature = "rand")]
            pub fn rand() -> Self {
                Self::with_rng(rand::thread_rng())
            }

            /// Creates a random hex.
            #[allow(dead_code)]
            #[cfg(feature = "rand")]
            pub fn with_rng(mut rng: impl rand::Rng) -> Self {
                Self::from_nonzero(rng.gen())
            }

            /// Gets the interior value.
            #[allow(dead_code)]
            #[inline]
            pub fn get(self) -> $nonzero {
                self.0
            }

            #[allow(dead_code)]
            #[inline]
            pub fn from_nonzero(n: $nonzero) -> Self {
                Self(n)
            }

            #[allow(dead_code)]
            #[inline]
            pub fn from(n: $alias) -> Option<Self> {
                <$nonzero>::new(n).map(Self)
            }

            /// # Safety
            ///
            /// The value must not be zero.
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn from_unchecked(n: $alias) -> Self {
                Self(<$nonzero>::new_unchecked(n))
            }

            #[allow(dead_code)]
            fn parse_str(s: &str) -> Result<Self, std::num::ParseIntError> {
                <$alias>::from_str_radix(s, 16).and_then(|n| {
                    if n == 0 {
                        Err("0".parse::<$nonzero>().unwrap_err())
                    } else {
                        // SAFETY: n is nonzero
                        unsafe { Ok(Self::from_unchecked(n)) }
                    }
                })
            }
        }

        impl_traits! { $hex }
    };
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::exports::*;

    impl_hex! {
        pub struct MyHex(u32);
    }

    impl_nonzero_hex! {
        pub struct MyNonZero(u32 => std::num::NonZeroU32);
    }

    #[test]
    fn hex_from_str() {
        let res = "ae01f7d".parse::<MyHex>();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), MyHex::from(0xae01f7d));

        let res = "ghx".parse::<MyHex>();
        assert!(res.is_err());

        let res = "0".parse::<MyNonZero>();
        assert!(res.is_err());
    }

    #[test]
    fn hex_to_str() {
        let hex = MyHex::from(0xae01f7d);
        assert_eq!(hex.to_string(), String::from("ae01f7d"));
    }
}
