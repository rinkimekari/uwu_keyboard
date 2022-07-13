use rdev::{
    listen, simulate, Event, EventType,
    Key::{
        Alt, AltGr, Backspace, ControlLeft, ControlRight, KeyL, KeyR, KeyW,
        MetaLeft, MetaRight, ShiftLeft,
    },
};

static mut NO_MODS: u8 = 0;

fn main() {
    listen(callback).unwrap();
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            ControlLeft | ControlRight | MetaLeft | MetaRight | Alt | AltGr => unsafe {
                NO_MODS += 1;
            },
            KeyR | KeyL => {
                if unsafe { NO_MODS == 0 } {
                    replace(&event.name.as_ref().unwrap());
                }
            }
            _ => {}
        },
        EventType::KeyRelease(key) => match key {
            ControlLeft | ControlRight | MetaLeft | MetaRight | Alt | AltGr => unsafe {
                NO_MODS -= 1;
            },
            _ => {}
        },
        _ => {}
    }
}

// get rid of this function if delay isnt needed for MacOS
fn send(event_type: &EventType) {
    simulate(event_type).unwrap();

    // Potentially remove if unnecessary
    #[cfg(target_os = "macos")]
    thread::sleep(Duration::from_millis(20));
}

fn replace(c: &String) {
    send(&EventType::KeyPress(Backspace));
    send(&EventType::KeyRelease(Backspace));

    if c == "R" || c == "L" {
        send(&EventType::KeyPress(ShiftLeft));
        send(&EventType::KeyPress(KeyW));
        send(&EventType::KeyRelease(KeyW));
        // send(&EventType::KeyRelease(ShiftLeft)); // just trust me bro
    } else if c == "r" || c == "l" {
        send(&EventType::KeyPress(KeyW));
        send(&EventType::KeyRelease(KeyW));
    }
}
