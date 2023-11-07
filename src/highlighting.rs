use termion::color;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self, colors: &HashMap<String, color::Rgb>) -> impl color::Color {
        match self {
            Type::Number => colors["color1"],
            Type::Match => colors["color0"],
            Type::String => colors["color2"],
            Type::Character => colors["color3"],
            Type::Comment | Type::MultilineComment => colors["color6"],
            Type::PrimaryKeywords => colors["color4"],
            Type::SecondaryKeywords => colors["color5"],
            _ => colors["color7"],
        }
    }
}
