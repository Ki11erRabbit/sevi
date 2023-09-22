
use core::fmt;

use bitflags::bitflags;


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
