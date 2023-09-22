
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

bitflags! {
    pub struct KeyModifiers: u8 {
        const NONE = 0;
        const SHIFT = 1;
        const CTRL = 2;
        const ALT = 4;
    }

}

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
