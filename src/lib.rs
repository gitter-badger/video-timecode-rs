//!  A library for manipulating SMPTE timecodes.

use std::convert::Into;
use std::marker::PhantomData;
use std::ops::Add;

/// Trait describing frame rates.
pub trait FrameRate {
    const DROP_FRAME: bool;
    const FPS_FLOAT: f32;
    const FPS_INT: u32;
    const MAX_FRAMES: u32;

    fn calculate_frame_number(
        hour: u32,
        minute: u32,
        second: u32,
        frame: u32,
    ) -> Option<u32> {
        let frames_per_minute = Self::FPS_INT * 60;
        let frames_per_hour = frames_per_minute * 60;

        let mut frame_number = (frames_per_hour * hour)
            + (frames_per_minute * minute)
            + (Self::FPS_INT * second) + frame;

        if Self::DROP_FRAME {
            let minutes = (60 * hour) + minute;
            let drop_frames_per_minute =
                (Self::FPS_FLOAT * (6.0 / 100.0)).round() as u32;
            frame_number -= drop_frames_per_minute * (minutes - (minutes / 10));
        }

        Some(frame_number)
    }

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
            let frames_per_minute = Self::FPS_INT * 60 - drop_frames_per_minute;
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

        let frame = frame_number % Self::FPS_INT;
        let second = (frame_number / Self::FPS_INT) % 60;
        let minute = ((frame_number / Self::FPS_INT) / 60) % 60;
        let hour = ((frame_number / Self::FPS_INT) / 60) / 60;

        (hour as u8, minute as u8, second as u8, frame as u8)
    }
}

macro_rules! create_frame_rate {
    ($frame_rate_name:ident,
     $frame_rate_int:expr,
     $frame_rate_float:expr,
     $max_frames:expr,
     $drop_frame:expr) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            const DROP_FRAME: bool = $drop_frame;
            const FPS_FLOAT: f32 = $frame_rate_float as f32;
            const FPS_INT: u32 = $frame_rate_int;
            const MAX_FRAMES: u32 = $max_frames;
        }
    )
}

create_frame_rate!(FrameRate24, 24, 24, 86400 * 24, false);
create_frame_rate!(FrameRate25, 25, 25, 86400 * 25, false);
create_frame_rate!(FrameRate30, 30, 30, 86400 * 30, false);
create_frame_rate!(FrameRate50, 50, 50, 86400 * 50, false);
create_frame_rate!(FrameRate60, 60, 60, 86400 * 60, false);
create_frame_rate!(FrameRate2398, 24, 24, 86400 * 24, false);
create_frame_rate!(
    FrameRate2997,
    30,
    30000_f32 / 1001_f32,
    86400 * 30 - 144 * 18,
    true
);
create_frame_rate!(
    FrameRate5994,
    60,
    60000_f32 / 1001_f32,
    86400 * 60 - 144 * 36,
    true
);

/// Representation of a timecode
#[derive(Debug, PartialEq)]
pub struct Timecode<FrameRate> {
    pub frame_number: u32,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
    frame_rate: PhantomData<FrameRate>,
}

impl<T> Timecode<T> {
    /// Returns a timecode with the given properties.
    ///
    /// # Arguments
    ///
    /// * `hour` - Hour part of timecode
    /// * `minute` - Minute part of timecode
    /// * `second` - Second part of timecode
    /// * `frame` - Frame part of timecode
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
    pub fn new<U: Into<u8> + Copy>(
        hour: U,
        minute: U,
        second: U,
        frame: U,
    ) -> Result<Timecode<T>, &'static str>
    where
        T: FrameRate,
    {
        if hour.into() > 23 {
            return Err("Invalid hour");
        }
        if minute.into() > 59 {
            return Err("Invalid minute");
        }
        if second.into() > 59 {
            return Err("Invalid second");
        }

        let frame_number = match T::calculate_frame_number(
            hour.into() as u32,
            minute.into() as u32,
            second.into() as u32,
            frame.into() as u32,
        ) {
            Some(frame_number) => frame_number,
            None => return Err("Invalid frame count"),
        };

        Ok(Timecode {
            frame_number,
            hour: hour.into(),
            minute: minute.into(),
            second: second.into(),
            frame: frame.into(),
            frame_rate: PhantomData,
        })
    }
}

macro_rules! impl_int {
    ($($t:ty)*) => ($(
        impl<T> From<$t> for Timecode<T>
        where
            T: FrameRate,
        {
            fn from(frame_number: $t) -> Timecode<T> {
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
            type Output = Timecode<T>;

            fn add(self, other: $t) -> Timecode<T> {
                Timecode::<T>::from(self.frame_number as $t + other)
            }
        }
    )*)
}
impl_int! { usize u32 u64 isize i32 i64 }
