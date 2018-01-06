extern crate video_timecode;

use std::str::FromStr;

use video_timecode::*;

// Test creating timecodes with various frame rates and durations.
//
// To check which frame number corresponds to which timecode in drop frame frame
// rates, I had this python script write them all out to a file
//
//     i = 0;
//     for h in range(0, 24):
//         for m in range(0, 60):
//             for s in range(0, 60):
//                 for f in range(0, 30):  # 60 for 59.94
//                     if f < 2 and s == 0 and m % 10 != 0:  # f < 4 for 59.94
//                         continue
//                     print("{:02d}:{:02d}:{:02d};{:02d} {}".format(h, m, s, f, i))
//                     i += 1

macro_rules! assert_tc {
    ($tc:expr,
     $hour:expr,
     $minute:expr,
     $second:expr,
     $frame:expr,
     $frame_number:expr) => (
         assert_eq!(
             ($tc.hour, $tc.minute, $tc.second, $tc.frame, $tc.frame_number),
             ($hour, $minute, $second, $frame, $frame_number)
         );
    )
}

macro_rules! test_factories {
    ($name:ident,
     $hour:expr,
     $minute:expr,
     $second:expr,
     $frame:expr ,
     $frame_rate:ident,
     $frame_number:expr) => (
         #[test]
         fn $name() {
            let tc_new = Timecode::<$frame_rate>::new($hour, $minute, $second, $frame)
                .unwrap();
            assert_tc!(tc_new, $hour, $minute, $second, $frame, $frame_number);

            let tc_from = Timecode::<$frame_rate>::from($frame_number);
            assert_tc!(tc_from, $hour, $minute, $second, $frame, $frame_number);
         }
    )
}

test_factories!(frame_number_24_zero, 0, 0, 0, 0, FrameRate24, 0);
test_factories!(frame_number_24_with_frames, 0, 0, 0, 20, FrameRate24, 20);
test_factories!(frame_number_24_with_seconds, 0, 0, 40, 10, FrameRate24, 970);
test_factories!(
    frame_number_24_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate24,
    33855
);
test_factories!(
    frame_number_24_with_hours,
    5,
    15,
    25,
    12,
    FrameRate24,
    454212
);

test_factories!(frame_number_25_zero, 0, 0, 0, 0, FrameRate25, 0);
test_factories!(frame_number_25_with_frames, 0, 0, 0, 23, FrameRate25, 23);
test_factories!(
    frame_number_25_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate25,
    1010
);
test_factories!(
    frame_number_25_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate25,
    35265
);
test_factories!(
    frame_number_25_with_hours,
    5,
    15,
    25,
    12,
    FrameRate25,
    473137
);

test_factories!(frame_number_30_zero, 0, 0, 0, 0, FrameRate30, 0);
test_factories!(frame_number_30_with_frames, 0, 0, 0, 26, FrameRate30, 26);
test_factories!(
    frame_number_30_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate30,
    1210
);
test_factories!(
    frame_number_30_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate30,
    42315
);
test_factories!(
    frame_number_30_with_hours,
    5,
    15,
    25,
    12,
    FrameRate30,
    567762
);

test_factories!(frame_number_50_zero, 0, 0, 0, 0, FrameRate50, 0);
test_factories!(frame_number_50_with_frames, 0, 0, 0, 35, FrameRate50, 35);
test_factories!(
    frame_number_50_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate50,
    2010
);
test_factories!(
    frame_number_50_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate50,
    70515
);
test_factories!(
    frame_number_50_with_hours,
    5,
    15,
    25,
    12,
    FrameRate50,
    946262
);

test_factories!(frame_number_60_zero, 0, 0, 0, 0, FrameRate60, 0);
test_factories!(frame_number_60_with_frames, 0, 0, 0, 45, FrameRate60, 45);
test_factories!(
    frame_number_60_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate60,
    2410
);
test_factories!(
    frame_number_60_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate60,
    84615
);
test_factories!(
    frame_number_60_with_hours,
    23,
    59,
    59,
    23,
    FrameRate60,
    5183963
);

test_factories!(frame_number_2398_zero, 0, 0, 0, 0, FrameRate2398, 0);
test_factories!(
    frame_number_2398_with_frames,
    0,
    0,
    0,
    22,
    FrameRate2398,
    22
);
test_factories!(
    frame_number_2398_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate2398,
    970
);
test_factories!(
    frame_number_2398_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate2398,
    33855
);
test_factories!(
    frame_number_2398_with_hours,
    5,
    15,
    25,
    12,
    FrameRate2398,
    454212
);

test_factories!(frame_number_2997_zero, 0, 0, 0, 0, FrameRate2997, 0);
test_factories!(
    frame_number_2997_with_frames,
    0,
    0,
    0,
    20,
    FrameRate2997,
    20
);
test_factories!(
    frame_number_2997_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate2997,
    1210
);
test_factories!(
    frame_number_2997_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate2997,
    42273
);
test_factories!(
    frame_number_2997_with_hours,
    23,
    23,
    23,
    1,
    FrameRate2997,
    2523565
);

test_factories!(frame_number_5994_zero, 0, 0, 0, 0, FrameRate5994, 0);
test_factories!(
    frame_number_5994_with_frames,
    0,
    0,
    0,
    45,
    FrameRate5994,
    45
);
test_factories!(
    frame_number_5994_with_seconds,
    0,
    0,
    40,
    10,
    FrameRate5994,
    2410
);
test_factories!(
    frame_number_5994_with_minutes,
    0,
    23,
    30,
    15,
    FrameRate5994,
    84531
);
test_factories!(
    frame_number_5994_with_hours,
    5,
    15,
    25,
    12,
    FrameRate5994,
    1134376
);

#[test]
fn dropped_timecode_2997() {
    match Timecode::<FrameRate2997>::new(23, 23, 00, 1) {
        Err(TimecodeError {
            kind: video_timecode::TimecodeErrorKind::InvalidTimecode,
        }) => {}
        _ => panic!(),
    }
}

#[test]
fn dropped_timecode_5994() {
    match Timecode::<FrameRate5994>::new(10, 01, 00, 3) {
        Err(TimecodeError {
            kind: video_timecode::TimecodeErrorKind::InvalidTimecode,
        }) => {}
        _ => panic!(),
    }
}


// Test adding integers to Timecodes

#[test]
fn add_frames_1() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap() + 1usize;
    assert_tc!(tc, 0, 0, 0, 1, 1);
}

#[test]
fn add_frames_500() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap() + 500u32;
    assert_tc!(tc, 0, 0, 20, 20, 500);
}

#[test]
fn add_frames_1000_df() {
    let tc = Timecode::<FrameRate2997>::new(10, 0, 0, 0).unwrap() + 1000u32;
    assert_tc!(tc, 10, 0, 33, 10, 1079920);
}

#[test]
fn add_frames_1000000_df() {
    let tc = Timecode::<FrameRate2997>::new(10, 0, 0, 0).unwrap() + 1000000u32;
    assert_tc!(tc, 19, 16, 6, 22, 2078920);
}

#[test]
fn add_frames_negative_2997() {
    let tc = Timecode::<FrameRate2997>::new(0, 0, 10, 0).unwrap() + (-1000i32);
    assert_tc!(tc, 23, 59, 36, 20, 2588708);
}

#[test]
fn add_frames_negative_5994() {
    let tc = Timecode::<FrameRate5994>::new(0, 0, 10, 0).unwrap() + (-1000i32);
    assert_tc!(tc, 23, 59, 53, 20, 5178416);
}

// Test adding to timecodes with += operator

#[test]
fn add_assign_frames_1() {
    let mut tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap();
    tc += 1usize;
    assert_tc!(tc, 0, 0, 0, 1, 1);
}

#[test]
fn add_assign_frames_500() {
    let mut tc = Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap();
    tc += 500u32;
    assert_tc!(tc, 0, 0, 20, 20, 500);
}

#[test]
fn add_assign_frames_1000_df() {
    let mut tc = Timecode::<FrameRate2997>::new(10, 0, 0, 0).unwrap();
    tc += 1000u32;
    assert_tc!(tc, 10, 0, 33, 10, 1079920);
}

#[test]
fn add_assign_frames_negative_2997() {
    let mut tc = Timecode::<FrameRate2997>::new(0, 0, 10, 0).unwrap();
    tc += -1000i32;
    assert_tc!(tc, 23, 59, 36, 20, 2588708);
}

#[test]
fn add_assign_frames_negative_5994() {
    let mut tc = Timecode::<FrameRate5994>::new(0, 0, 10, 0).unwrap();
    tc += -1000i32;
    assert_tc!(tc, 23, 59, 53, 20, 5178416);
}

// Test adding Timecodes to Timecodes

#[test]
fn add_timecode_1() {
    let tc = Timecode::<FrameRate50>::new(0, 0, 0, 0).unwrap()
        + Timecode::<FrameRate50>::new(0, 0, 0, 1).unwrap();
    assert_eq!(tc, Timecode::<FrameRate50>::new(0, 0, 0, 1).unwrap());
}

#[test]
fn add_timecode_500() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 10).unwrap()
        + Timecode::<FrameRate24>::new(0, 0, 20, 10).unwrap();
    assert_eq!(tc, Timecode::<FrameRate24>::new(0, 0, 20, 20).unwrap());
}

#[test]
fn add_timecode_1000_df() {
    let tc = Timecode::<FrameRate2997>::new(10, 0, 0, 0).unwrap()
        + Timecode::<FrameRate2997>::new(0, 0, 33, 10).unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(10, 0, 33, 10).unwrap());
}

#[test]
fn add_timecode_1000000_df() {
    let tc = Timecode::<FrameRate2997>::new(10, 0, 0, 0).unwrap()
        + Timecode::<FrameRate2997>::new(9, 16, 6, 22).unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(19, 16, 6, 22).unwrap());
}

// Test subtracting integers from Timecodes

#[test]
fn subtract_frames_1() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 0, 10).unwrap() - 1usize;
    assert_eq!(
        (tc.hour, tc.minute, tc.second, tc.frame, tc.frame_number),
        (0, 0, 0, 9, 9)
    );
}

#[test]
fn sub_frames_500() {
    let tc = Timecode::<FrameRate24>::new(0, 0, 20, 20).unwrap() - 500u32;
    assert_tc!(tc, 0, 0, 0, 0, 0);
}

// Test parsing

#[test]
fn parse_0() {
    let tc = Timecode::<FrameRate24>::from_str("00:00:00:00").unwrap();
    assert_eq!(tc, Timecode::<FrameRate24>::new(0, 0, 0, 0).unwrap());
}

#[test]
fn parse_0_df_one_semicolon() {
    let tc = Timecode::<FrameRate2997>::from_str("00:00:00;00").unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap());
}

#[test]
fn parse_0_df_all_semicolon() {
    let tc = Timecode::<FrameRate2997>::from_str("00;00;00;00").unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap());
}

#[test]
fn parse_0_df_one_dot() {
    let tc = Timecode::<FrameRate2997>::from_str("00:00:00.00").unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap());
}

#[test]
fn parse_0_df_all_dot() {
    let tc = Timecode::<FrameRate2997>::from_str("00.00.00.00").unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(0, 0, 0, 0).unwrap());
}

#[test]
fn parse_0_df_mixed() {
    match Timecode::<FrameRate2997>::from_str("00.00:00.00") {
        Err(TimecodeError {
            kind: video_timecode::TimecodeErrorKind::InvalidFormat,
        }) => {}
        _ => panic!(),
    }
}

#[test]
fn parse_0_df_for_ndf_frame_rate() {
    match Timecode::<FrameRate24>::from_str("00:00:00;00") {
        Err(TimecodeError {
            kind: video_timecode::TimecodeErrorKind::InvalidDropFrameFormat,
        }) => {}
        _ => panic!(),
    }
}

#[test]
fn parse_500() {
    let tc = Timecode::<FrameRate24>::from_str("00:00:20:10").unwrap();
    assert_eq!(tc, Timecode::<FrameRate24>::new(0, 0, 20, 10).unwrap());
}

#[test]
fn parse_1000000_df() {
    let tc = Timecode::<FrameRate2997>::from_str("00:00:20:10").unwrap();
    assert_eq!(tc, Timecode::<FrameRate2997>::new(0, 0, 20, 10).unwrap())
}

// Test formatting

#[test]
fn format_ndf() {
    assert_eq!(
        Timecode::<FrameRate50>::new(10, 12, 6, 00)
            .unwrap()
            .to_string(),
        "10:12:06:00"
    );
}

#[test]
fn format_df() {
    assert_eq!(
        Timecode::<FrameRate2997>::new(10, 0, 20, 10)
            .unwrap()
            .to_string(),
        "10:00:20;10"
    );
}
