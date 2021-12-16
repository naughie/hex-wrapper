use crate::macros::exports::*;

impl_hex! {
    /// Pointer-sized unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::HexUsize;
    ///
    /// // from random value
    /// let rand = HexUsize::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = HexUsize::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<HexUsize>();
    /// assert_eq!(from_str.unwrap(), HexUsize::from(0xa3));
    /// ```
    pub struct HexUsize(usize);
}

impl_hex! {
    /// 128-bit unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::Hex128;
    ///
    /// // from random value
    /// let rand = Hex128::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = Hex128::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<Hex128>();
    /// assert_eq!(from_str.unwrap(), Hex128::from(0xa3));
    /// ```
    pub struct Hex128(u128);
}

impl_hex! {
    /// 64-bit unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::Hex64;
    ///
    /// // from random value
    /// let rand = Hex64::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = Hex64::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<Hex64>();
    /// assert_eq!(from_str.unwrap(), Hex64::from(0xa3));
    /// ```
    pub struct Hex64(u64);
}

impl_hex! {
    /// 32-bit unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::Hex32;
    ///
    /// // from random value
    /// let rand = Hex32::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = Hex32::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<Hex32>();
    /// assert_eq!(from_str.unwrap(), Hex32::from(0xa3));
    /// ```
    pub struct Hex32(u32);
}

impl_hex! {
    /// 16-bit unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::Hex16;
    ///
    /// // from random value
    /// let rand = Hex16::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = Hex16::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<Hex16>();
    /// assert_eq!(from_str.unwrap(), Hex16::from(0xa3));
    /// ```
    pub struct Hex16(u16);
}

impl_hex! {
    /// 8-bit unsigned hexadecimal numbers.
    ///
    /// ```rust
    /// use hex_wrapper::Hex8;
    ///
    /// // from random value
    /// let rand = Hex8::rand();
    /// let inner = rand.get();
    /// assert_eq!(rand.to_string(), format!("{:x}", inner));
    ///
    /// // from given value
    /// let given = Hex8::from(0xa3);
    /// assert_eq!(given.to_string(), String::from("a3"));
    ///
    /// // from string
    /// let from_str = "a3".parse::<Hex8>();
    /// assert_eq!(from_str.unwrap(), Hex8::from(0xa3));
    /// ```
    pub struct Hex8(u8);
}
