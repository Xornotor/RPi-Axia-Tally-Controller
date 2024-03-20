pub fn parse_event(event: &String) -> Option<(u8, u8, bool)> {
    let event_parts: Vec<&str> = event.split(|c: char| c == '=' || c == ' ').collect();

    let console_number: u8 = match event_parts[1].trim().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    let mut fader_number: u8 = 0;
    let mut state: bool = false;

    if event_parts[3].to_string().contains("mon") {
        fader_number = 255;
        if event_parts[4].to_string().contains("cr_mute") {
            if event_parts[5].to_string().contains("MUTED") {
                state = true;
            }
        } else {
            return None;
        }
    } else if event_parts[3].to_string().contains("FaCH#") {
        if event_parts[4].to_string().contains("ON_State") {
            if event_parts[5].to_string().contains("ON") {
                state = true;
            }
        } else {
            return None;
        }
        let part_string = event_parts[3].to_string();
        let number_chars: Vec<char> = part_string
            .chars()
            .filter(|c: &char| c >= &'0' && c <= &'9')
            .collect();
        let mut number_string = String::new();
        for c in number_chars {
            number_string.push(c);
        }
        fader_number = match number_string.trim().parse() {
            Ok(num) => num,
            Err(_) => return None,
        };
    }
    Some((console_number, fader_number, state))
}
