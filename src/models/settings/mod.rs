
use self::editor_settings::EditorSettings;
use self::mode_keybindings::ModeKeybindings;

pub mod editor_settings;
pub mod mode_keybindings;
//pub mod language_formats;
//pub mod colors;



pub struct Settings {
    /// The general settings for the editor
    /// (e.g. font size, tab size, etc.)
    pub editor_settings: EditorSettings,
    /// The keybindings for the editor
    /// The keybindings are separated by the mode
    pub mode_keybindings: ModeKeybindings,
    /*/// The different formats for different programming languages
    /// (e.g. Kernel, Google, Microsoft, etc.)
    pub language_formats: LanguageFormats,
    /// The colors for the editor
    pub colors: Colors,*/
}




impl Default for Settings {
    fn default() -> Self {
        Settings {
            editor_settings: EditorSettings::default(),
            mode_keybindings: ModeKeybindings::new(),
            /*language_formats: LanguageFormats::default(),
            colors: Colors::default(),*/
        }
    }
}




