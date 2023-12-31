use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use crate::models::file::File;
use crate::models::settings::Settings;

//---------------------------------------|----------------------------------------
pub static TITLE_TEXT: &str = "\n                             SEVI - main help file\n\n";
pub static HELP_TEXT: [&str;199] = ["You can save this file by typing \":w<Enter>\".\n",
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
    "A note on Cut/Copy/Paste:\n",
    "    These interact with the system clipboard if no error occured.\n",
    "    This is rather limiting, so to solve this, you can enter a number before the keypress.\n",
    "    The number specifies a register to store or pull the text from. There is no limit to the\n",
    "    number of registers.\n\n",
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
    "Configuring Sevi:\n",
    "    You can configure Sevi by editing the config files.\n",
    "    The config files are located in the following locations:\n",
    "        Linux:   $HOME/.config/sevi/\n",
    //"        Windows: %APPDATA%\\sevi\\config\\\n",
    //"        Mac:     $HOME/Library/Application Support/sevi/config/\n",
    "    The config files are in the TOML format.\n",
    "    The config files are as follows:\n",
    "        config.toml: This file contains the settings for the editor.\n",
    "        keybindings.toml: This file contains the keybindings for the editor.\n",
    "        colors.toml: This file contains coloring of various parts of the editor.\n\n",
    "    config.toml:\n",
    "        number_line - This setting controls how the number bar is displayed.\n",
    "            Possible values are:\n",
    "                \"None\" - No number bar is displayed.\n",
    "                \"Absolute\" - The number bar displays the absolute line number.\n",
    "                \"Relative\" - The number bar displays the relative line number.\n",
    "        tab_size - This setting controls the size of a tab. This is an integer.\n",
    "        use_spaces - This setting controls whether spaces are used instead of tabs.\n",
    "                It is a boolean value.\n",
    "        rainbow_delimiters - This setting controls whether rainbow delimiters are used.\n",
    "                It is a boolean value. Rainbow delimiters, or Rainbow Parenthesis is a plugin\n",
    "                for Emacs that highlights matching parenthesis and other symbol pairs with \n",
    "                different colors.\n",
    "        default_mode - This setting controls the default mode that the editor starts in.\n",
    "            Possible values are:\n",
    "                \"Normal\" - The editor starts in Normal Mode.\n",
    "                \"Insert\" - The editor starts in Insert Mode.\n",
    "        pairs - This setting holds the data for the pairs that are used in Pair Mode.\n",
    "            The format is an array of arrays of strings. Each array of strings is a pair.\n",
    "            For example, one of the pairs is:\n",
    "                [\"(\", \")\"]\n\n",
    "    colors.toml:\n",
    "        The colors.toml uses toml tables extensively to organize the colors.\n",
    "        rainbow_delimiters - This is the only array but it holds the colors for the rainbow\n",
    "               delimiters.\n",
    "        selected - The color scheme for the selected text.\n",
    "        buffer_color - The color scheme for displayed text.\n",
    "        number_bar - The color scheme for the number bar.\n",
    "            This one has two settings:\n",
    "                current_line - The color scheme for the line the cursor is on.\n",
    "                other_lines - The color scheme for the other lines.\n",
    "        status_bar - The color scheme for the status bar.\n",
    "            this has 5 fields:\n",
    "                message - The color scheme status messages.\n",
    "                first - The color scheme for first section of the status bar.\n",
    "                second - The color scheme for the second section of the status bar.\n",
    "                rest - The color scheme for the rest of the status bar.\n",
    "                mode - the color schemes for the indicator of the current mode.\n",
    "                    You can style these individually by using the following fields:\n",
    "                        Normal, Insert, Selection, Command, Search, Mirror, Pair\n",
    "        The table values for a color scheme are:\n",
    "            fg - This is the background color.\n",
    "                Possible values are:\n",
    "                    \"reset\" - for no setting\n",
    "                    ANSI color sequences (e.g. \"black\" or \"light-cyan\")\"\n",
    "                    RGB values (e.g. [255, 255, 255])\n",
    "                    8-bit 256 color values (e.g. 255)\n",
    "            bg - This is the foreground color.\n",
    "                Possible values are the same as fg\n",
    "            underline_color - This is the color of the underline if there is one.\n",
    "            modifiers - This is an array of strings that are the modifiers for the color.\n",
    "                Possible values are:\n",
    "                    \"bold\"\n",
    "                    \"dim\"\n",
    "                    \"italic\"\n",
    "                    \"underline\"\n",
    "                    \"slow_blink\"\n",
    "                    \"rapid_blink\"\n",
    "                    \"reversed\"\n",
    "                    \"hidden\"\n",
    "                    \"crossed_out\"\n\n",
    "    keybindings.toml:\n",
    "        The keybindings.toml uses toml tables extensively to organize the keybindings.\n",
    "        The keybindings are quite extensive and will not be listed here.\n",
    "        You can however generate a configuration file with the default settings by running\n",
    "        Sevi with the \"--generate-default-settings\" (\"-g\") flag.\n",
    "        The format is as follows:\n",
    "            \"<action> = <keybinding>\"\n",
    "             Where a keybinding can be one of the following:\n",
    "                 - A string for a single key with uppercase specifying the use of the \"Shift\"\n",
    "                   key. You can also use key names but they must start with a capital.\n",
    "                 - A table with the following fields:\n",
    "                     - key - The key to use. This can be a string or an array of strings.\n",
    "                     - mod - This is an array of strings that are the modifiers for the\n",
    "                       keybinding. Values are \"Ctrl\" or \"Alt\"\n",
    "                 - A table with the following fields:\n",
    "                     - keys - This is an array of the above table. This is to allow for\n",
    "                       a chain of keys to be used for a binding.\n",
    "                 - An array of the above table. This is to allow for multiple keybindings\n",
    "                   for a single action.\n",





];




pub fn create_help_file(settings: Rc<RefCell<Settings>>) -> File {
    let total_text = String::from(TITLE_TEXT) + &HELP_TEXT.join("");

    let mut file = File::new(None, settings).unwrap();

    file.set_path(PathBuf::from("help.txt"));
    file.insert_after(0, total_text);

    file
}