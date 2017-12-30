//!  A library for manipulating SMPTE timecodes.

use std::marker::PhantomData;

#[cfg(test)]
mod test;

/// Trait describing frame rates.
pub trait FrameRate {
    fn as_tuple() -> (u16, u16);
}

macro_rules! create_frame_rate {
    ($frame_rate_name:ident, $numerator:expr, $denominator:expr) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            fn as_tuple() -> (u16, u16) { ($numerator, $denominator) }
        }
    )
}

create_frame_rate!(FrameRate24, 24, 1);
create_frame_rate!(FrameRate25, 25, 1);
create_frame_rate!(FrameRate30, 30, 1);
create_frame_rate!(FrameRate50, 50, 1);
create_frame_rate!(FrameRate60, 60, 1);
create_frame_rate!(FrameRate2398, 24000, 1001);
create_frame_rate!(FrameRate2997, 30000, 1001);
create_frame_rate!(FrameRate5994, 60000, 1001);

/// Representation of a timecode
#[derive(Debug, PartialEq)]
pub struct Timecode<FrameRate> {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
    frame_rate_numerator: u16,
    frame_rate_denominator: u16,
    pub drop_frame: bool,
    pub frame_number: u32,
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
    /// assert_eq!(timecode.hour, 10);
    /// assert_eq!(timecode.minute, 0);
    /// assert_eq!(timecode.second, 0);
    /// assert_eq!(timecode.frame, 0);
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

        let (frame_rate_numerator, frame_rate_denominator) = T::as_tuple();

        let frame_number = match get_frame_count(
            hour as u32,
            minute as u32,
            second as u32,
            frame as u32,
            frame_rate_numerator,
            frame_rate_denominator,
        ) {
            Some(frame_number) => frame_number,
            None => return Err("Invalid frame count"),
        };

        let drop_frame = match frame_rate_denominator {
            1 => false,
            1001 => true,
            _ => panic!(),
        };

        Ok(Timecode {
            hour,
            minute,
            second,
            frame,
            frame_rate_numerator,
            frame_rate_denominator,
            drop_frame,
            frame_number,
            frame_rate: PhantomData,
        })
    }
}

fn get_frame_count(
    hour: u32,
    minute: u32,
    second: u32,
    frame: u32,
    frame_rate_numerator: u16,
    frame_rate_denominator: u16,
) -> Option<u32> {
    let fps: f32 = frame_rate_numerator as f32 / frame_rate_denominator as f32;
    let drop_frames_per_minute = (fps * (6.0 / 100.0)).round() as u32;
    let fps_rounded = fps.round() as u32;
    let frames_per_minute = fps_rounded * 60;
    let frames_per_hour = frames_per_minute * 60;
    let minutes = (60 * hour) + minute;

    let frame_number = ((frames_per_hour * hour) + (frames_per_minute * minute)
        + (fps_rounded * second) + frame)
        - (drop_frames_per_minute * (minutes - (minutes / 10)));

    Some(frame_number)
}
