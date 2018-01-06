/// Trait describing frame rates.
pub trait FrameRate {
    const FPS: u32;

    const DROP_FRAME: bool;

    #[doc(hidden)]
    const MAX_FRAMES: u32;

    #[doc(hidden)]
    const FRAMES_PER_MINUTE: u32 = Self::FPS * 60;

    #[doc(hidden)]
    const FRAMES_PER_HOUR: u32 = Self::FRAMES_PER_MINUTE * 60;

    #[doc(hidden)]
    const DROP_FRAME_COUNT: u32 = Self::FPS / 15;

    /// Given the elements of a timecode, calculate the frame offset from zero.
    #[doc(hidden)]
    fn calculate_frame_number(
        hour: u32,
        minute: u32,
        second: u32,
        frame: u32,
    ) -> Option<u32> {
        if hour > 23 || minute > 59 || second > 59 || frame > Self::FPS
            || (Self::DROP_FRAME && second == 0 && minute % 10 != 0
                && frame < Self::DROP_FRAME_COUNT)
        {
            return None;
        }

        let frame_number_before_drop_frames =
            (Self::FRAMES_PER_HOUR * hour as u32)
                + (Self::FRAMES_PER_MINUTE * minute as u32)
                + (Self::FPS * second as u32) + frame as u32;

        let frame_number = if Self::DROP_FRAME {
            let tens = hour * 6 + minute / 10;
            let minutes_without_tens = minute % 10;
            let drop_frame_minutes_without_tens = if minutes_without_tens > 1 {
                minutes_without_tens
            } else {
                0
            };
            let drop_frames_per_ten = Self::DROP_FRAME_COUNT * 9;
            frame_number_before_drop_frames - (tens * drop_frames_per_ten)
                - drop_frame_minutes_without_tens * Self::DROP_FRAME_COUNT
        } else {
            frame_number_before_drop_frames
        };

        Some(frame_number)
    }

    /// Given a frame number, calculate the fields for a time code.
    #[doc(hidden)]
    fn calculate_time_code(frame_number: u32) -> (u8, u8, u8, u8) {
        if frame_number > Self::MAX_FRAMES {
            panic!(
                "Frame rate only supports up to {:?} frames.",
                Self::MAX_FRAMES
            );
        }

        let (hour, minute, second, frame) = if Self::DROP_FRAME {
            let frames_per_drop_minute =
                Self::FRAMES_PER_MINUTE - Self::DROP_FRAME_COUNT;
            let frames_per_ten =
                Self::FRAMES_PER_MINUTE + (frames_per_drop_minute * 9);
            let frames_per_hour = frames_per_ten * 6;

            let hour = frame_number / frames_per_hour;

            let frame_number_without_hours = frame_number % frames_per_hour;
            let tens = frame_number_without_hours / frames_per_ten;
            let frame_number_without_tens =
                frame_number_without_hours % frames_per_ten;

            let (minute, second, frame) = if frame_number_without_tens
                < Self::FRAMES_PER_MINUTE
            {
                let minute = tens * 10;
                let frame_number_without_minutes = frame_number_without_tens;

                let second = frame_number_without_minutes / Self::FPS;

                let frame = frame_number_without_minutes % Self::FPS;

                (minute, second, frame)
            } else {
                let frame_number_without_first_minute =
                    frame_number_without_tens - Self::FRAMES_PER_MINUTE;

                let minute = 1
                    + (frame_number_without_first_minute
                        / frames_per_drop_minute);

                let frame_number_without_minutes =
                    (frame_number_without_first_minute % frames_per_drop_minute)
                        + Self::DROP_FRAME_COUNT;

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
            const MAX_FRAMES: u32 = 86400 * Self::FPS;
        }
    );
    ($frame_rate_name:ident, $frame_rate:expr, true) => (
        #[derive(Debug, PartialEq)]
        pub struct $frame_rate_name;

        impl FrameRate for $frame_rate_name {
            const FPS: u32 = $frame_rate;
            const DROP_FRAME: bool = true;
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
