#[derive(Debug, Clone, Copy)]
pub enum AnsiCode {
    Reset,
    Bold,
    FgColor(u8),
    ClearScreen,
    ClearLine,
    Unknown,
}

#[derive(Debug)]
pub enum AnsiState {
    Normal,
    Escape,
    Csi,
}

#[derive(Debug)]
pub struct AnsiParser {
    pub state: AnsiState,
    pub buffer: Vec<u8>,
}

impl AnsiParser {
    pub const fn new() -> Self {
        Self {
            state: AnsiState::Normal,
            buffer: Vec::new(),
        }
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<Vec<AnsiCode>> {
        match self.state {
            AnsiState::Normal => {
                if byte == 0x1B {
                    self.state = AnsiState::Escape;
                    self.buffer.clear();
                    self.buffer.push(byte);
                    None
                } else {
                    None
                }
            }
            AnsiState::Escape => {
                if byte == b'[' {
                    self.state = AnsiState::Csi;
                    self.buffer.push(byte);
                    None
                } else {
                    self.state = AnsiState::Normal;
                    None
                }
            }
            AnsiState::Csi => {
                self.buffer.push(byte);
                
                if (64..=126).contains(&byte) {
                    self.state = AnsiState::Normal;
                    let codes = parse_csi(&self.buffer);
                    return Some(codes);
                }
                None
            }
        }
    }
}

fn parse_csi(buffer: &[u8]) -> Vec<AnsiCode> {
    if buffer.len() < 3 {
        return vec![AnsiCode::Unknown];
    }

    let final_byte = buffer[buffer.len() - 1] as char;
    let inner = &buffer[2..buffer.len() - 1];
    let s = std::str::from_utf8(inner).unwrap_or("");

    let mut result = Vec::new();
    match final_byte {
        'm' => {
            if s.is_empty() {
                result.push(AnsiCode::Reset);
            } else {
                for part in s.split(';') {
                    if part.is_empty() {
                        continue;
                    }
                    if let Ok(n) = part.parse::<u8>() {
                        match n {
                            0 => result.push(AnsiCode::Reset),
                            1 => result.push(AnsiCode::Bold),
                            30..=37 => result.push(AnsiCode::FgColor(n - 30)),
                            _ => result.push(AnsiCode::Unknown),
                        }
                    } else {
                        result.push(AnsiCode::Unknown);
                    }
                }
            }
        }
        'J' => {
            if s == "2" {
                result.push(AnsiCode::ClearScreen);
            } else {
                result.push(AnsiCode::Unknown);
            }
        }
        'K' => {
            result.push(AnsiCode::ClearLine);
        }
        _ => result.push(AnsiCode::Unknown),
    }

    result
}
