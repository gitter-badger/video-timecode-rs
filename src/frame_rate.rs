/// Trait describing frame rates.
pub trait FrameRate {
    const FPS: u32;
    const DROP_FRAME: bool;

    #[doc(hidden)]
    const MAX_FRAMES: u32;

    #[doc(hidden)]
    const FPS_FLOAT: f32;

    /// Given the elements of a timecode, calculate the frame offset from zero.
    #[doc(hidden)]
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

        // TODO: Replace this stuff with an integer based method.
        if Self::DROP_FRAME {
            let minutes = (60 * hour as u32) + minute as u32;
            let drop_frames_per_minute =
                (Self::FPS_FLOAT * (6.0 / 100.0)).round() as u32;
            frame_number -= drop_frames_per_minute * (minutes - (minutes / 10));
        }

        Ok(frame_number)
    }

    /// Given a frame number, calculate the fields for a time code.
    #[doc(hidden)]
    fn calculate_time_code(frame_number: u32) -> (u8, u8, u8, u8) {
        if frame_number > Self::MAX_FRAMES {
            panic!(
                "FrameRate {:?} only supports up to {:?} frames.",
                Self::FPS_FLOAT,
                Self::MAX_FRAMES
            );
        }

        let (hour, minute, second, frame) = if Self::DROP_FRAME {
            let drop_frames_each_time = (Self::FPS) / 15;

            let frames_per_minute = Self::FPS * 60;
            let frames_per_drop_minute =
                frames_per_minute - drop_frames_each_time;
            let frames_per_ten =
                frames_per_minute + (frames_per_drop_minute * 9);
            let frames_per_hour = frames_per_ten * 6;

            let hour = frame_number / frames_per_hour;

            let frame_number_without_hours = frame_number % frames_per_hour;
            let tens = frame_number_without_hours / frames_per_ten;
            let frame_number_without_tens =
                frame_number_without_hours % frames_per_ten;

            let (minute, second, frame) = if frame_number_without_tens
                < frames_per_minute
            {
                let minute = tens * 10;
                let frame_number_without_minutes = frame_number_without_tens;

                let second = frame_number_without_minutes / Self::FPS;

                let frame = frame_number_without_minutes % Self::FPS;

                (minute, second, frame)
            } else {
                let frame_number_without_first_minute =
                    frame_number_without_tens - frames_per_minute;

                let minute = 1
                    + (frame_number_without_first_minute
                        / frames_per_drop_minute);

                let frame_number_without_minutes =
                    (frame_number_without_first_minute % frames_per_drop_minute)
                        + drop_frames_each_time;

                let second = frame_number_without_minutes / Self::FPS;

                let frame = frame_number_without_minutes % Self::FPS;

                (tens * 10 + minute, second, frame)
            };

            (hour, minute, second, frame)
        } else {
            let total_seconds = frame_number / Self::FPS;
            let total_minutes = total_seconds / 60;

            let hour = total_minutes / 60;
            let minute = total_minutes % 60;
            let second = total_seconds % 60;
            let frame = frame_number % Self::FPS;

            (hour, minute, second, frame)
        };

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

pub trait NormalizeFrameNumber<T> {
    fn normalize(self, max_frames: T) -> u32;
}

macro_rules! impl_int_unsigned {
    ($($t:ty)*) => ($(
        impl NormalizeFrameNumber<$t> for $t {
            fn normalize(self, max_frames: $t) -> u32 {
                (self % max_frames) as u32
            }
        }
    )*)
}
impl_int_unsigned! { usize u8 u16 u32 u64 }

macro_rules! impl_int_signed {
    ($($t:ty)*) => ($(
        impl NormalizeFrameNumber<$t> for $t {
            fn normalize(self, max_frames: $t) -> u32 {
                let remainder = self % max_frames;

                let result = if remainder < 0 {
                    remainder + max_frames
                } else {
                    remainder
                };

                result as u32
            }
        }
    )*)
}
impl_int_signed! { isize i8 i16 i32 i64 }
