use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use crate::models::file::File;
use crate::models::settings::Settings;

//---------------------------------------|----------------------------------------
pub static TITLE_TEXT: &str = "\n                             SEVI - main help file\n\n";
pub static HELP_TEXT: [&str;105] = ["You can save this file by typing \":w<Enter>\".\n",
    "Move around:\n",
    "    You can use the arrow keys to move around.\n",
    "    You can also use the 'h', 'j', 'k', and 'l' keys to move left, down, up, and right respectively.\n",
    "\n",
    "Here are the commands that you can use in Sevi:\n",
    "    Close a pane:     Use \":q<Enter>\".\n",
    "    Close the editor: Use \":qa!<Enter>\" (careful as all changes will be lost!).\n",
    "    Save a file:      Use \":w<Enter>\".\n",
    "    Open a file:      Use \":e <filename><Enter>\" (this will also open previously opened files).\n",
    "    Open help file:   Use \":help<Enter>\" or (\":h<Enter>\").\n",
    "    Recover a file:   Use \":recover<Enter>\".\n\n",
    "The exclamation mark ('!') can be used with w, and q to force the action to happen.\n",
    "You can also chain certain commands together, even with exclamation marks.\n",
    "For example, \":wq!<Enter>\" will save and close the current file, even if saving failed.\n\n",
    "Movement Keybindings for Normal and Selection Mode:\n",
    "    \"h\" - move left\n",
    "    \"j\" - move down\n",
    "    \"k\" - move up\n",
    "    \"l\" - move right\n",
    "    \"w\" - move to start of the next word\n",
    "    \"W\" - move to start of the previous word\n",
    "    \"b\" - move to end of the previous word\n",
    "    \"B\" - move to end of the next word\n",
    "    \"0\" - move to the start of the line\n",
    "    \"$\" - move to the end of the line\n",
    "    \"gg\" - move to the start of the file\n",
    "    \"GG\" - move to the end of the file\n",
    "    \"G\" - move to a specific line (e.g. \"10G\" will move to line 10)\n",
    "    \"C-b\" - move up one page\n",
    "    \"C-f\" - move down one page\n",
    "    \"C-u\" - move up half a page\n",
    "    \"C-d\" - move down half a page\n",
    "    \"-\" - move up one line\n",
    "    \"+\" - move down one line\n\n",
    "Mode Change Keybindings:\n",
    "    Normal Mode:     \"Esc\" (Usually)\n",
    "    Insert Mode:     \"i\", \"I\", \"a\", \"A\", \"o\", \"O\"\n",
    "    Selection Mode:  \"v\", \"V\", \"C-v\"\n",
    "    Command Mode:    \":\"\n\n",
    "    Search Mode:     \"/\", \"?\"\n",
    "    Mirror Mode:     \"m\" from within Selection Mode and \"C-o\" from within Search Mode\n",
    "    Pair Mode:       \"C-p\" or \"a\" from within Selection Mode\"C-a\" from within Search Mode\n\n",
    "Copy Keybindings:\n",
    "    \"yy\" - copy the current character\n",
    "    \"Y\" - copy the current character\n",
    "    \"yl\" - copy the current line\n",
    "    \"yw\" - copy the current word\n",
    "    \"ynw\" - copy to the next word\n",
    "    \"ypw\" - copy to the previous word\n",
    "    \"y$\" - copy to the end of the line\n",
    "    \"ye\" - copy to the end of the line\n",
    "    \"ys\" - copy to the start of the line\n",
    "    \"y\" - copy selection\n",
    "    \"C-y\" - copy search\n\n",
    "Cut Keybindings:\n",
    "    \"xx\" - cut the current character\n",
    "    \"X\" - cut the current character\n",
    "    \"xl\" - cut the current line\n",
    "    \"xw\" - cut the current word\n",
    "    \"xnw\" - cut to the next word\n",
    "    \"xpw\" - cut to the previous word\n",
    "    \"x$\" - cut to the end of the line\n",
    "    \"xe\" - cut to the end of the line\n",
    "    \"xs\" - cut to the start of the line\n",
    "    \"x\" - cut selection\n",
    "    \"C-x\" - cut search\n\n",
    "Delete Keybindings:\n",
    "    \"dd\" - delete the current character\n",
    "    \"D\" - delete the current character\n",
    "    \"dl\" - delete the current line\n",
    "    \"dw\" - delete the current word\n",
    "    \"dnw\" - delete to the next word\n",
    "    \"dpw\" - delete to the previous word\n",
    "    \"d$\" - delete to the end of the line\n",
    "    \"de\" - delete to the end of the line\n",
    "    \"ds\" - delete to the start of the line\n",
    "    \"d\" - delete selection\n",
    "    \"C-d\" - delete search\n\n",
    "Paste Keybindings:\n",
    "    \"p\" - paste at the current cursor location or over selection\n",
    "    \"P\" - paste at the current cursor location\n",
    "    \"C-p\" - paste over the current search\n\n",
    "Search Keybindings:\n",
    "    \"/\" - starts a search forward\n",
    "    \"?\" - starts a search backward\n",
    "    \"C-n\" - move to the next search result\n",
    "    \"C-N\" - move to the previous search result\n",
    "    \"C-l\" - move to the next search result\n",
    "    \"C-h\" - move to the previous search result\n",
    "    \"C-j\" - move to the next search result\n",
    "    \"C-k\" - move to the previous search result\n\n",
    "Undo/Redo Keybindings:\n",
    "    \"u\" - undo\n",
    "    \"C-r\" - redo\n\n",
    "Description of Modes:\n",
    "    Normal Mode:     This is the default mode. You can move around and edit the file.\n",
    "    Insert Mode:     This mode allows you to insert text into the file.\n",
    "    Selection Mode:  This mode allows you to select text.\n",
    "    Command Mode:    This mode allows you to enter commands.\n",
    "    Search Mode:     This mode allows you to search for text.\n",
    "    Mirror Mode:     This mode allows you to insert text that is mirrored around a selection\n",
    "                     or a search.\n",
    "    Pair Mode:       This mode allows you to insert text that has a matching pair around a\n",
    "                     selection or a search.\n\n",



];




pub fn create_help_file(settings: Rc<RefCell<Settings>>) -> File {
    let total_text = String::from(TITLE_TEXT) + &HELP_TEXT.join("");

    let mut file = File::new(None, settings).unwrap();

    file.set_path(PathBuf::from("help.txt"));
    file.insert_after(0, total_text);

    file
}