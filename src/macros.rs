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
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, DB>,
            ) -> diesel::serialize::Result {
                self.get_ref().to_sql(out)
            }
        }

        impl<T, DB> diesel::deserialize::FromSql<T, DB> for $hex
        where
            DB: diesel::backend::Backend,
            $inner: diesel::deserialize::FromSql<T, DB>,
        {
            fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
                <$inner>::from_sql(bytes).map(Self)
            }

            fn from_nullable_sql(
                bytes: Option<DB::RawValue<'_>>,
            ) -> diesel::deserialize::Result<Self> {
                <$inner>::from_nullable_sql(bytes).map(Self)
            }
        }

        impl<DB, ST> diesel::deserialize::Queryable<ST, DB> for $hex
        where
            DB: diesel::backend::Backend,
            $inner: diesel::deserialize::Queryable<ST, DB>,
        {
            type Row = <$inner as diesel::deserialize::Queryable<ST, DB>>::Row;

            #[inline]
            fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
                <$inner>::build(row).map(Self)
            }
        }

        impl<'a, T> diesel::expression::AsExpression<T> for &'a $hex
        where
            T: diesel::sql_types::SqlType + diesel::expression::TypedExpressionType,
            &'a $inner: diesel::expression::AsExpression<T>,
        {
            type Expression = <&'a $inner as diesel::expression::AsExpression<T>>::Expression;

            #[inline]
            fn as_expression(self) -> Self::Expression {
                self.get_ref().as_expression()
            }
        }

        impl<T> diesel::expression::AsExpression<T> for $hex
        where
            T: diesel::sql_types::SqlType + diesel::expression::TypedExpressionType,
            $inner: diesel::expression::AsExpression<T>,
        {
            type Expression = <$inner as diesel::expression::AsExpression<T>>::Expression;

            #[inline]
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
            #[cfg(feature = "rand")]
            pub fn rand() -> Self {
                Self::with_rng(&mut rand::thread_rng())
            }

            /// Creates a random hex.
            #[cfg(feature = "rand")]
            pub fn with_rng(rng: &mut impl rand::Rng) -> Self {
                Self(rng.gen())
            }

            /// Gets the interior value.
            #[inline]
            pub const fn get(self) -> $inner {
                self.0
            }

            /// Borrows the interior value.
            #[inline]
            pub const fn get_ref(&self) -> &$inner {
                &self.0
            }

            /// Gets the mutable reference to the interior value.
            #[inline]
            pub const fn get_ref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }

            /// The converse of [`Self::get()`]. This is same as the implementation of [`From`].
            #[inline]
            pub const fn from(n: $inner) -> Self {
                Self(n)
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

        impl std::fmt::Debug for $hex {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl std::fmt::Display for $hex {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.0, f)
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
            #[inline]
            pub fn new(n: $alias) -> Self {
                Self(n)
            }

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
     pub struct $hex: ident ( $alias: ty );) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $hex(std::num::NonZero<$alias>);

        impl $hex {
            #[inline]
            pub fn new(n: $alias) -> Option<Self> {
                <std::num::NonZero<$alias>>::new(n).map(Self)
            }

            /// # Safety
            ///
            /// The value must not be zero.
            #[inline]
            pub unsafe fn new_unchecked(n: $alias) -> Self {
                Self(<std::num::NonZero<$alias>>::new_unchecked(n))
            }

            fn parse_str(s: &str) -> Result<Self, std::num::ParseIntError> {
                <$alias>::from_str_radix(s, 16).and_then(|n| {
                    if n == 0 {
                        Err("0".parse::<std::num::NonZero<$alias>>().unwrap_err())
                    } else {
                        // SAFETY: n is nonzero
                        unsafe { Ok(Self::new_unchecked(n)) }
                    }
                })
            }
        }

        impl TryFrom<$alias> for $hex {
            type Error = <std::num::NonZero<$alias> as TryFrom<$alias>>::Error;

            fn try_from(value: $alias) -> Result<Self, Self::Error> {
                <std::num::NonZero<$alias>>::try_from(value).map(Self)
            }
        }

        _impl_hex_common! { $hex, std::num::NonZero<$alias> }
        #[cfg(feature = "serde")]
        _impl_serde! { $hex }
        #[cfg(feature = "db")]
        _impl_diesel! { $hex, std::num::NonZero<$alias> }
    };
}

#[cfg(test)]
mod test {
    impl_hex! {
        pub struct MyHex(u32);
    }

    impl_nonzero_hex! {
        pub struct MyNonZero(u32);
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
