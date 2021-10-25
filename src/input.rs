use std::thread;
use tokio::sync::mpsc::Sender;

#[cfg(not(target_family = "windows"))]
pub fn ler_input(tx: Sender<(bool, u16)>) {
    use evdev::{Device, InputEventKind};
    thread::spawn(move || {
        let mut device = Device::open(format!("/dev/input/event{}", get_keyboard())).unwrap();
        loop {
            for ev in device.fetch_events().unwrap() {
                if let InputEventKind::Key(key) = ev.kind() {
                    if ev.value() != 2 {
                        tx.blocking_send((ev.value() == 1, key.code())).unwrap();
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
