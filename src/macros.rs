pub(crate) mod exports {
    pub use std::fmt;
}

#[cfg(feature = "serde")]
macro_rules! _impl_serde {
    ($hex: ident) => {
        #[cfg(feature = "serde")]
        impl serde::ser::Serialize for $hex {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::de::Deserialize<'de> for $hex {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::{Error, Visitor};
                struct HexVisitor;

                impl<'de> Visitor<'de> for HexVisitor {
                    type Value = $hex;

                    #[inline]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("hex string, e.g. 00ab127e6")
                    }

                    #[inline]
                    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                        value.parse().map_err(E::custom)
                    }
                }

                deserializer.deserialize_str(HexVisitor)
            }
        }
    };
}

#[cfg(feature = "db")]
macro_rules! _impl_diesel {
    ($hex: ident, $inner: ty) => {
        impl<T, DB> diesel::serialize::ToSql<T, DB> for $hex
        where
            DB: diesel::backend::Backend,
            $inner: diesel::serialize::ToSql<T, DB>,
        {
            #[inline]
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> diesel::serialize::Result {
                self.get_ref().to_sql(out)
            }
        }

        impl<DB, ST> diesel::deserialize::Queryable<ST, DB> for $hex
        where
            DB: diesel::backend::Backend,
            $inner: diesel::deserialize::Queryable<ST, DB>,
        {
            type Row = <$inner as diesel::deserialize::Queryable<ST, DB>>::Row;

            #[inline]
            fn build(row: Self::Row) -> Self {
                Self(<$inner>::build(row))
            }
        }

        impl<'a, T> diesel::expression::AsExpression<T> for &'a $hex
        where
            &'a $inner: diesel::expression::AsExpression<T>,
        {
            type Expression = <&'a $inner as diesel::expression::AsExpression<T>>::Expression;

            fn as_expression(self) -> Self::Expression {
                self.get_ref().as_expression()
            }
        }

        impl<T> diesel::expression::AsExpression<T> for $hex
        where
            $inner: diesel::expression::AsExpression<T>,
        {
            type Expression = <$inner as diesel::expression::AsExpression<T>>::Expression;

            fn as_expression(self) -> Self::Expression {
                self.get().as_expression()
            }
        }
    };
}

macro_rules! _impl_hex_common {
    ($hex: ident, $inner: ty) => {
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
                Self(rng.gen())
            }

            /// Gets the interior value.
            #[allow(dead_code)]
            #[inline]
            pub fn get(self) -> $inner {
                self.0
            }

            /// Borrows the interior value.
            #[allow(dead_code)]
            #[inline]
            pub fn get_ref(&self) -> &$inner {
                &self.0
            }

            /// Gets the mutable reference to the interior value.
            #[allow(dead_code)]
            #[inline]
            pub fn get_ref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl From<$inner> for $hex {
            #[inline]
            fn from(n: $inner) -> Self {
                Self(n)
            }
        }

        impl std::str::FromStr for $hex {
            type Err = std::num::ParseIntError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse_str(s)
            }
        }

        impl TryFrom<&str> for $hex {
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
            pub fn new(n: $alias) -> Self {
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

        _impl_hex_common! { $hex, $alias }
        #[cfg(feature = "serde")]
        _impl_serde! { $hex }
        #[cfg(feature = "db")]
        _impl_diesel! { $hex, $alias }
    };
}

macro_rules! impl_nonzero_hex {
    ($(#[$meta: meta])*
     pub struct $hex: ident ( $alias: ty => $nonzero: ty );) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $hex($nonzero);

        impl $hex {
            #[allow(dead_code)]
            #[inline]
            pub fn new(n: $alias) -> Option<Self> {
                <$nonzero>::new(n).map(Self)
            }

            /// # Safety
            ///
            /// The value must not be zero.
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn new_unchecked(n: $alias) -> Self {
                Self(<$nonzero>::new_unchecked(n))
            }

            #[allow(dead_code)]
            fn parse_str(s: &str) -> Result<Self, std::num::ParseIntError> {
                <$alias>::from_str_radix(s, 16).and_then(|n| {
                    if n == 0 {
                        Err("0".parse::<$nonzero>().unwrap_err())
                    } else {
                        // SAFETY: n is nonzero
                        unsafe { Ok(Self::new_unchecked(n)) }
                    }
                })
            }
        }

        impl TryFrom<$alias> for $hex {
            type Error = <$nonzero as TryFrom<$alias>>::Error;

            fn try_from(value: $alias) -> Result<Self, Self::Error> {
                <$nonzero>::try_from(value).map(Self)
            }
        }

        _impl_hex_common! { $hex, $nonzero }
        #[cfg(feature = "serde")]
        _impl_serde! { $hex }
        #[cfg(feature = "db")]
        _impl_diesel! { $hex, $nonzero }
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
