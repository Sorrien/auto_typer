use rdev::{simulate, EventType, Key, SimulateError};
use std::{env, fs, path::Path, thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_duration = time::Duration::from_secs(2);
    let start_time = time::Instant::now();

    let file_path = &args[1];
    println!("Typing from file {} will commence in 2 seconds. Please switch to the window you want to be focused.", file_path);
    let file_contents = load_file(&file_path);

    //only wait up to 2 seconds. subtract the time spent loading the file.
    let now = time::Instant::now();
    let duration_since_start = now - start_time;
    if duration_since_start <= max_duration {
        let duration = max_duration - duration_since_start;
        if duration > time::Duration::ZERO {
            thread::sleep(duration);
        }
    }

    let mut chars = file_contents.chars();
    let mut events_to_send = Vec::new();
    while let Some(character) = chars.next() {
        match character {
            'a' => push_press_and_release(Key::KeyA, &mut events_to_send),
            'b' => push_press_and_release(Key::KeyB, &mut events_to_send),
            'c' => push_press_and_release(Key::KeyC, &mut events_to_send),
            'd' => push_press_and_release(Key::KeyD, &mut events_to_send),
            'e' => push_press_and_release(Key::KeyE, &mut events_to_send),
            'f' => push_press_and_release(Key::KeyF, &mut events_to_send),
            'g' => push_press_and_release(Key::KeyG, &mut events_to_send),
            'h' => push_press_and_release(Key::KeyH, &mut events_to_send),
            'i' => push_press_and_release(Key::KeyI, &mut events_to_send),
            'j' => push_press_and_release(Key::KeyJ, &mut events_to_send),
            'k' => push_press_and_release(Key::KeyK, &mut events_to_send),
            'l' => push_press_and_release(Key::KeyL, &mut events_to_send),
            'm' => push_press_and_release(Key::KeyM, &mut events_to_send),
            'n' => push_press_and_release(Key::KeyN, &mut events_to_send),
            'o' => push_press_and_release(Key::KeyO, &mut events_to_send),
            'p' => push_press_and_release(Key::KeyP, &mut events_to_send),
            'q' => push_press_and_release(Key::KeyQ, &mut events_to_send),
            'r' => push_press_and_release(Key::KeyR, &mut events_to_send),
            's' => push_press_and_release(Key::KeyS, &mut events_to_send),
            't' => push_press_and_release(Key::KeyT, &mut events_to_send),
            'u' => push_press_and_release(Key::KeyU, &mut events_to_send),
            'v' => push_press_and_release(Key::KeyV, &mut events_to_send),
            'w' => push_press_and_release(Key::KeyW, &mut events_to_send),
            'x' => push_press_and_release(Key::KeyX, &mut events_to_send),
            'y' => push_press_and_release(Key::KeyY, &mut events_to_send),
            'z' => push_press_and_release(Key::KeyZ, &mut events_to_send),

            'A' => capitalize_and_press_and_release(Key::KeyA, &mut events_to_send),
            'B' => capitalize_and_press_and_release(Key::KeyB, &mut events_to_send),
            'C' => capitalize_and_press_and_release(Key::KeyC, &mut events_to_send),
            'D' => capitalize_and_press_and_release(Key::KeyD, &mut events_to_send),
            'E' => capitalize_and_press_and_release(Key::KeyE, &mut events_to_send),
            'F' => capitalize_and_press_and_release(Key::KeyF, &mut events_to_send),
            'G' => capitalize_and_press_and_release(Key::KeyG, &mut events_to_send),
            'H' => capitalize_and_press_and_release(Key::KeyH, &mut events_to_send),
            'I' => capitalize_and_press_and_release(Key::KeyI, &mut events_to_send),
            'J' => capitalize_and_press_and_release(Key::KeyJ, &mut events_to_send),
            'K' => capitalize_and_press_and_release(Key::KeyK, &mut events_to_send),
            'L' => capitalize_and_press_and_release(Key::KeyL, &mut events_to_send),
            'M' => capitalize_and_press_and_release(Key::KeyM, &mut events_to_send),
            'N' => capitalize_and_press_and_release(Key::KeyN, &mut events_to_send),
            'O' => capitalize_and_press_and_release(Key::KeyO, &mut events_to_send),
            'P' => capitalize_and_press_and_release(Key::KeyP, &mut events_to_send),
            'Q' => capitalize_and_press_and_release(Key::KeyQ, &mut events_to_send),
            'R' => capitalize_and_press_and_release(Key::KeyR, &mut events_to_send),
            'S' => capitalize_and_press_and_release(Key::KeyS, &mut events_to_send),
            'T' => capitalize_and_press_and_release(Key::KeyT, &mut events_to_send),
            'U' => capitalize_and_press_and_release(Key::KeyU, &mut events_to_send),
            'V' => capitalize_and_press_and_release(Key::KeyV, &mut events_to_send),
            'W' => capitalize_and_press_and_release(Key::KeyW, &mut events_to_send),
            'X' => capitalize_and_press_and_release(Key::KeyX, &mut events_to_send),
            'Y' => capitalize_and_press_and_release(Key::KeyY, &mut events_to_send),
            'Z' => capitalize_and_press_and_release(Key::KeyZ, &mut events_to_send),

            ' ' => push_press_and_release(Key::Space, &mut events_to_send),
            '\t' => push_press_and_release(Key::Tab, &mut events_to_send),
            //'\r' => push_press_and_release(Key::Return, &mut events_to_send), //ignoring returns for now because to a reasonable user the effect is just doubling the number of times enter is pressed
            '\n' => push_press_and_release(Key::Return, &mut events_to_send),
            ',' => push_press_and_release(Key::Comma, &mut events_to_send),
            '.' => push_press_and_release(Key::Dot, &mut events_to_send),
            '\\' => push_press_and_release(Key::BackSlash, &mut events_to_send),
            '/' => push_press_and_release(Key::Slash, &mut events_to_send),
            ';' => push_press_and_release(Key::SemiColon, &mut events_to_send),
            ':' => capitalize_and_press_and_release(Key::SemiColon, &mut events_to_send),
            '?' => capitalize_and_press_and_release(Key::Slash, &mut events_to_send),
            '<' => capitalize_and_press_and_release(Key::Comma, &mut events_to_send),
            '>' => capitalize_and_press_and_release(Key::Dot, &mut events_to_send),

            '0' => push_press_and_release(Key::Num0, &mut events_to_send),
            '1' => push_press_and_release(Key::Num1, &mut events_to_send),
            '2' => push_press_and_release(Key::Num2, &mut events_to_send),
            '3' => push_press_and_release(Key::Num3, &mut events_to_send),
            '4' => push_press_and_release(Key::Num4, &mut events_to_send),
            '5' => push_press_and_release(Key::Num5, &mut events_to_send),
            '6' => push_press_and_release(Key::Num6, &mut events_to_send),
            '7' => push_press_and_release(Key::Num7, &mut events_to_send),
            '8' => push_press_and_release(Key::Num8, &mut events_to_send),
            '9' => push_press_and_release(Key::Num9, &mut events_to_send),

            '!' => capitalize_and_press_and_release(Key::Num1, &mut events_to_send),
            '@' => capitalize_and_press_and_release(Key::Num2, &mut events_to_send),
            '#' => capitalize_and_press_and_release(Key::Num3, &mut events_to_send),
            '$' => capitalize_and_press_and_release(Key::Num4, &mut events_to_send),
            '%' => capitalize_and_press_and_release(Key::Num5, &mut events_to_send),
            '^' => capitalize_and_press_and_release(Key::Num6, &mut events_to_send),
            '&' => capitalize_and_press_and_release(Key::Num7, &mut events_to_send),
            '*' => capitalize_and_press_and_release(Key::Num8, &mut events_to_send),
            '(' => capitalize_and_press_and_release(Key::Num9, &mut events_to_send),
            ')' => capitalize_and_press_and_release(Key::Num0, &mut events_to_send),

            _ => (),
        }
    }

    events_to_send.iter().for_each(|event| send(event));

    println!("Automatic Typing of file is complete.")
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn load_file(file_path: &String) -> String
/*where
    P: AsRef<Path>,*/
{
    let path = Path::new(file_path);
    println!("Attempting to read file from path: {}", path.display());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

fn push_press_and_release(key: Key, events: &mut Vec<EventType>) {
    events.push(EventType::KeyPress(key));
    events.push(EventType::KeyRelease(key));
}

fn capitalize_and_press_and_release(key: Key, events: &mut Vec<EventType>) {
    shift_press(events);
    events.push(EventType::KeyPress(key));
    events.push(EventType::KeyRelease(key));
    shift_release(events);
}

fn shift_press(events: &mut Vec<EventType>) {
    events.push(EventType::KeyPress(Key::ShiftLeft));
}

fn shift_release(events: &mut Vec<EventType>) {
    events.push(EventType::KeyRelease(Key::ShiftLeft));
}
