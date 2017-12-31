extern crate video_timecode;

use video_timecode::*;

#[test]
fn new_zero_timecode() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 0, 0)
            .unwrap()
            .frame_number,
        0
    )
}

#[test]
fn new_timecode_frames() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 0, 20)
            .unwrap()
            .frame_number,
        20
    )
}

#[test]
fn new_timecode_seconds() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 0, 40, 10)
            .unwrap()
            .frame_number,
        1210
    )
}

#[test]
fn new_timecode_minutes() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(0, 23, 30, 15)
            .unwrap()
            .frame_number,
        42273
    )
}

#[test]
fn new_timecode_hours() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(5, 15, 25, 12)
            .unwrap()
            .frame_number,
        567194
    )
}
