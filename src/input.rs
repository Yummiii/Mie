use std::thread;
use tokio::sync::mpsc::Sender;

#[cfg(not(target_family = "windows"))]
pub fn ler_input(tx: Sender<(bool, u16, bool)>) {
    use evdev::{Device, InputEventKind};
    thread::spawn(move || {
        let mut device = Device::open(format!("/dev/input/event{}", get_keyboard())).unwrap();
        loop {
            for ev in device.fetch_events().unwrap() {
                if let InputEventKind::Key(key) = ev.kind() {
                    if ev.value() != 2 {
                        let scan_code = get_arduino_scancodes(key.code());
                        tx.blocking_send((ev.value() == 1, scan_code.0, scan_code.1)).unwrap();
                        println!("{:?} =-= {}", key, ev.value());
                    }
                }
            }
        }
    });
}

#[cfg(not(target_family = "windows"))]
pub fn get_keyboard() -> i32 {
    use run_script::ScriptOptions;
    let (code, output, _) = run_script::run(r#"grep -E 'Handlers|EV=' /proc/bus/input/devices | grep -B1 'EV=120013' | grep -Eo 'event[0-9]+' | grep -Eo '[0-9]+' | tr -d '\n'"#, &vec![], &ScriptOptions::new()).unwrap();
    if code == 0 {
        output.parse::<i32>().unwrap()
    } else {
        panic!("fudeu colega")
    }
}

pub fn get_arduino_scancodes(code: u16) -> (u16, bool) {
    match code {
        1 => (0x76, false),
        59 => (0x05, false),
        60 => (0x06, false),
        61 => (0x04, false),
        62 => (0x0c, false),
        63 => (0x03, false),
        64 => (0x0b, false),
        65 => (0x83, false),
        66 => (0x0a, false),
        67 => (0x01, false),
        68 => (0x09, false),
        87 => (0x78, false),
        88 => (0x07, false),
        70 => (0x7e, false),
        41 => (0x0e, false),
        2 => (0x16, false),
        3 => (0x1e, false),
        4 => (0x26, false),
        5 => (0x25, false),
        6 => (0x2e, false),
        7 => (0x36, false),
        8 => (0x3d, false),
        9 => (0x3e, false),
        10 => (0x46, false),
        11 => (0x44, false),
		12 => (0x4e, false),
		13 => (0x55, false),
		14 => (0x66, false),
        15 => (0x0d, false),
        16 => (0x15, false),
        17 => (0x1d, false),
        18 => (0x24, false),
        19 => (0x2d, false),
        20 => (0x2c, false),
        21 => (0x35, false),
        22 => (0x3c, false),
        23 => (0x43, false),
        24 => (0x44, false),
        25 => (0x4d, false),
        26 => (0x54, false), 
        27 => (0x5b, false),
        43 => (0x5d, false),
        58 => (0x58, false),
        30 => (0x1c, false),
        31 => (0x1b, false),
        32 => (0x23, false),
        33 => (0x2b, false),
        34 => (0x34, false),
        35 => (0x33, false),
        36 => (0x3b, false),
        37 => (0x42, false),
        38 => (0x4b, false),
        39 => (0x4c, false),
        40 => (0x52, false),
        28 => (0x5a, false),
        42 => (0x12, false),
        44 => (0x1a, false),
        45 => (0x22, false),
        46 => (0x21, false),
        47 => (0x2a, false),
        48 => (0x32, false),
        49 => (0x31, false),
        50 => (0x3a, false),
        51 => (0x41, false),
        52 => (0x49, false),
        53 => (0x4a, false),
        54 => (0x59, false),
        29 => (0x14, false),
        56 => (0x11, false),
        57 => (0x29, false),
        69 => (0x77, false),
        55 => (0x7c, false),
        74 => (0x7b, false),
        71 => (0x6c ,false),
        72 => (0x75, false),
        73 => (0x7d, false),
        78 => (0x79, false),
        75 => (0x6b, false),
        76 => (0x73, false),
        77 => (0x74, false),
        79 => (0x69, false),
        80 => (0x72, false),
        81 => (0x7a, false),
        82 => (0x70, false),
        83 => (0x71, false),
        100 => (0x11, true),
        183 => (0x2f, true),
        97 => (0x14, true),
        110 => (0x70, true),
        102 => (0x6c, true),
        104 => (0x7d, true),
        111 => (0x71, true),
        107 => (0x69, true),
        109 => (0x7a, true),
        _ => (0, false)
    }
}