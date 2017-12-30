//! Tests that require access to internal values.

use super::*;

macro_rules! test_timecode {
    ($hour:expr,
     $minute:expr,
     $second:expr,
     $frame:expr,
     $numerator:expr,
     $denominator:expr,
     $drop_frame:expr,
     $frame_number:expr
     ) => (
        Timecode {
            hour: $hour,
            minute: $minute,
            second: $second,
            frame: $frame,
            frame_rate_numerator: $numerator,
            frame_rate_denominator: $denominator,
            drop_frame: $drop_frame,
            frame_number: $frame_number,
            frame_rate: PhantomData
         }
    )
}

#[test]
fn new_zero_timecode() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap(),
        test_timecode!(0, 0, 0, 0, 30000, 1001, true, 0)
    )
}

#[test]
fn new_timecode_frames() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 0, 20).unwrap(),
        test_timecode!(0, 0, 0, 20, 30000, 1001, true, 20)
    )
}

#[test]
fn new_timecode_seconds() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 40, 10).unwrap(),
        test_timecode!(0, 0, 40, 10, 30000, 1001, true, 1210)
    )
}

#[test]
fn new_timecode_minutes() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 23, 30, 15).unwrap(),
        test_timecode!(0, 23, 30, 15, 30000, 1001, true, 42273)
    )
}

#[test]
fn new_timecode_hours() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(5, 15, 25, 12).unwrap(),
        test_timecode!(5, 15, 25, 12, 30000, 1001, true, 567194)
    )
}
