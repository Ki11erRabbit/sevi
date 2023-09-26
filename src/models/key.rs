
use core::fmt;

use bitflags::bitflags;
//use iced::keyboard::Event;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// The backspace key.
    Backspace,
    /// The enter key.
    Enter,
    /// The left arrow key.
    Left,
    /// The right arrow key.
    Right,
    /// The up arrow key.
    Up,
    /// The down arrow key.
    Down,
    /// The home key.
    Home,
    /// The end key.
    End,
    /// The page up key.
    PageUp,
    /// The page down key.
    PageDown,
    /// The tab key.
    Tab,
    /// The back tab key.
    BackTab,
    /// The delete key.
    Delete,
    /// The insert key.
    Insert,
    /// The function keys.
    F(u8),
    /// A character.
    Char(char),
    /// Null
    Null,
    /// The caps lock pressed key.
    CapsLock,
    /// The scroll lock pressed key.
    ScrollLock,
    /// The num lock pressed key.
    NumLock,
    /// The Print Screen key.
    PrintScreen,
    /// The Pause key.
    Pause,
    /// The Menu key.
    Menu,
    /// keypad begin
    KeypadBegin,
    /// A media key
    Media(MediaKey),
    /// The escape key.
    Esc,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Backspace => write!(f, "Backspace"),
            Key::Enter => write!(f, "Enter"),
            Key::Left => write!(f, "Left"),
            Key::Right => write!(f, "Right"),
            Key::Up => write!(f, "Up"),
            Key::Down => write!(f, "Down"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            Key::Tab => write!(f, "Tab"),
            Key::BackTab => write!(f, "BackTab"),
            Key::Delete => write!(f, "Delete"),
            Key::Insert => write!(f, "Insert"),
            Key::F(n) => write!(f, "F{}", n),
            Key::Char(c) => write!(f, "{}", c),
            Key::Null => write!(f, "Null"),
            Key::CapsLock => write!(f, "CapsLock"),
            Key::ScrollLock => write!(f, "ScrollLock"),
            Key::NumLock => write!(f, "NumLock"),
            Key::PrintScreen => write!(f, "PrintScreen"),
            Key::Pause => write!(f, "Pause"),
            Key::Menu => write!(f, "Menu"),
            Key::KeypadBegin => write!(f, "KeypadBegin"),
            Key::Media(media_key) => write!(f, "Media({})", media_key),
            Key::Esc => write!(f, "Esc"),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaKey {
    /// The play key.
    Play,
    /// The pause key.
    Pause,
    /// The Play/Pause key.
    PlayPause,
    /// The reverse key.
    Reverse,
    /// The stop key.
    Stop,
    /// The fast forward key.
    FastForward,
    /// The rewind key.
    Rewind,
    /// The track next key.
    TrackNext,
    /// The track previous key.
    TrackPrevious,
    /// The record key.
    Record,
    /// The volume down key.
    LowerVolume,
    /// The volume up key.
    RaiseVolume,
    /// The mute key.
    MuteVolume,
}

impl fmt::Display for MediaKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaKey::Play => write!(f, "Play"),
            MediaKey::Pause => write!(f, "Pause"),
            MediaKey::PlayPause => write!(f, "Play/Pause"),
            MediaKey::Reverse => write!(f, "Reverse"),
            MediaKey::Stop => write!(f, "Stop"),
            MediaKey::FastForward => write!(f, "FastForward"),
            MediaKey::Rewind => write!(f, "Rewind"),
            MediaKey::TrackNext => write!(f, "TrackNext"),
            MediaKey::TrackPrevious => write!(f, "TrackPrevious"),
            MediaKey::Record => write!(f, "Record"),
            MediaKey::LowerVolume => write!(f, "LowerVolume"),
            MediaKey::RaiseVolume => write!(f, "RaiseVolume"),
            MediaKey::MuteVolume => write!(f, "MuteVolume"),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyModifiers: u8 {
        const NONE = 0;
        const SHIFT = 1;
        const CTRL = 2;
        const ALT = 4;
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    pub fn new(key: Key, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            key,
            modifiers,
        }
    }
}

impl From<Key> for KeyEvent {
    fn from(key: Key) -> KeyEvent {
        Self::new(key, KeyModifiers::empty())
    }
}


impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::CTRL) {
            write!(f, "C-")?;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            write!(f, "M-")?;
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            write!(f, "S-")?;
        }
        write!(f, "{}", self.key)
    }
}





//TODO: add conditional compilation for using TUI
impl From<tuirealm::event::KeyEvent> for KeyEvent {
    fn from(key_event: tuirealm::event::KeyEvent) -> KeyEvent {
        let code = match key_event.code {
            tuirealm::event::Key::Backspace => Key::Backspace,
            tuirealm::event::Key::Enter => Key::Enter,
            tuirealm::event::Key::Left => Key::Left,
            tuirealm::event::Key::Right => Key::Right,
            tuirealm::event::Key::Up => Key::Up,
            tuirealm::event::Key::Down => Key::Down,
            tuirealm::event::Key::Home => Key::Home,
            tuirealm::event::Key::End => Key::End,
            tuirealm::event::Key::PageUp => Key::PageUp,
            tuirealm::event::Key::PageDown => Key::PageDown,
            tuirealm::event::Key::Tab => Key::Tab,
            tuirealm::event::Key::BackTab => Key::BackTab,
            tuirealm::event::Key::Delete => Key::Delete,
            tuirealm::event::Key::Insert => Key::Insert,
            tuirealm::event::Key::Function(n) => Key::F(n),
            tuirealm::event::Key::Char(c) => Key::Char(c),
            tuirealm::event::Key::Null => Key::Null,
            tuirealm::event::Key::CapsLock => Key::CapsLock,
            tuirealm::event::Key::ScrollLock => Key::ScrollLock,
            tuirealm::event::Key::NumLock => Key::NumLock,
            tuirealm::event::Key::PrintScreen => Key::PrintScreen,
            tuirealm::event::Key::Pause => Key::Pause,
            tuirealm::event::Key::Menu => Key::Menu,
            tuirealm::event::Key::KeypadBegin => Key::KeypadBegin,
            tuirealm::event::Key::Media(media_key) => Key::Media(match media_key {
                tuirealm::event::MediaKeyCode::Play => MediaKey::Play,
                tuirealm::event::MediaKeyCode::Pause => MediaKey::Pause,
                tuirealm::event::MediaKeyCode::PlayPause => MediaKey::PlayPause,
                tuirealm::event::MediaKeyCode::Reverse => MediaKey::Reverse,
                tuirealm::event::MediaKeyCode::Stop => MediaKey::Stop,
                tuirealm::event::MediaKeyCode::FastForward => MediaKey::FastForward,
                tuirealm::event::MediaKeyCode::Rewind => MediaKey::Rewind,
                tuirealm::event::MediaKeyCode::TrackNext => MediaKey::TrackNext,
                tuirealm::event::MediaKeyCode::TrackPrevious => MediaKey::TrackPrevious,
                tuirealm::event::MediaKeyCode::Record => MediaKey::Record,
                tuirealm::event::MediaKeyCode::LowerVolume => MediaKey::LowerVolume,
                tuirealm::event::MediaKeyCode::RaiseVolume => MediaKey::RaiseVolume,
                tuirealm::event::MediaKeyCode::MuteVolume => MediaKey::MuteVolume,
            }),
            tuirealm::event::Key::Esc => Key::Esc,
        };


        let mut modifiers = if key_event.modifiers.contains(tuirealm::event::KeyModifiers::SHIFT) {
            if let Key::Char(x) = code {
                if x.is_uppercase() {
                    KeyModifiers::NONE
                } else {
                    KeyModifiers::SHIFT
                }
            } else {
                KeyModifiers::SHIFT
            }
        } else {
            KeyModifiers::NONE
        };
        if key_event.modifiers.contains(tuirealm::event::KeyModifiers::CONTROL) {
            modifiers |= KeyModifiers::CTRL;
        }
        if key_event.modifiers.contains(tuirealm::event::KeyModifiers::ALT) {
            modifiers |= KeyModifiers::ALT;
        }

        KeyEvent::new(code, modifiers)
    }
}

/*impl From<Event> for KeyEvent {
    fn from(value: Event) -> Self {
        match value {
            Event::KeyPressed {
                key_code: code,
                modifiers,
            } => {
                let mut key = match code {
                    iced::keyboard::KeyCode::Key1 => Key::Char('1'),
                    iced::keyboard::KeyCode::Key2 => Key::Char('2'),
                    iced::keyboard::KeyCode::Key3 => Key::Char('3'),
                    iced::keyboard::KeyCode::Key4 => Key::Char('4'),
                    iced::keyboard::KeyCode::Key5 => Key::Char('5'),
                    iced::keyboard::KeyCode::Key6 => Key::Char('6'),
                    iced::keyboard::KeyCode::Key7 => Key::Char('7'),
                    iced::keyboard::KeyCode::Key8 => Key::Char('8'),
                    iced::keyboard::KeyCode::Key9 => Key::Char('9'),
                    iced::keyboard::KeyCode::Key0 => Key::Char('0'),
                    iced::keyboard::KeyCode::A => Key::Char('a'),
                    iced::keyboard::KeyCode::B => Key::Char('b'),
                    iced::keyboard::KeyCode::C => Key::Char('c'),
                    iced::keyboard::KeyCode::D => Key::Char('d'),
                    iced::keyboard::KeyCode::E => Key::Char('e'),
                    iced::keyboard::KeyCode::F => Key::Char('f'),
                    iced::keyboard::KeyCode::G => Key::Char('g'),
                    iced::keyboard::KeyCode::H => Key::Char('h'),
                    iced::keyboard::KeyCode::I => Key::Char('i'),
                    iced::keyboard::KeyCode::J => Key::Char('j'),
                    iced::keyboard::KeyCode::K => Key::Char('k'),
                    iced::keyboard::KeyCode::L => Key::Char('l'),
                    iced::keyboard::KeyCode::M => Key::Char('m'),
                    iced::keyboard::KeyCode::N => Key::Char('n'),
                    iced::keyboard::KeyCode::O => Key::Char('o'),
                    iced::keyboard::KeyCode::P => Key::Char('p'),
                    iced::keyboard::KeyCode::Q => Key::Char('q'),
                    iced::keyboard::KeyCode::R => Key::Char('r'),
                    iced::keyboard::KeyCode::S => Key::Char('s'),
                    iced::keyboard::KeyCode::T => Key::Char('t'),
                    iced::keyboard::KeyCode::U => Key::Char('u'),
                    iced::keyboard::KeyCode::V => Key::Char('v'),
                    iced::keyboard::KeyCode::W => Key::Char('w'),
                    iced::keyboard::KeyCode::X => Key::Char('x'),
                    iced::keyboard::KeyCode::Y => Key::Char('y'),
                    iced::keyboard::KeyCode::Z => Key::Char('z'),
                    iced::keyboard::KeyCode::Escape => Key::Esc,
                    iced::keyboard::KeyCode::F1 => Key::F(1),
                    iced::keyboard::KeyCode::F2 => Key::F(2),
                    iced::keyboard::KeyCode::F3 => Key::F(3),
                    iced::keyboard::KeyCode::F4 => Key::F(4),
                    iced::keyboard::KeyCode::F5 => Key::F(5),
                    iced::keyboard::KeyCode::F6 => Key::F(6),
                    iced::keyboard::KeyCode::F7 => Key::F(7),
                    iced::keyboard::KeyCode::F8 => Key::F(8),
                    iced::keyboard::KeyCode::F9 => Key::F(9),
                    iced::keyboard::KeyCode::F10 => Key::F(10),
                    iced::keyboard::KeyCode::F11 => Key::F(11),
                    iced::keyboard::KeyCode::F12 => Key::F(12),
                    iced::keyboard::KeyCode::F13 => Key::F(13),
                    iced::keyboard::KeyCode::F14 => Key::F(14),
                    iced::keyboard::KeyCode::F15 => Key::F(15),
                    iced::keyboard::KeyCode::F16 => Key::F(16),
                    iced::keyboard::KeyCode::F17 => Key::F(17),
                    iced::keyboard::KeyCode::F18 => Key::F(18),
                    iced::keyboard::KeyCode::F19 => Key::F(19),
                    iced::keyboard::KeyCode::F20 => Key::F(20),
                    iced::keyboard::KeyCode::F21 => Key::F(21),
                    iced::keyboard::KeyCode::F22 => Key::F(22),
                    iced::keyboard::KeyCode::F23 => Key::F(23),
                    iced::keyboard::KeyCode::F24 => Key::F(24),
                    iced::keyboard::KeyCode::Snapshot => Key::PrintScreen,
                    iced::keyboard::KeyCode::Scroll => Key::ScrollLock,
                    iced::keyboard::KeyCode::Pause => Key::Pause,
                    iced::keyboard::KeyCode::Insert => Key::Insert,
                    iced::keyboard::KeyCode::Home => Key::Home,
                    iced::keyboard::KeyCode::Delete => Key::Delete,
                    iced::keyboard::KeyCode::End => Key::End,
                    iced::keyboard::KeyCode::PageDown => Key::PageDown,
                    iced::keyboard::KeyCode::PageUp => Key::PageUp,
                    iced::keyboard::KeyCode::Left => Key::Left,
                    iced::keyboard::KeyCode::Up => Key::Up,
                    iced::keyboard::KeyCode::Right => Key::Right,
                    iced::keyboard::KeyCode::Down => Key::Down,
                    iced::keyboard::KeyCode::Backspace => Key::Backspace,
                    iced::keyboard::KeyCode::Enter => Key::Enter,
                    iced::keyboard::KeyCode::Space => Key::Char(' '),
                    iced::keyboard::KeyCode::Compose => Key::Null,
                    iced::keyboard::KeyCode::Caret => Key::Char('^'),
                    iced::keyboard::KeyCode::Numlock => Key::NumLock,
                    iced::keyboard::KeyCode::Numpad0 => Key::Null,
                    iced::keyboard::KeyCode::Numpad1 => Key::Null,
                    iced::keyboard::KeyCode::Numpad2 => Key::Null,
                    iced::keyboard::KeyCode::Numpad3 => Key::Null,
                    iced::keyboard::KeyCode::Numpad4 => Key::Null,
                    iced::keyboard::KeyCode::Numpad5 => Key::Null,
                    iced::keyboard::KeyCode::Numpad6 => Key::Null,
                    iced::keyboard::KeyCode::Numpad7 => Key::Null,
                    iced::keyboard::KeyCode::Numpad8 => Key::Null,
                    iced::keyboard::KeyCode::Numpad9 => Key::Null,
                    iced::keyboard::KeyCode::NumpadAdd => Key::Null,
                    iced::keyboard::KeyCode::NumpadDivide => Key::Null,
                    iced::keyboard::KeyCode::NumpadDecimal => Key::Null,
                    iced::keyboard::KeyCode::NumpadComma => Key::Null,
                    iced::keyboard::KeyCode::NumpadEnter => Key::Null,
                    iced::keyboard::KeyCode::NumpadEquals => Key::Null,
                    iced::keyboard::KeyCode::NumpadMultiply => Key::Null,
                    iced::keyboard::KeyCode::NumpadSubtract => Key::Null,
                    iced::keyboard::KeyCode::AbntC1 => Key::Null,
                    iced::keyboard::KeyCode::AbntC2 => Key::Null,
                    iced::keyboard::KeyCode::Apostrophe => Key::Char('\''),
                    iced::keyboard::KeyCode::Apps => Key::Null,
                    iced::keyboard::KeyCode::Asterisk => Key::Char('*'),
                    iced::keyboard::KeyCode::At => Key::Char('@'),
                    iced::keyboard::KeyCode::Ax => Key::Null,
                    iced::keyboard::KeyCode::Backslash => Key::Char('\\'),
                    iced::keyboard::KeyCode::Calculator => Key::Null,
                    iced::keyboard::KeyCode::Capital => Key::Null,
                    iced::keyboard::KeyCode::Colon => Key::Char(':'),
                    iced::keyboard::KeyCode::Comma => Key::Char(','),
                    iced::keyboard::KeyCode::Convert => Key::Null,
                    iced::keyboard::KeyCode::Equals => Key::Char('='),
                    iced::keyboard::KeyCode::Grave => Key::Char('`'),
                    iced::keyboard::KeyCode::Kana => Key::Null,
                    iced::keyboard::KeyCode::Kanji => Key::Null,
                    iced::keyboard::KeyCode::LAlt => Key::Null,
                    iced::keyboard::KeyCode::LBracket => Key::Char('['),
                    iced::keyboard::KeyCode::LControl => Key::Null,
                    iced::keyboard::KeyCode::LShift => Key::Null,
                    iced::keyboard::KeyCode::LWin => Key::Null,
                    iced::keyboard::KeyCode::Mail => Key::Null,
                    iced::keyboard::KeyCode::MediaSelect => Key::Null,
                    iced::keyboard::KeyCode::MediaStop => Key::Media(MediaKey::Stop),
                    iced::keyboard::KeyCode::Minus => Key::Char('-'),
                    iced::keyboard::KeyCode::Mute => Key::Media(MediaKey::MuteVolume),
                    iced::keyboard::KeyCode::MyComputer => Key::Null,
                    iced::keyboard::KeyCode::NavigateForward => Key::Null,
                    iced::keyboard::KeyCode::NavigateBackward => Key::Null,
                    iced::keyboard::KeyCode::NextTrack => Key::Media(MediaKey::TrackNext),
                    iced::keyboard::KeyCode::NoConvert => Key::Null,
                    iced::keyboard::KeyCode::OEM102 => Key::Null,
                    iced::keyboard::KeyCode::Period => Key::Char('.'),
                    iced::keyboard::KeyCode::PlayPause => Key::Media(MediaKey::PlayPause),
                    iced::keyboard::KeyCode::Plus => Key::Char('+'),
                    iced::keyboard::KeyCode::Power => Key::Null,
                    iced::keyboard::KeyCode::PrevTrack => Key::Media(MediaKey::TrackPrevious),
                    iced::keyboard::KeyCode::RAlt => Key::Null,
                    iced::keyboard::KeyCode::RBracket => Key::Char(']'),
                    iced::keyboard::KeyCode::RControl => Key::Null,
                    iced::keyboard::KeyCode::RShift => Key::Null,
                    iced::keyboard::KeyCode::RWin => Key::Null,
                    iced::keyboard::KeyCode::Semicolon => Key::Char(';'),
                    iced::keyboard::KeyCode::Slash => Key::Char('/'),
                    iced::keyboard::KeyCode::Sleep => Key::Null,
                    iced::keyboard::KeyCode::Stop => Key::Media(MediaKey::Stop),
                    iced::keyboard::KeyCode::Sysrq => Key::Null,
                    iced::keyboard::KeyCode::Tab => Key::Tab,
                    iced::keyboard::KeyCode::Underline => Key::Char('_'),
                    iced::keyboard::KeyCode::Unlabeled => Key::Null,
                    iced::keyboard::KeyCode::VolumeDown => Key::Media(MediaKey::LowerVolume),
                    iced::keyboard::KeyCode::VolumeUp => Key::Media(MediaKey::RaiseVolume),
                    iced::keyboard::KeyCode::Wake => Key::Null,
                    iced::keyboard::KeyCode::WebBack => Key::Null,
                    iced::keyboard::KeyCode::WebFavorites => Key::Null,
                    iced::keyboard::KeyCode::WebForward => Key::Null,
                    iced::keyboard::KeyCode::WebHome => Key::Null,
                    iced::keyboard::KeyCode::WebRefresh => Key::Null,
                    iced::keyboard::KeyCode::WebSearch => Key::Null,
                    iced::keyboard::KeyCode::WebStop => Key::Null,
                    iced::keyboard::KeyCode::Yen => Key::Null,
                    iced::keyboard::KeyCode::Copy => Key::Null,
                    iced::keyboard::KeyCode::Paste => Key::Null,
                    iced::keyboard::KeyCode::Cut => Key::Null,
                };

                let mut modifier = if modifiers.contains(iced::keyboard::Modifiers::SHIFT) {
                    if let Key::Char(mut x) = &mut key {
                        if x.is_lowercase() {
                            x = x.to_uppercase().into();
                            KeyModifiers::NONE
                        } else {
                            KeyModifiers::SHIFT
                        }
                    } else {
                        KeyModifiers::SHIFT
                    }
                } else {
                    KeyModifiers::NONE
                };

                if modifiers.contains(iced::keyboard::Modifiers::CONTROL) {
                    modifier |= KeyModifiers::CTRL;
                }
                if modifiers.contains(iced::keyboard::Modifiers::ALT) {
                    modifier |= KeyModifiers::ALT;
                }

                KeyEvent::new(key, modifier)

            }
            Event::KeyReleased {
                ..
            } => {
                KeyEvent::new(Key::Null, KeyModifiers::NONE)
            }
            Event::CharacterReceived(c) => {
                KeyEvent::new(Key::Char(c), KeyModifiers::NONE)
            }
            Event::ModifiersChanged(modifiers) => {
                KeyEvent::new(Key::Null, KeyModifiers::NONE)
            }
        }
    }
}*/







