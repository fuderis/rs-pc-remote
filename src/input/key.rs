use crate::prelude::*;
use enigo::Key as EnigoKey;

// The keyboard key codes
#[derive(Hash, Debug, Display, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Key {
    // Modifiers:
    Ctrl, Shift, Alt, Super,

    // Characters:
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Functions:
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Digitals:
    D0, D1, D2, D3, D4, D5, D6, D7, D8, D9,

    // NumPad:
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,

    // Symbols:
    Plus, Minus, Equal, Star, Slash, Backslash,
    
    // Special keys:
    Esc, Tab, CapsLock, NumLock, Space, Enter, Backspace, Delete,

    // Arrows:
    Left, Right, Up, Down,

    // Media:
    PlayPause, Prev, Next, Stop, VolumeUp, VolumeDown, Mute,
}

impl ::std::convert::Into<EnigoKey> for Key {
    fn into(self) -> EnigoKey {
        match self {
            // Modifiers:
            Self::Ctrl  => EnigoKey::Control,
            Self::Shift => EnigoKey::Shift,
            Self::Alt   => EnigoKey::Alt,
            Self::Super => EnigoKey::Meta,

            // Characters:
            Self::A => EnigoKey::A,
            Self::B => EnigoKey::B,
            Self::C => EnigoKey::C,
            Self::D => EnigoKey::D,
            Self::E => EnigoKey::E,
            Self::F => EnigoKey::F,
            Self::G => EnigoKey::G,
            Self::H => EnigoKey::H,
            Self::I => EnigoKey::I,
            Self::J => EnigoKey::J,
            Self::K => EnigoKey::K,
            Self::L => EnigoKey::L,
            Self::M => EnigoKey::M,
            Self::N => EnigoKey::N,
            Self::O => EnigoKey::O,
            Self::P => EnigoKey::P,
            Self::Q => EnigoKey::Q,
            Self::R => EnigoKey::R,
            Self::S => EnigoKey::S,
            Self::T => EnigoKey::T,
            Self::U => EnigoKey::U,
            Self::V => EnigoKey::V,
            Self::W => EnigoKey::W,
            Self::X => EnigoKey::X,
            Self::Y => EnigoKey::Y,
            Self::Z => EnigoKey::Z,

            // Functions:
            Self::F1 => EnigoKey::F1,
            Self::F2 => EnigoKey::F2,
            Self::F3 => EnigoKey::F3,
            Self::F4 => EnigoKey::F4,
            Self::F5 => EnigoKey::F5,
            Self::F6 => EnigoKey::F6,
            Self::F7 => EnigoKey::F7,
            Self::F8 => EnigoKey::F8,
            Self::F9 => EnigoKey::F9,
            Self::F10 => EnigoKey::F10,
            Self::F11 => EnigoKey::F11,
            Self::F12 => EnigoKey::F12,

            // Digitals:
            Self::D0 => EnigoKey::Num0,
            Self::D1 => EnigoKey::Num1,
            Self::D2 => EnigoKey::Num2,
            Self::D3 => EnigoKey::Num3,
            Self::D4 => EnigoKey::Num4,
            Self::D5 => EnigoKey::Num5,
            Self::D6 => EnigoKey::Num6,
            Self::D7 => EnigoKey::Num7,
            Self::D8 => EnigoKey::Num8,
            Self::D9 => EnigoKey::Num9,

            // NumPad:
            Self::N0 => EnigoKey::Numpad0,
            Self::N1 => EnigoKey::Numpad1,
            Self::N2 => EnigoKey::Numpad2,
            Self::N3 => EnigoKey::Numpad3,
            Self::N4 => EnigoKey::Numpad4,
            Self::N5 => EnigoKey::Numpad5,
            Self::N6 => EnigoKey::Numpad6,
            Self::N7 => EnigoKey::Numpad7,
            Self::N8 => EnigoKey::Numpad8,
            Self::N9 => EnigoKey::Numpad9,

            // Symbols:
            Self::Plus => EnigoKey::Unicode('+'),
            Self::Minus => EnigoKey::Unicode('-'),
            Self::Equal => EnigoKey::Unicode('='),
            Self::Star => EnigoKey::Unicode('*'),
            Self::Slash => EnigoKey::Unicode('/'),
            Self::Backslash => EnigoKey::Unicode('\\'),
            
            // Special keys:
            Self::Esc => EnigoKey::Escape,
            Self::Tab => EnigoKey::Tab,
            Self::CapsLock => EnigoKey::CapsLock,
            Self::NumLock => EnigoKey::Numlock,
            Self::Space => EnigoKey::Space,
            Self::Enter => EnigoKey::Return,
            Self::Backspace => EnigoKey::Backspace,
            Self::Delete => EnigoKey::Delete,

            // Arrows:
            Self::Left => EnigoKey::LeftArrow,
            Self::Right => EnigoKey::RightArrow,
            Self::Up => EnigoKey::UpArrow,
            Self::Down => EnigoKey::DownArrow,

            // Media:
            Self::PlayPause => EnigoKey::MediaPlayPause,
            Self::Prev => EnigoKey::MediaPrevTrack,
            Self::Next => EnigoKey::MediaNextTrack,
            Self::Stop => EnigoKey::MediaStop,
            Self::VolumeUp => EnigoKey::VolumeUp,
            Self::VolumeDown => EnigoKey::VolumeDown,
            Self::Mute => EnigoKey::VolumeMute,
        }
    }
}
