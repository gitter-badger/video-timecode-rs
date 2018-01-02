//!  A library for manipulating SMPTE timecodes.

use std::fmt;
use std::marker;
use std::ops;
use std::str;

mod frame_rate;

pub use frame_rate::{FrameRate, FrameRate2398, FrameRate24, FrameRate25,
                     FrameRate2997, FrameRate30, FrameRate50, FrameRate5994,
                     FrameRate60};
use frame_rate::NormalizeFrameNumber;

#[derive(Debug)]
pub struct ParseTimecodeError {
    pub kind: TimecodeErrorKind,
}

#[derive(Debug)]
pub enum TimecodeErrorKind {
    InvalidFormat,
    InvalidTimecode,
    InvalidDropFrameFormat,
}

/// Representation of a timecode
///
/// **Note**: Currently the user-facing values are open properties. These may
///           be replaced by getters to facilitate lazy evaluation.
///
/// # Example
///
/// ```
/// use video_timecode::*;
///
/// let timecode = match Timecode::<FrameRate24>::new(0, 0, 0, 10) {
///     Ok(tc) => tc,
///     _ => panic!()
/// };
/// assert_eq!(timecode.frame_number, 10);
///
/// let timecode = match Timecode::<FrameRate24>::new(0, 0, 10, 0) {
///     Ok(tc) => tc,
///     _ => panic!()
/// };
/// assert_eq!(timecode.frame_number, 240);
///
/// let timecode = Timecode::<FrameRate24>::from(240);
/// assert_eq!(timecode.hour, 0);
/// assert_eq!(timecode.minute, 0);
/// assert_eq!(timecode.second, 10);
/// assert_eq!(timecode.frame, 0);
/// assert_eq!(timecode.frame_number, 240);
//// ```
#[derive(Debug, PartialEq)]
pub struct Timecode<FrameRate> {
    /// Frame number. The count of frames after `00:00:00:00`
    pub frame_number: u32,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
    frame_rate: marker::PhantomData<FrameRate>,
}

impl<T> Timecode<T> {
    /// Returns a timecode with the given hour/minute/second/frame fields.
    ///
    /// # Example
    ///
    /// ```
    /// use video_timecode::*;
    ///
    /// let timecode = match Timecode::<FrameRate24>::new(10, 0, 0, 0) {
    ///     Ok(tc) => tc,
    ///     _ => panic!()
    /// };
    ///
    /// assert_eq!(timecode.frame_number, 864000);
    /// ```
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
    ) -> Result<Timecode<T>, &'static str>
    where
        T: FrameRate,
    {
        let frame_number =
            T::_calculate_frame_number(hour, minute, second, frame)?;

        Ok(Timecode {
            frame_number,
            hour,
            minute,
            second,
            frame,
            frame_rate: marker::PhantomData,
        })
    }
}

impl<T> str::FromStr for Timecode<T>
where
    T: FrameRate,
{
    type Err = ParseTimecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::TimecodeErrorKind::*;

        let mut colon_notation = false;
        let mut semi_colon_notation = false;
        let mut dot_notation = false;

        let mut it = s.chars();

        let hour_string: String = it.by_ref().take(2).collect();
        let hour: u8 = match hour_string.parse() {
            Ok(n) if n < 60 => n,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let minute_sep_char = it.next();
        match minute_sep_char {
            Some(':') => colon_notation = true,
            Some(';') => semi_colon_notation = true,
            Some('.') => dot_notation = true,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let minute_string: String = it.by_ref().take(2).collect();
        let minute: u8 = match minute_string.parse() {
            Ok(n) if n < 60 => n,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let second_sep_char = it.next();
        match second_sep_char {
            Some(':') if colon_notation => {}
            Some(';') if semi_colon_notation => {}
            Some('.') if dot_notation => {}
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        }

        let second_string: String = it.by_ref().take(2).collect();
        let second: u8 = match second_string.parse() {
            Ok(n) if n < 60 => n,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let frame_sep_char = it.next();
        let drop_frame = match frame_sep_char {
            Some(':') if colon_notation => false,
            Some(';') if semi_colon_notation || colon_notation => true,
            Some('.') if dot_notation || colon_notation => true,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let frame_string: String = it.by_ref().take(2).collect();
        let frame: u8 = match frame_string.parse() {
            Ok(n) => n,
            _ => {
                return Err(ParseTimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        if it.next() != None {
            return Err(ParseTimecodeError {
                kind: InvalidFormat,
            });
        }

        if drop_frame && !T::DROP_FRAME {
            return Err(ParseTimecodeError {
                kind: InvalidDropFrameFormat,
            });
        }

        match Timecode::<T>::new(hour, minute, second, frame) {
            Ok(timecode) => Ok(timecode),
            Err(_) => Err(ParseTimecodeError {
                kind: InvalidTimecode,
            }),
        }
    }
}

impl<T> fmt::Display for Timecode<T>
where
    T: FrameRate,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = match T::DROP_FRAME {
            true => ';',
            false => ':',
        };

        write!(
            f,
            "{:02}:{:02}:{:02}{}{:02}",
            self.hour, self.minute, self.second, separator, self.frame
        )
    }
}

macro_rules! impl_int_all {
    ($($t:ty)*) => ($(
        impl<T> From<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn from(frame_number: $t) -> Self {
                let new_frame_number = frame_number._normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::_calculate_time_code(new_frame_number);

                Timecode {
                    frame_number: new_frame_number,
                    hour,
                    minute,
                    second,
                    frame,
                    frame_rate: marker::PhantomData,
                }
            }
        }

        impl<T> ops::Add<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            type Output = Self;

            fn add(self, other: $t) -> Self {
                Timecode::<T>::from(self.frame_number as $t + other)
            }
        }

        impl<T> ops::AddAssign<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn add_assign(&mut self, other: $t) {
                let new_frame_number = (self.frame_number as $t + other)
                    ._normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::_calculate_time_code(new_frame_number);

                self.hour = hour;
                self.minute = minute;
                self.second = second;
                self.frame = frame;
                self.frame_number = new_frame_number;
            }
        }

        impl<T> ops::Sub<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            type Output = Self;

            fn sub(self, other: $t) -> Self {
                Timecode::<T>::from(self.frame_number as $t - other)
            }
        }

        impl<T> ops::SubAssign<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn sub_assign(&mut self, other: $t) {
                let new_frame_number = (self.frame_number as $t - other)
                    ._normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::_calculate_time_code(new_frame_number);

                self.hour = hour;
                self.minute = minute;
                self.second = second;
                self.frame = frame;
                self.frame_number = new_frame_number;
            }
        }
    )*)
}
impl_int_all! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

/// Adding timecodes of different framerates together is not supported.
///
/// Since adding Timecodes of different frame rates together normally does not make
/// any sense, it is better that the programmer has to mark this, by explicitly
/// adding the number of frames.
///
/// # Example
///
/// ```
/// use video_timecode::*;
///
/// let tc1 = Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap();
/// let tc2 = Timecode::<FrameRate24>::new(0, 0, 10, 0).unwrap();
/// let tc3 = tc1 + tc2.frame_number;
///
/// assert_eq!(tc3, Timecode::<FrameRate2997>::new(0, 0, 8, 0).unwrap());
/// ```
impl<T> ops::Add for Timecode<T>
where
    T: FrameRate,
{
    type Output = Timecode<T>;

    fn add(self, other: Self) -> Self {
        self + other.frame_number
    }
}
