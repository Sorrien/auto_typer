use rdev::{simulate, EventType, Key, SimulateError};
use std::{env, fs, thread, time};

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let delay_seconds = 3;
        let max_duration = time::Duration::from_secs(delay_seconds);

        let args: Vec<String> = env::args().collect();
        let file_path = parse_config(&args);

        println!("Typing from file {} will commence in {} seconds. Please switch to the window you want to be focused.", file_path, delay_seconds);
        let start_time = time::Instant::now();
        let file_contents = load_file(file_path);

        //only wait up to 2 seconds. subtract the time spent loading the file.
        let now = time::Instant::now();
        let duration_since_start = now - start_time;
        if duration_since_start <= max_duration {
            let duration = max_duration - duration_since_start;
            if duration > time::Duration::ZERO {
                thread::sleep(duration);
            }
        }

        let chars = file_contents.chars();
        let mut events_to_send = Vec::new();
        println!("Typing...");
        chars.for_each(|c| parse_character(c, &mut events_to_send));

        events_to_send.iter().for_each(|event| send(event));

        println!("Typing is complete.")
    }
}

fn parse_character(c: char, events_to_send: &mut Vec<EventType>) {
    match c {
        'a' => push_press_and_release(Key::KeyA, events_to_send),
        'b' => push_press_and_release(Key::KeyB, events_to_send),
        'c' => push_press_and_release(Key::KeyC, events_to_send),
        'd' => push_press_and_release(Key::KeyD, events_to_send),
        'e' => push_press_and_release(Key::KeyE, events_to_send),
        'f' => push_press_and_release(Key::KeyF, events_to_send),
        'g' => push_press_and_release(Key::KeyG, events_to_send),
        'h' => push_press_and_release(Key::KeyH, events_to_send),
        'i' => push_press_and_release(Key::KeyI, events_to_send),
        'j' => push_press_and_release(Key::KeyJ, events_to_send),
        'k' => push_press_and_release(Key::KeyK, events_to_send),
        'l' => push_press_and_release(Key::KeyL, events_to_send),
        'm' => push_press_and_release(Key::KeyM, events_to_send),
        'n' => push_press_and_release(Key::KeyN, events_to_send),
        'o' => push_press_and_release(Key::KeyO, events_to_send),
        'p' => push_press_and_release(Key::KeyP, events_to_send),
        'q' => push_press_and_release(Key::KeyQ, events_to_send),
        'r' => push_press_and_release(Key::KeyR, events_to_send),
        's' => push_press_and_release(Key::KeyS, events_to_send),
        't' => push_press_and_release(Key::KeyT, events_to_send),
        'u' => push_press_and_release(Key::KeyU, events_to_send),
        'v' => push_press_and_release(Key::KeyV, events_to_send),
        'w' => push_press_and_release(Key::KeyW, events_to_send),
        'x' => push_press_and_release(Key::KeyX, events_to_send),
        'y' => push_press_and_release(Key::KeyY, events_to_send),
        'z' => push_press_and_release(Key::KeyZ, events_to_send),

        'A' => capitalize_and_press_and_release(Key::KeyA, events_to_send),
        'B' => capitalize_and_press_and_release(Key::KeyB, events_to_send),
        'C' => capitalize_and_press_and_release(Key::KeyC, events_to_send),
        'D' => capitalize_and_press_and_release(Key::KeyD, events_to_send),
        'E' => capitalize_and_press_and_release(Key::KeyE, events_to_send),
        'F' => capitalize_and_press_and_release(Key::KeyF, events_to_send),
        'G' => capitalize_and_press_and_release(Key::KeyG, events_to_send),
        'H' => capitalize_and_press_and_release(Key::KeyH, events_to_send),
        'I' => capitalize_and_press_and_release(Key::KeyI, events_to_send),
        'J' => capitalize_and_press_and_release(Key::KeyJ, events_to_send),
        'K' => capitalize_and_press_and_release(Key::KeyK, events_to_send),
        'L' => capitalize_and_press_and_release(Key::KeyL, events_to_send),
        'M' => capitalize_and_press_and_release(Key::KeyM, events_to_send),
        'N' => capitalize_and_press_and_release(Key::KeyN, events_to_send),
        'O' => capitalize_and_press_and_release(Key::KeyO, events_to_send),
        'P' => capitalize_and_press_and_release(Key::KeyP, events_to_send),
        'Q' => capitalize_and_press_and_release(Key::KeyQ, events_to_send),
        'R' => capitalize_and_press_and_release(Key::KeyR, events_to_send),
        'S' => capitalize_and_press_and_release(Key::KeyS, events_to_send),
        'T' => capitalize_and_press_and_release(Key::KeyT, events_to_send),
        'U' => capitalize_and_press_and_release(Key::KeyU, events_to_send),
        'V' => capitalize_and_press_and_release(Key::KeyV, events_to_send),
        'W' => capitalize_and_press_and_release(Key::KeyW, events_to_send),
        'X' => capitalize_and_press_and_release(Key::KeyX, events_to_send),
        'Y' => capitalize_and_press_and_release(Key::KeyY, events_to_send),
        'Z' => capitalize_and_press_and_release(Key::KeyZ, events_to_send),

        ' ' => push_press_and_release(Key::Space, events_to_send),
        '\t' => push_press_and_release(Key::Tab, events_to_send),
        '\r' => (), //ignoring returns for now because to a reasonable user the effect is just doubling the number of times enter is pressed
        '\n' => push_press_and_release(Key::Return, events_to_send),
        ',' => push_press_and_release(Key::Comma, events_to_send),
        '.' => push_press_and_release(Key::Dot, events_to_send),
        '\\' => push_press_and_release(Key::BackSlash, events_to_send),
        '/' => push_press_and_release(Key::Slash, events_to_send),
        ';' => push_press_and_release(Key::SemiColon, events_to_send),
        ':' => capitalize_and_press_and_release(Key::SemiColon, events_to_send),
        '?' => capitalize_and_press_and_release(Key::Slash, events_to_send),
        '<' => capitalize_and_press_and_release(Key::Comma, events_to_send),
        '>' => capitalize_and_press_and_release(Key::Dot, events_to_send),

        '0' => push_press_and_release(Key::Num0, events_to_send),
        '1' => push_press_and_release(Key::Num1, events_to_send),
        '2' => push_press_and_release(Key::Num2, events_to_send),
        '3' => push_press_and_release(Key::Num3, events_to_send),
        '4' => push_press_and_release(Key::Num4, events_to_send),
        '5' => push_press_and_release(Key::Num5, events_to_send),
        '6' => push_press_and_release(Key::Num6, events_to_send),
        '7' => push_press_and_release(Key::Num7, events_to_send),
        '8' => push_press_and_release(Key::Num8, events_to_send),
        '9' => push_press_and_release(Key::Num9, events_to_send),

        '!' => capitalize_and_press_and_release(Key::Num1, events_to_send),
        '@' => capitalize_and_press_and_release(Key::Num2, events_to_send),
        '#' => capitalize_and_press_and_release(Key::Num3, events_to_send),
        '$' => capitalize_and_press_and_release(Key::Num4, events_to_send),
        '%' => capitalize_and_press_and_release(Key::Num5, events_to_send),
        '^' => capitalize_and_press_and_release(Key::Num6, events_to_send),
        '&' => capitalize_and_press_and_release(Key::Num7, events_to_send),
        '*' => capitalize_and_press_and_release(Key::Num8, events_to_send),
        '(' => capitalize_and_press_and_release(Key::Num9, events_to_send),
        ')' => capitalize_and_press_and_release(Key::Num0, events_to_send),

        _ => println!("Unsupported character {}", c),
    }
}

fn parse_config(args: &[String]) -> &str {
    if args.len() < 1 {
        panic!("not enough arguments");
    }
    let file_path = &args[1];

    file_path
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

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
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
