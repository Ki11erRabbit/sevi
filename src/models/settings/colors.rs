use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::models::style::color::Color;
use crate::models::style::Style;


impl Default for EditorColors {
    fn default() -> Self {
        EditorColors {
            buffer_color: Style::new(),
            selected: Style::new().fg(Color::Black).bg(Color::LightBlue),
            number_bar: NumberBarColor::default(),
            status_bar: StatusBarColor::default(),
            rainbow_delimiters: vec![
                Style::new().fg(Color::LightMagenta),
                Style::new().fg(Color::LightBlue),
                Style::new().fg(Color::LightCyan),
                Style::new().fg(Color::LightGreen),
                Style::new().fg(Color::LightYellow),
                Style::new().fg(Color::LightRed),
            ],
            syntax_highlighting: Self::generate_default_syntax_highlighting(),
        }
    }
}
pub struct EditorColors {
    pub buffer_color: Style,
    pub selected: Style,
    pub number_bar: NumberBarColor,
    pub status_bar: StatusBarColor,
    pub rainbow_delimiters: Vec<Style>,
    pub syntax_highlighting: HashMap<String, Vec<SyntaxHighlightRule>>,
}

impl EditorColors {
    pub fn config_file(&self) -> String {
        let mut output = String::from("[EditorColors]\n");

        output.push_str(&format!("rainbow_delimiters = [{}]\n",
                                 self.rainbow_delimiters.iter()
                                     .map(|x| x
                                         .config_file().replace("\n", ", ").split(", ").filter(|x| !x.is_empty()).collect::<Vec<&str>>().join(", ")
                                     ).map(|x| format!("{{ {} }}", x))
                                     .collect::<Vec<String>>()
                                     .join(", ")));

        output.push_str(&format!("[EditorColors.buffer_color]\n{}\n", self.buffer_color.config_file()));

        output.push_str(&format!("[EditorColors.selected]\n{}\n", self.selected.config_file()));


        output.push_str(&format!("{}\n", self.number_bar.config_file()));

        output.push_str(&format!("{}\n", self.status_bar.config_file()));




        output
    }

    pub fn create_default_config_file() -> io::Result<()> {
        let settings = EditorColors::default();

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let user_config_path = xdg_dirs.place_config_file("colors.toml").expect("Could not create config file");

        let mut file = File::create(user_config_path)?;
        file.write_all(settings.config_file().as_bytes())
    }

    fn load_user_settings(config: &str) -> Result<EditorColors, String> {
        let table = config.parse::<toml::Value>().expect("Could not parse config file");

        match table.get("EditorColors") {
            None => Ok(EditorColors::default()),
            Some(editor_colors) => parse_editor_colors(editor_colors),
        }
    }

    pub fn new() -> EditorColors {
        let mut settings = EditorColors::default();

        if crate::arg_parser::IGNORE_USER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed) {
            return settings;
        }

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let config_path = xdg_dirs.place_config_file("colors.toml").expect("Could not create config file");

        match File::open(config_path) {
            Err(_) => {},
            Ok(mut user_config) => {
                let mut string = String::new();
                user_config.read_to_string(&mut string).expect("Could not read config file");

                let user_settings = EditorColors::load_user_settings(&string).expect("Could not parse config file");

                settings.merge(user_settings);
            }
        }


        settings
    }

    fn merge(&mut self, other: Self) {
        self.buffer_color.patch(other.buffer_color);
        self.number_bar.merge(other.number_bar);
        self.status_bar.merge(other.status_bar);
        self.rainbow_delimiters = other.rainbow_delimiters;
    }

    fn generate_default_syntax_highlighting() -> HashMap<String, Vec<SyntaxHighlightRule>> {
        use SyntaxHighlightRule as SHR;
        let mut syntax_highlighting = HashMap::new();
        // built in
        syntax_highlighting.insert("namespace".to_string(), vec![
            SHR::new(vec![],Style::new().fg(Color::LightMagenta)),
            SHR::new(vec!["library".to_string(),"defaultLibrary".to_string(),"declaration".to_string(),"crateRoot".to_string()],Style::new().fg(Color::LightMagenta)),
        ]);
        syntax_highlighting.insert("type".to_string(), vec![SHR::new(Vec::new(),Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("class".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("enum".to_string(), vec![SHR::new(vec!["declaration".to_string(),"public".to_string(), "library".to_string()],Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("interface".to_string(), vec![SHR::new(vec!["declaration".to_string(),"library".to_string()],Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("struct".to_string(), vec![SHR::new(vec!["declaration".to_string(),"public".to_string(), "library".to_string()],Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("typeParameter".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        syntax_highlighting.insert("parameter".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("variable".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("property".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("enumMember".to_string(), vec![
            SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta)),
            SHR::new(vec!["defaultLibrary".to_string(), "library".to_string()],Style::new().fg(Color::Yellow))
        ]);
        syntax_highlighting.insert("event".to_string(), vec![SHR::new(vec![],Style::new())]);
        syntax_highlighting.insert("function".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("method".to_string(), vec![SHR::new(vec!["declaration".to_string()],Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("macro".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        syntax_highlighting.insert("keyword".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightBlue))]);
        syntax_highlighting.insert("modifier".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Magenta))]);
        syntax_highlighting.insert("comment".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::DarkGray))]);
        syntax_highlighting.insert("string".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightGreen))]);
        syntax_highlighting.insert("number".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("regexp".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("operator".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("decorator".to_string(), vec![SHR::new(Vec::new(), Style::new())]);

        // Rust Specific
        syntax_highlighting.insert("attribute".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightBlue).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        //enum
        //function
        syntax_highlighting.insert("derive".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightBlue).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        //macro
        //method
        //namespace
        //struct
        syntax_highlighting.insert("trait".to_string(), vec![SHR::new(vec!["declaration".to_string()], Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("typeAlias".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("union".to_string(), vec![SHR::new(vec!["declaration".to_string()], Style::new().fg(Color::Yellow))]);

        syntax_highlighting.insert("boolean".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightBlue))]);
        syntax_highlighting.insert("character".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightGreen))]);
        //string
        syntax_highlighting.insert("escapeSequence".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightYellow))]);
        syntax_highlighting.insert("formatSpecifier".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightYellow))]);

        //operator
        syntax_highlighting.insert("arithmetic".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("bitwise".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("comparison".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("logical".to_string(), vec![SHR::new(Vec::new(), Style::new())]);

        syntax_highlighting.insert("punctuation".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("attributeBracket".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        syntax_highlighting.insert("angle".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan))]);
        syntax_highlighting.insert("brace".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("bracket".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("parenthesis".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("colon".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("comma".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("dot".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("semi".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("macroBang".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);

        syntax_highlighting.insert("builtinAttribute".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        syntax_highlighting.insert("builtinType".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow))]);
        //comment
        syntax_highlighting.insert("constParameter".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        syntax_highlighting.insert("deriveHelper".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightCyan).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        //enumMember
        syntax_highlighting.insert("generic".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow).add_modifier(crate::models::style::text_modifier::Modifier::BOLD))]);
        //keyword
        syntax_highlighting.insert("label".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightMagenta))]);
        syntax_highlighting.insert("lifetime".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightMagenta))]);
        //parameter
        //property
        syntax_highlighting.insert("selfKeyword".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightBlue))]);
        syntax_highlighting.insert("selfTypeKeyword".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::Yellow))]);
        syntax_highlighting.insert("toolModule".to_string(), vec![SHR::new(Vec::new(), Style::new())]);
        //typeParameter
        syntax_highlighting.insert("unresolvedReference".to_string(), vec![SHR::new(Vec::new(), Style::new().fg(Color::LightRed))]);
        //variable





        syntax_highlighting
    }
}


pub struct SyntaxHighlightRule {
    pub modifiers: Vec<String>,
    pub style: Style,
}

impl SyntaxHighlightRule {
    pub fn new(modifiers: Vec<String>, style: Style) -> Self {
        SyntaxHighlightRule {
            modifiers,
            style,
        }
    }

    pub fn can_apply(&self, modifiers: &Vec<String>) -> bool {
        if self.modifiers.is_empty() {
            return true;
        } else {
            let mut acc = false;
            for modifier in self.modifiers.iter() {
                if modifiers.contains(modifier) {
                    acc = true;
                }
            }
            acc
        }
        //modifiers.is_empty() || self.modifiers.iter().all(|x| modifiers.contains(x))
    }
}

fn parse_editor_colors(table: &toml::Value) -> Result<EditorColors, String> {
    let mut editor_colors = EditorColors::default();

    if let Some(buffer_color) = table.get("buffer_color") {
        editor_colors.buffer_color = crate::models::style::parse_style(buffer_color)?;
    }

    if let Some(selected) = table.get("selected") {
        editor_colors.selected = crate::models::style::parse_style(selected)?;
    }

    if let Some(number_bar) = table.get("number_bar") {
        editor_colors.number_bar = parse_number_bar_color(number_bar)?;
    }

    if let Some(status_bar) = table.get("status_bar") {
        editor_colors.status_bar = parse_status_bar_color(status_bar)?;
    }

    if let Some(rainbow_delimiters) = table.get("rainbow_delimiters") {
        editor_colors.rainbow_delimiters = parse_rainbow_delimiters(rainbow_delimiters)?;
    }

    Ok(editor_colors)
}

fn parse_rainbow_delimiters(list: &toml::Value) -> Result<Vec<Style>, String> {
    let mut rainbow_delimiters = Vec::new();

    if list.is_array() {
        let list = list.as_array().ok_or("rainbow_delimiters was not an array".to_string())?;

        for value in list {
            if value.is_table() {

                rainbow_delimiters.push(crate::models::style::parse_style(value)?);
            } else {
                return Err("rainbow_delimiters was not a table".to_string());
            }
        }
    } else {
        return Err("rainbow_delimiters was not an array".to_string());
    }

    Ok(rainbow_delimiters)
}


impl Default for NumberBarColor {
    fn default() -> Self {
        NumberBarColor {
            current_line: Style::new().bg(Color::DarkGray),
            other_lines: Style::new().fg(Color::DarkGray),
        }
    }
}

pub struct NumberBarColor {
    pub current_line: Style,
    pub other_lines: Style,
}

impl NumberBarColor {
    pub fn config_file(&self) -> String {
        format!("[number_bar.current_line]\n{}\n[number_bar.other_lines]\n{}",
                self.current_line.config_file(),
                self.other_lines.config_file())
    }

    fn merge(&mut self, other: Self) {
        self.current_line.patch(other.current_line);
        self.other_lines.patch(other.other_lines);
    }
}

fn parse_number_bar_color(table: &toml::Value) -> Result<NumberBarColor, String> {
    let mut number_bar_color = NumberBarColor::default();

    if let Some(current_line) = table.get("current_line") {
        number_bar_color.current_line = crate::models::style::parse_style(current_line)?;
    }

    if let Some(other_lines) = table.get("other_lines") {
        number_bar_color.other_lines = crate::models::style::parse_style(other_lines)?;
    }

    Ok(number_bar_color)
}

impl Default for StatusBarColor {
    fn default() -> Self {

        let mut mode = HashMap::new();

        mode.insert("Normal".to_string(), Style::new().fg(Color::Black).bg(Color::LightCyan));
        mode.insert("Insert".to_string(), Style::new().fg(Color::Black).bg(Color::LightGreen));
        mode.insert("Selection".to_string(), Style::new().fg(Color::Black).bg(Color::LightYellow));
        mode.insert("Command".to_string(), Style::new().fg(Color::Black).bg(Color::LightMagenta));
        mode.insert("Search".to_string(), Style::new().fg(Color::Black).bg(Color::LightBlue));
        mode.insert("Replace".to_string(), Style::new().fg(Color::Black).bg(Color::LightRed));
        mode.insert("Mirror".to_string(), Style::new().fg(Color::Black).bg(Color::Gray));
        mode.insert("Pair".to_string(), Style::new().fg(Color::Black).bg(Color::White));

        StatusBarColor {
            message: Style::new().bg(Color::DarkGray),
            mode,
            first: Style::new(),
            second: Style::new(),
            rest: Style::new(),
        }
    }
}


pub struct StatusBarColor {
    pub message : Style,
    pub mode: HashMap<String, Style>,
    pub first: Style,
    pub second: Style,
    pub rest: Style,
}

impl StatusBarColor {
    pub fn config_file(&self) -> String {
        let mut output = String::from("[StatusBar]\n");

        output.push_str(&format!("[StatusBar.message]\n{}\n", self.message.config_file()));

        output.push_str(&format!("[StatusBar.first]\n{}\n", self.first.config_file()));

        output.push_str(&format!("[StatusBar.second]\n{}\n", self.second.config_file()));

        output.push_str(&format!("[StatusBar.rest]\n{}\n", self.rest.config_file()));

        self.mode.iter().for_each(|(k, v)| {
            output.push_str(&format!("[StatusBar.mode.{}]\n{}\n", k, v.config_file()));
        });

        /*output.push_str(&format!("[StatusBar.mode] = {{\n{}\n}}\n",
                                 self.mode.iter()
                                     .map(|(k, v)| format!("{} = {}", k, v.config_file()))
                                     .collect::<Vec<String>>()
                                     .join("\n")));*/

        output
    }

    fn merge(&mut self, other: Self) {
        self.message.patch(other.message);
        self.first.patch(other.first);
        self.second.patch(other.second);
        self.rest.patch(other.rest);
        self.mode.extend(other.mode);
    }
}

fn parse_status_bar_color(table: &toml::Value) -> Result<StatusBarColor, String> {
    let mut status_bar_color = StatusBarColor::default();

    if let Some(message) = table.get("message") {
        status_bar_color.message = crate::models::style::parse_style(message)?;
    }

    if let Some(mode) = table.get("mode") {
        status_bar_color.mode = parse_mode(mode)?;
    }

    if let Some(first) = table.get("first") {
        status_bar_color.first = crate::models::style::parse_style(first)?;
    }

    if let Some(second) = table.get("second") {
        status_bar_color.second = crate::models::style::parse_style(second)?;
    }

    if let Some(rest) = table.get("rest") {
        status_bar_color.rest = crate::models::style::parse_style(rest)?;
    }

    Ok(status_bar_color)
}

fn parse_mode(table: &toml::Value) -> Result<HashMap<String, Style>, String> {
    let mut mode = HashMap::new();

    if table.is_table() {
        let table = table.as_table().ok_or("mode was not a table".to_string())?;

        for (k, v) in table {
            if v.is_table() {
                mode.insert(k.clone(), crate::models::style::parse_style(v)?);
            } else {
                return Err("mode was not a table".to_string());
            }
        }
    } else {
        return Err("mode was not a table".to_string());
    }

    Ok(mode)
}













