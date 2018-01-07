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
/// All [Result](https://doc.rust-lang.org/std/result/enum.Result.html) values
/// returned by this library will use this for error values.
pub struct TimecodeError {
    pub kind: TimecodeErrorKind,
}

#[derive(Debug)]
/// Error values for this library.
pub enum TimecodeErrorKind {
    /// Timecode parsing failed due to input having an invalid format.
    InvalidFormat,

    /// Timecode had an invalid value. For instance the frame field might have
    /// a value higher than the frame rate allows.
    InvalidTimecode
}

/// Representation of a timecode as a struct, generic over types implementing
/// the trait [FrameRate](trait.FrameRate.html).
///
/// **Note**: Currently the user-facing values are open properties. These may
///           be replaced by getters to facilitate lazy evaluation.
///
/// ```
/// use video_timecode::*;
/// use std::str::FromStr;
///
/// let tc1 = Timecode::<FrameRate24>::new(0, 0, 0, 10).unwrap();
/// assert_eq!(tc1.frame_number, 10);
///
/// let tc2 = Timecode::<FrameRate24>::from_str("00:00:10:00").unwrap();
/// assert_eq!(tc2.frame_number, 240);
///
/// let mut tc3 = Timecode::<FrameRate24>::from(240);
/// assert_eq!(tc3.hour, 0);
/// assert_eq!(tc3.minute, 0);
/// assert_eq!(tc3.second, 10);
/// assert_eq!(tc3.frame, 0);
/// assert_eq!(tc3.frame_number, 240);
///
/// tc3 += tc1;
/// assert_eq!(tc3.hour, 0);
/// assert_eq!(tc3.minute, 0);
/// assert_eq!(tc3.second, 10);
/// assert_eq!(tc3.frame, 10);
/// assert_eq!(tc3.frame_number, 250);
/// ```
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
    /// ```
    /// use video_timecode::*;
    ///
    /// let timecode = Timecode::<FrameRate24>::new(10, 0, 0, 0).unwrap();
    /// assert_eq!(timecode.frame_number, 864000);
    /// ```
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
    ) -> Result<Timecode<T>, TimecodeError>
    where
        T: FrameRate,
    {
        use self::TimecodeErrorKind::*;

        let result = T::calculate_frame_number(
            hour as u32,
            minute as u32,
            second as u32,
            frame as u32,
        );

        match result {
            Some(frame_number) => Ok(Timecode {
                frame_number,
                hour,
                minute,
                second,
                frame,
                frame_rate: marker::PhantomData,
            }),
            None => Err(TimecodeError {
                kind: InvalidTimecode,
            }),
        }
    }
}

/// Parse a string into a timecode.
///
/// Colon separator is alright for all types.
///
/// ```
/// use video_timecode::*;
/// use std::str::FromStr;
///
/// let tc1 = Timecode::<FrameRate24>::from_str("00:00:10:00").unwrap();
/// assert_eq!(tc1.frame_number, 240);
///
/// let tc2 = Timecode::<FrameRate2997>::from_str("00:00:10:00").unwrap();
/// assert_eq!(tc2.frame_number, 300);
/// ```
///
/// For frame rates with drop frame, the following formats are also allowed:
///
/// * `00:00:00;00`
/// * `00;00;00;00`
/// * `00.00.00.00`
/// * `00:00:00.00`
///
/// ```
/// use video_timecode::*;
/// use std::str::FromStr;
///
/// let tc1 = Timecode::<FrameRate2997>::from_str("00:00:10;00").unwrap();
/// assert_eq!(tc1.frame_number, 300);
///
/// let tc2 = Timecode::<FrameRate2997>::from_str("00;00;10;00").unwrap();
/// assert_eq!(tc2.frame_number, 300);
///
/// let tc3 = Timecode::<FrameRate2997>::from_str("00:00:10.00").unwrap();
/// assert_eq!(tc3.frame_number, 300);
///
/// let tc4 = Timecode::<FrameRate2997>::from_str("00.00.10.00").unwrap();
/// assert_eq!(tc4.frame_number, 300);
/// ```
impl<T> str::FromStr for Timecode<T>
where
    T: FrameRate,
{
    /// If parsing fails, a timecode error is returned.
    ///
    /// If the input format is invalid in some way, the `TimecodeErrorKind` field
    /// of the [TimecodeError](struct.TimecodeError.html) will be
    /// [InvalidFormat](enum.TimecodeErrorKind.html#variant.InvalidFormat).
    ///
    /// ```
    /// use video_timecode::*;
    /// use video_timecode::TimecodeErrorKind::*;
    /// use std::str::FromStr;
    ///
    /// // Semicolon notation only allowed for drop frame frame rates.
    /// match Timecode::<FrameRate24>::from_str("00:00:10;00") {
    ///     Err(TimecodeError { kind: InvalidFormat }) => {}
    ///     _ => panic!()
    /// }
    /// ```
    ///
    /// If the timecode is not valid for the given frame rate, it will be
    /// [InvalidTimecode](enum.TimecodeErrorKind.html#variant.Timecode).
    ///
    /// ```
    /// use video_timecode::*;
    /// use video_timecode::TimecodeErrorKind::*;
    /// use std::str::FromStr;
    ///
    /// // This is a dropped frame.
    /// match Timecode::<FrameRate2997>::from_str("00:01:00;00") {
    ///     Err(TimecodeError { kind: InvalidTimecode }) => {}
    ///     _ => panic!()
    /// }
    /// ```
    type Err = TimecodeError;

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
                return Err(TimecodeError {
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
                return Err(TimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let minute_string: String = it.by_ref().take(2).collect();
        let minute: u8 = match minute_string.parse() {
            Ok(n) if n < 60 => n,
            _ => {
                return Err(TimecodeError {
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
                return Err(TimecodeError {
                    kind: InvalidFormat,
                });
            }
        }

        let second_string: String = it.by_ref().take(2).collect();
        let second: u8 = match second_string.parse() {
            Ok(n) if n < 60 => n,
            _ => {
                return Err(TimecodeError {
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
                return Err(TimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        let frame_string: String = it.by_ref().take(2).collect();
        let frame: u8 = match frame_string.parse() {
            Ok(n) => n,
            _ => {
                return Err(TimecodeError {
                    kind: InvalidFormat,
                });
            }
        };

        if it.next() != None {
            return Err(TimecodeError {
                kind: InvalidFormat,
            });
        }

        if drop_frame && !T::DROP_FRAME {
            return Err(TimecodeError {
                kind: InvalidFormat,
            });
        }

        match Timecode::<T>::new(hour, minute, second, frame) {
            Ok(timecode) => Ok(timecode),
            Err(_) => Err(TimecodeError {
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
        /// Create a timecode with the the given frame number.
        impl<T> From<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn from(frame_number: $t) -> Self {
                let new_frame_number = frame_number.normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::calculate_time_code(new_frame_number);

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

        /// Make a new timecode by adding a number of frames to a timecode.
        impl<T> ops::Add<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            type Output = Self;

            fn add(self, other: $t) -> Self {
                Timecode::<T>::from(self.frame_number as $t + other)
            }
        }

        /// Add a number of frames to a timecode.
        impl<T> ops::AddAssign<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn add_assign(&mut self, other: $t) {
                let new_frame_number = (self.frame_number as $t + other)
                    .normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::calculate_time_code(new_frame_number);

                self.hour = hour;
                self.minute = minute;
                self.second = second;
                self.frame = frame;
                self.frame_number = new_frame_number;
            }
        }

        /// Make a new timecode by removing a number of frames to a timecode.
        impl<T> ops::Sub<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            type Output = Self;

            fn sub(self, other: $t) -> Self {
                Timecode::<T>::from(self.frame_number as $t - other)
            }
        }

        /// Remove a number of frames from a timecode.
        impl<T> ops::SubAssign<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn sub_assign(&mut self, other: $t) {
                let new_frame_number = (self.frame_number as $t - other)
                    .normalize(T::MAX_FRAMES as $t);

                let (hour, minute, second, frame) =
                    T::calculate_time_code(new_frame_number);

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

/// Make a new timecode by adding two timecodes together. The result is a
/// timecode where the field `frame_number` is the sum of the frame numbers
/// of the two added timecodes.
///
/// ```
/// use video_timecode::*;
///
/// let tc1 = Timecode::<FrameRate24>::new(0, 0, 20, 0).unwrap();
/// let tc2 = Timecode::<FrameRate24>::new(0, 0, 10, 0).unwrap();
/// let tc3 = tc1 + tc2;
/// assert_eq!(tc3, Timecode::<FrameRate24>::new(0, 0, 30, 0).unwrap());
/// ```
///
/// # Adding Timecodes of different frame rates
///
/// Adding timecodes of different framerates together is not supported.
///
/// Since adding Timecodes of different frame rates together normally does not make
/// any sense, it is better that the programmer has to mark this, by explicitly
/// adding the number of frames.
///
/// ```compile_fail
/// use video_timecode::*;
///
/// let tc1 = Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap();
/// let tc2 = Timecode::<FrameRate24>::new(0, 0, 10, 0).unwrap();
/// let tc3 = tc1 + tc2;
/// ```
///
/// # Timecode roll-over
///
/// The timecode (including the `frame_number` field) will roll over when the
/// timecode reaches 24 hours.
///
/// ```
/// use video_timecode::*;
///
/// let tc1 = Timecode::<FrameRate24>::new(23, 59, 30, 0).unwrap();
/// let tc2 = Timecode::<FrameRate24>::new(0, 1, 0, 0).unwrap();
/// let tc3 = tc1 + tc2;
/// assert_eq!(tc3, Timecode::<FrameRate24>::new(0, 0, 30, 0).unwrap());
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

/// Add one timecode to another, of the same frame rate. The first timecode
/// will have a `frame_number` that's the sum of the frame numbers of the
/// two timecodes.
///
/// ```
/// use video_timecode::*;
///
/// let mut tc1 = Timecode::<FrameRate24>::new(0, 0, 10, 0).unwrap();
/// let tc2 = Timecode::<FrameRate24>::new(0, 0, 10, 0).unwrap();
/// tc1 += tc2;
///
/// assert_eq!(tc1, Timecode::<FrameRate24>::new(0, 0, 20, 0).unwrap());
/// ```
impl<T> ops::AddAssign for Timecode<T>
where
    T: FrameRate,
{
    fn add_assign(&mut self, other: Self) {
        *self += other.frame_number;
    }
}
