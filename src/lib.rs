//! Don't let `unsafe` give the impression that your code is somehow "bad" or "dangerous",
//! you're a programmer goddammit, and you know what you're doing!

/// Do you want to do something that the haters would probably say is "foolish"
/// or "reckless"? Just tell the compiler to hold your beer for you as you
/// prove the haters wrong!
///
/// ```
/// # #[macro_use] extern crate hold_my_beer;
/// #[derive(Debug, PartialEq)]
/// #[repr(u32)]
/// enum AmazingEnum {
///     One = 1,
///     FortyTwo = 42,
///     // TODO: Other very important numbers
/// }
///
/// impl From<u32> for AmazingEnum {
///     fn from(val: u32) -> Self {
///         // Is taking untrusted input dangerous? Maybe.
///         hold_my_beer!(std::mem::transmute(val))
///     }
/// }
///
/// assert_eq!(AmazingEnum::FortyTwo, AmazingEnum::from(42));
/// ```
#[macro_export]
macro_rules! hold_my_beer {
    ($something_potentially_bad:expr) => {
        unsafe { $something_potentially_bad }
    };
}

/// Rustc doesn't like your code? Sometimes friends disagree, and that's ok.
/// The important thing is that you absolutely know this code will work,
/// you just need to convince rustc to try it out! Nothing bad could happen!
///
/// ```
/// # #[macro_use] extern crate hold_my_beer;
///
/// pub fn to_hex<'a>(input: &[u8], output: &'a mut [u8]) -> Option<&'a str> {
///     use std::str;
///     const CHARS: &[u8] = b"0123456789abcdef";
///
///     if output.len() < input.len() * 2 {
///         return None;
///     }
///
///     let mut ind = 0;
///
///     for &byte in input {
///         output[ind] = CHARS[(byte >> 4) as usize];
///         output[ind + 1] = CHARS[(byte & 0xf) as usize];
///
///         ind += 2;
///     }
///
///     i_got_this!(Some(str::from_utf8_unchecked(&output[0..input.len() * 2])))
/// }
///
/// let input = [0xffu8; 20];
/// let mut output = [0u8; 40];
/// assert_eq!(to_hex(&input, &mut output), Some("ffffffffffffffffffffffffffffffffffffffff"));
/// ```
#[macro_export]
macro_rules! i_got_this {
    ($something_potentially_bad:expr) => {
        unsafe { $something_potentially_bad }
    };
}

/// Somtimes, leaps of faith are needed to achieve true greatness.
/// It's even more impressive if you do a leap of faith in a car,
/// but why not take advantage of the safety features available,
/// and `buckle_up!`?
///
/// ```
/// # #[macro_use] extern crate hold_my_beer;
/// pub fn parse(arbitrary_user_input_from_the_wilds: &[u8]) -> u32 {
///     buckle_up!({
///         std::str::from_utf8_unchecked(arbitrary_user_input_from_the_wilds).parse::<u32>().unwrap()
///     })
/// }
///
/// let input = b"12345";
/// assert_eq!(parse(input), 12345);
/// ```
#[macro_export]
macro_rules! buckle_up {
    ($something_potentially_bad:expr) => {
        unsafe { $something_potentially_bad }
    };
}
