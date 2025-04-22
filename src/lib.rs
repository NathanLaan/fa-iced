//!
//! Rust-Iced-FA
//!
//! Wrapper for Font Awesome icons.
//!

use std::borrow::Cow;
use iced::advanced::graphics::text::font_system;
use iced::{Element, Font};
use iced::font::Family;
use iced::widget::text;
use std::fmt;
use std::str::FromStr;

///
/// Load Font Awesome files. Should only be called once.
///
pub fn load_font_fontawesome() {
    let mut font_system = font_system().write().unwrap();
    font_system.load_font(Cow::from(FONT_DATA_FA_REGULAR));
    font_system.load_font(Cow::from(FONT_DATA_FA_BRANDS));
    font_system.load_font(Cow::from(FONT_DATA_FA_SOLID));
}

///
/// Create an iced `text` element containing the specified Font Awesome icon.
///
/// Uses `FONT_FA_REGULAR`.
///
pub fn iced_text_icon<'a, Message>(code: &str) -> Element<'a, Message> {
    let code_u32 = u32::from_str_radix(&code, 16).unwrap();
    let unicode_char = char::from_u32(code_u32).unwrap();
    text(unicode_char).font(FONT_FA_REGULAR).into()
}

///
/// The "Regular" version of Font Awesome version 6.
///
pub const FONT_FA_REGULAR: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};

//
// Font data
//

const FONT_DATA_FA_REGULAR: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-regular-400.otf");

const FONT_DATA_FA_BRANDS: &[u8] =
    include_bytes!("../fonts/font-awesome-6-brands-regular-400.otf");

const FONT_DATA_FA_SOLID: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-solid-900.otf");

//
// File operations
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/user`.
pub const FA_ICON_USER: &str = "f007";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/file`.
pub const FA_ICON_NEW: &str = "f15b";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/folder-open`.
pub const FA_ICON_OPEN: &str = "f07c";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/floppy-disk`.
pub const FA_ICON_SAVE: &str = "f0c7";

//
// Numbers
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/0`.
pub const FA_ICON_0: &str = "30";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/1`.
pub const FA_ICON_1: &str = "31";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/2`.
pub const FA_ICON_2: &str = "32";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/3`.
pub const FA_ICON_3: &str = "33";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/4`.
pub const FA_ICON_4: &str = "34";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/5`.
pub const FA_ICON_5: &str = "35";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/6`.
pub const FA_ICON_6: &str = "36";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/7`.
pub const FA_ICON_7: &str = "37";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/8`.
pub const FA_ICON_8: &str = "38";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/9`.
pub const FA_ICON_9: &str = "39";

//
// Circle
//

/// Font Awesome Unicode string for https://fontawesome.com/icons/circle-check
pub const FA_ICON_CIRCLE_CHECK: &str = "f058";

/// Font Awesome Unicode string for https://fontawesome.com/icons/circle-xmark
pub const FA_ICON_CIRCLE_XMARK: &str = "f057";

//
// Settings/Options/Utility
//

/// Font Awesome Unicode string for https://fontawesome.com/icons/bars
pub const FA_ICON_BARS: &str = "f0c9";

/// Font Awesome Unicode string for https://fontawesome.com/icons/gear
pub const FA_ICON_GEAR: &str = "f013";

/// Font Awesome Unicode string for https://fontawesome.com/icons/screwdriver-wrench
pub const FA_ICON_SCREWDRIVER_WRENCH: &str = "f7d9";


///
/// Implementation of Font Awesome icons as enum.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaIcon {
    User,
    CircleCheck,
    CircleXmark,
    ScrewdriverWrench,
    Gear,
    Bars,
    New,
    Open,
    Save,
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
}

impl fmt::Display for FaIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FaIcon {
    pub fn as_str(&self) -> &str {
        match self {
            FaIcon::User => "f007",
            FaIcon::CircleCheck => "f058",
            FaIcon::CircleXmark => "f057",
            FaIcon::ScrewdriverWrench => "f7d9",
            FaIcon::Gear => "f013",
            FaIcon::Bars => "f0c9",
            FaIcon::New => "f15b",
            FaIcon::Open => "f07c",
            FaIcon::Save => "f0c7",
            FaIcon::Number0 => "30",
            FaIcon::Number1 => "31",
            FaIcon::Number2 => "32",
            FaIcon::Number3 => "33",
            FaIcon::Number4 => "34",
            FaIcon::Number5 => "35",
            FaIcon::Number6 => "36",
            FaIcon::Number7 => "37",
            FaIcon::Number8 => "38",
            FaIcon::Number9 => "39",
        }
    }
}
