extern crate video_timecode;

use video_timecode::*;

// Test creating timecodes with various frame rates and durations.

macro_rules! test_frame_number {
    ($name:ident,
     $hour:expr,
     $minute:expr,
     $second:expr,
     $frame:expr ,
     $frame_rate:ident,
     $frame_number:expr) => (
         #[test]
         fn $name() {
            assert_eq!(
                Timecode::<$frame_rate>::new($hour,
                                             $minute,
                                             $second,
                                             $frame)
                    .unwrap()
                    .frame_number,
                $frame_number
            )
         }
    )
}

test_frame_number!(test_frame_number_24_zero, 0, 0, 0, 0, FrameRate24, 0);
test_frame_number!(
    test_frame_number_24_with_frames,
    0,
    0,
    0,
    20,
    FrameRate24,
    20
);
test_frame_number!(
    test_frame_number_24_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate24,
    970
);
test_frame_number!(
    test_frame_number_24_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate24,
    33855
);
test_frame_number!(
    test_frame_number_24_with_hours,
    5,
    15,
    25,
    12,
    FrameRate24,
    454212
);

test_frame_number!(test_frame_number_25_zero, 0, 0, 0, 0, FrameRate25, 0);
test_frame_number!(
    test_frame_number_25_with_frames,
    0,
    0,
    0,
    23,
    FrameRate25,
    23
);
test_frame_number!(
    test_frame_number_25_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate25,
    1010
);
test_frame_number!(
    test_frame_number_25_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate25,
    35265
);
test_frame_number!(
    test_frame_number_25_with_hours,
    5,
    15,
    25,
    12,
    FrameRate25,
    473137
);

test_frame_number!(test_frame_number_30_zero, 0, 0, 0, 0, FrameRate30, 0);
test_frame_number!(
    test_frame_number_30_with_frames,
    0,
    0,
    0,
    26,
    FrameRate30,
    26
);
test_frame_number!(
    test_frame_number_30_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate30,
    1210
);
test_frame_number!(
    test_frame_number_30_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate30,
    42315
);
test_frame_number!(
    test_frame_number_30_with_hours,
    5,
    15,
    25,
    12,
    FrameRate30,
    567762
);

test_frame_number!(test_frame_number_50_zero, 0, 0, 0, 0, FrameRate50, 0);
test_frame_number!(
    test_frame_number_50_with_frames,
    0,
    0,
    0,
    35,
    FrameRate50,
    35
);
test_frame_number!(
    test_frame_number_50_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate50,
    2010
);
test_frame_number!(
    test_frame_number_50_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate50,
    70515
);
test_frame_number!(
    test_frame_number_50_with_hours,
    5,
    15,
    25,
    12,
    FrameRate50,
    946262
);

test_frame_number!(test_frame_number_60_zero, 0, 0, 0, 0, FrameRate60, 0);
test_frame_number!(
    test_frame_number_60_with_frames,
    0,
    0,
    0,
    45,
    FrameRate60,
    45
);
test_frame_number!(
    test_frame_number_60_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate60,
    2410
);
test_frame_number!(
    test_frame_number_60_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate60,
    84615
);
test_frame_number!(
    test_frame_number_60_with_hours,
    23,
    59,
    59,
    23,
    FrameRate60,
    5183963
);

test_frame_number!(test_frame_number_2398_zero, 0, 0, 0, 0, FrameRate2398, 0);
test_frame_number!(
    test_frame_number_2398_with_frames,
    0,
    0,
    0,
    22,
    FrameRate2398,
    22
);
test_frame_number!(
    test_frame_number_2398_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate2398,
    970
);
test_frame_number!(
    test_frame_number_2398_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate2398,
    33855
);
test_frame_number!(
    test_frame_number_2398_with_hours,
    5,
    15,
    25,
    12,
    FrameRate2398,
    454212
);

test_frame_number!(test_frame_number_2997_zero, 0, 0, 0, 0, FrameRate2997, 0);
test_frame_number!(
    test_frame_number_2997_with_frames,
    0,
    0,
    0,
    20,
    FrameRate2997,
    20
);
test_frame_number!(
    test_frame_number_2997_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate2997,
    1210
);
test_frame_number!(
    test_frame_number_2997_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate2997,
    42273
);
test_frame_number!(
    test_frame_number_2997_with_hours,
    5,
    15,
    25,
    12,
    FrameRate2997,
    567194
);

test_frame_number!(test_frame_number_5994_zero, 0, 0, 0, 0, FrameRate5994, 0);
test_frame_number!(
    test_frame_number_5994_with_frames,
    0,
    0,
    0,
    45,
    FrameRate5994,
    45
);
test_frame_number!(
    test_frame_number_5994_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate5994,
    2410
);
test_frame_number!(
    test_frame_number_5994_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate5994,
    84531
);
test_frame_number!(
    test_frame_number_5994_with_hours,
    5,
    15,
    25,
    12,
    FrameRate5994,
    1134376
);

// Test adding integers to Timecodes

#[test]
fn test_add_1() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap() + 1usize;
    assert_eq!(tc.hour, 0);
    assert_eq!(tc.minute, 0);
    assert_eq!(tc.second, 0);
    assert_eq!(tc.frame, 1);
    assert_eq!(tc.frame_number, 1);
}

#[test]
fn test_add_500() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap() + 500u32;
    assert_eq!(tc.hour, 0);
    assert_eq!(tc.minute, 0);
    assert_eq!(tc.second, 20);
    assert_eq!(tc.frame, 20);
    assert_eq!(tc.frame_number, 500);
}
