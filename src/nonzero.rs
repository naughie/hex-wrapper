use crate::macros::exports::*;

impl_nonzero_hex! {
    /// Pointer-sized unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHexUsize;
    ///
    /// // from random value
    /// let rand = NonZeroHexUsize::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHexUsize::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHexUsize>();
    /// assert_eq!(from_str.unwrap(), NonZeroHexUsize::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHexUsize(usize => std::num::NonZeroUsize);
}

impl_nonzero_hex! {
    /// 128-bit unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHex128;
    ///
    /// // from random value
    /// let rand = NonZeroHex128::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHex128::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHex128>();
    /// assert_eq!(from_str.unwrap(), NonZeroHex128::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHex128(u128 => std::num::NonZeroU128);
}

impl_nonzero_hex! {
    /// 64-bit unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHex64;
    ///
    /// // from random value
    /// let rand = NonZeroHex64::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHex64::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHex64>();
    /// assert_eq!(from_str.unwrap(), NonZeroHex64::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHex64(u64 => std::num::NonZeroU64);
}

impl_nonzero_hex! {
    /// 32-bit unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHex32;
    ///
    /// // from random value
    /// let rand = NonZeroHex32::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHex32::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHex32>();
    /// assert_eq!(from_str.unwrap(), NonZeroHex32::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHex32(u32 => std::num::NonZeroU32);
}

impl_nonzero_hex! {
    /// 16-bit unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHex16;
    ///
    /// // from random value
    /// let rand = NonZeroHex16::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHex16::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHex16>();
    /// assert_eq!(from_str.unwrap(), NonZeroHex16::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHex16(u16 => std::num::NonZeroU16);
}

impl_nonzero_hex! {
    /// 8-bit unsigned nonzero hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::NonZeroHex8;
    ///
    /// // from random value
    /// let rand = NonZeroHex8::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = NonZeroHex8::new(0xa3).unwrap();
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<NonZeroHex8>();
    /// assert_eq!(from_str.unwrap(), NonZeroHex8::new(0xa3).unwrap());
    /// ```
    pub struct NonZeroHex8(u8 => std::num::NonZeroU8);
}
