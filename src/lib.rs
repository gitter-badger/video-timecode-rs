//!  A library for manipulating SMPTE timecodes.

use std::marker::PhantomData;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

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

/// Trait describing frame rates.
pub trait FrameRate {
    const FPS: u32;
    const DROP_FRAME: bool;
    const FPS_FLOAT: f32;
    const MAX_FRAMES: u32;

    /// Given the elements of a timecode, calculate the frame offset from zero.
    fn calculate_frame_number(
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
    ) -> Result<u32, &'static str> {
        if hour > 23 {
            return Err("Invalid hour");
        }
        if minute > 59 {
            return Err("Invalid minute");
        }
        if second > 59 {
            return Err("Invalid second");
        }
        if frame as u32 > Self::FPS {
            return Err("Invalid frame");
        }

        let frames_per_minute = Self::FPS * 60;
        let frames_per_hour = frames_per_minute * 60;

        let mut frame_number = (frames_per_hour * hour as u32)
            + (frames_per_minute * minute as u32)
            + (Self::FPS * second as u32)
            + frame as u32;

        if Self::DROP_FRAME {
            let minutes = (60 * hour as u32) + minute as u32;
            let drop_frames_per_minute =
                (Self::FPS_FLOAT * (6.0 / 100.0)).round() as u32;
            frame_number -= drop_frames_per_minute * (minutes - (minutes / 10));
        }

        Ok(frame_number)
    }

    /// Given a frame number, calculate the fields for a time code.
    fn calculate_time_code(mut frame_number: u32) -> (u8, u8, u8, u8) {
        if frame_number > Self::MAX_FRAMES {
            panic!(
                "FrameRate {:?} only supports up to {:?} frames.",
                Self::FPS_FLOAT,
                Self::MAX_FRAMES
            );
        }

        let drop_frames_per_minute = match Self::DROP_FRAME {
            true => (Self::FPS_FLOAT * (6.0 / 100.0)).round() as u32,
            false => 0,
        };

        frame_number %= Self::MAX_FRAMES;

        if Self::DROP_FRAME {
            let frames_per_minute = Self::FPS * 60 - drop_frames_per_minute;
            let frames_per_10_minutes =
                (Self::FPS_FLOAT * 600.0).round() as u32;
            let q = frame_number / frames_per_10_minutes;
            let r = frame_number % frames_per_10_minutes;
            if r > drop_frames_per_minute {
                frame_number += (drop_frames_per_minute * 9 * q)
                    + drop_frames_per_minute
                        * ((r - drop_frames_per_minute) / frames_per_minute);
            } else {
                frame_number += drop_frames_per_minute;
            }
        }

        let frame = frame_number % Self::FPS;
        let second = (frame_number / Self::FPS) % 60;
        let minute = ((frame_number / Self::FPS) / 60) % 60;
        let hour = ((frame_number / Self::FPS) / 60) / 60;

        (hour as u8, minute as u8, second as u8, frame as u8)
    }
}

macro_rules! create_frame_rate {
    ($frame_rate_name:ident, $frame_rate:expr, false) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            const FPS: u32 = $frame_rate;
            const DROP_FRAME: bool = false;
            const FPS_FLOAT: f32 = Self::FPS as f32;
            const MAX_FRAMES: u32 = 86400 * Self::FPS;
        }
    );
    ($frame_rate_name:ident, $frame_rate:expr, true) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            const FPS: u32 = $frame_rate;
            const DROP_FRAME: bool = true;
            const FPS_FLOAT: f32 = ((Self::FPS as f32 * 1000.0) / 1001.0);
            const MAX_FRAMES: u32 = 86400 * Self::FPS
                - 144 * (18 * (Self::FPS / 30));
        }
    );
}

create_frame_rate!(FrameRate24, 24, false);
create_frame_rate!(FrameRate25, 25, false);
create_frame_rate!(FrameRate30, 30, false);
create_frame_rate!(FrameRate50, 50, false);
create_frame_rate!(FrameRate60, 60, false);
create_frame_rate!(FrameRate2398, 24, false);
create_frame_rate!(FrameRate2997, 30, true);
create_frame_rate!(FrameRate5994, 60, true);

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
    frame_rate: PhantomData<FrameRate>,
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
            T::calculate_frame_number(hour, minute, second, frame)?;

        Ok(Timecode {
            frame_number,
            hour,
            minute,
            second,
            frame,
            frame_rate: PhantomData,
        })
    }
}

impl<T> FromStr for Timecode<T>
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

macro_rules! impl_int {
    ($($t:ty)*) => ($(
        impl<T> From<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn from(frame_number: $t) -> Self {
                let mut normalized_frame_number = frame_number;

                #[allow(unused_comparisons)]
                while normalized_frame_number < 0 {
                    normalized_frame_number += T::MAX_FRAMES as $t;
                }

                while normalized_frame_number > T::MAX_FRAMES as $t {
                    normalized_frame_number -= T::MAX_FRAMES as $t;
                }

                let (hour, minute, second, frame) =
                    T::calculate_time_code(normalized_frame_number as u32);

                Timecode {
                    frame_number: normalized_frame_number as u32,
                    hour,
                    minute,
                    second,
                    frame,
                    frame_rate: PhantomData,
                }
            }
        }

        impl<T> Add<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            type Output = Self;

            fn add(self, other: $t) -> Self {
                Timecode::<T>::from(self.frame_number as $t + other)
            }
        }

        impl<T> AddAssign<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn add_assign(&mut self, other: $t) {
                let mut normalized_frame_number = self.frame_number as $t + other;

                #[allow(unused_comparisons)]
                while normalized_frame_number < 0 {
                    normalized_frame_number += T::MAX_FRAMES as $t;
                }

                while normalized_frame_number > T::MAX_FRAMES as $t {
                    normalized_frame_number -= T::MAX_FRAMES as $t;
                }

                let (hour, minute, second, frame) =
                    T::calculate_time_code(normalized_frame_number as u32);

                self.hour = hour;
                self.minute = minute;
                self.second = second;
                self.frame = frame;
                self.frame_number = normalized_frame_number as u32;
            }
        }
    )*)
}
impl_int! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

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
impl<T> Add for Timecode<T>
where
    T: FrameRate,
{
    type Output = Timecode<T>;

    fn add(self, other: Self) -> Self {
        self + other.frame_number
    }
}
