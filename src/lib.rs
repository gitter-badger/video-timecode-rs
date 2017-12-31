//!  A library for manipulating SMPTE timecodes.

use std::marker::PhantomData;

/// Trait describing frame rates.
pub trait FrameRate {
    const DROP_FRAME: bool;
    const FPS_FLOAT: f32;
    const FPS_INT: u32;

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
}

macro_rules! create_frame_rate {
    ($frame_rate_name:ident,
     $frame_rate_int:expr,
     $frame_rate_float:expr,
     $drop_frame:expr) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            const DROP_FRAME: bool = $drop_frame;
            const FPS_FLOAT: f32 = $frame_rate_float as f32;
            const FPS_INT: u32 = $frame_rate_int;
        }
    )
}

create_frame_rate!(FrameRate24, 24, 24, false);
create_frame_rate!(FrameRate25, 25, 25, false);
create_frame_rate!(FrameRate30, 30, 30, false);
create_frame_rate!(FrameRate50, 50, 50, false);
create_frame_rate!(FrameRate60, 60, 60, false);
create_frame_rate!(FrameRate2398, 24, 24, false);
create_frame_rate!(
    FrameRate2997,
    30,
    29.97002997002997002997002997002997002997002997002997002997002997002,
    true
);
create_frame_rate!(
    FrameRate5994,
    60,
    59.94005994005994005994005994005994005994005994005994005994005994005,
    true
);

/// Representation of a timecode
#[derive(Debug, PartialEq)]
pub struct Timecode<FrameRate> {
    pub frame_number: u32,
    pub drop_frame: bool,
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
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
    ) -> Result<Timecode<T>, &'static str>
    where
        T: FrameRate,
    {
        if hour > 23 {
            return Err("Invalid hour");
        }
        if minute > 59 {
            return Err("Invalid minute");
        }
        if second > 59 {
            return Err("Invalid second");
        }

        let frame_number = match T::calculate_frame_number(
            hour as u32,
            minute as u32,
            second as u32,
            frame as u32,
        ) {
            Some(frame_number) => frame_number,
            None => return Err("Invalid frame count"),
        };

        Ok(Timecode {
            frame_number,
            hour,
            minute,
            second,
            frame,
            drop_frame: T::DROP_FRAME,
            frame_rate: PhantomData,
        })
    }
}
