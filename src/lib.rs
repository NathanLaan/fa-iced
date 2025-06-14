//!
//! Rust-Iced-FA
//!
//! Wrapper for Font Awesome icons.
//!

use iced::advanced::graphics::text::font_system;
use iced::font::{Family, Weight};
use iced::widget::text;
use iced::{Element, Font};
use std::borrow::Cow;
use std::fmt;

//
// TODO: FUTURE API
//
// pub enum FaStyle {
//     Regular,
//     Solid,
//     Brands,
// }
// pub const FONT_FA_REGULAR: Font = Font {
//     family: Family::Name("Font Awesome 6 Free"),
//     ..Font::DEFAULT
// };
//
// pub const FONT_FA_SOLID: Font = Font {
//     family: Family::Name("Font Awesome 6 Free Solid"),
//     ..Font::DEFAULT
// };
//
// pub const FONT_FA_BRANDS: Font = Font {
//     family: Family::Name("Font Awesome 6 Brands"),
//     ..Font::DEFAULT
// };
//
// pub fn fa_font(style: FaStyle) -> Font {
//     match style {
//         FaStyle::Regular => FONT_FA_REGULAR,
//         FaStyle::Solid => FONT_FA_SOLID,
//         FaStyle::Brands => FONT_FA_BRANDS,
//     }
// }
//
// pub fn iced_fa_icon<'a, Message>(code: &str, style: FaStyle) -> Element<'a, Message> {
//     let code_u32 = u32::from_str_radix(code, 16).unwrap();
//     let unicode_char = char::from_u32(code_u32).unwrap();
//
//     text(unicode_char)
//         .font(fa_font(style))
//         .size(32)
//         .into()
// }

///
/// Load Font Awesome TTF files. Should only be called once.
///
pub fn load_font_fontawesome_ttf() {
    let mut font_system = font_system().write().unwrap();
    font_system.load_font(Cow::from(FONT_DATA_FA_REGULAR_TTF));
    font_system.load_font(Cow::from(FONT_DATA_FA_BRANDS_TTF));
    font_system.load_font(Cow::from(FONT_DATA_FA_SOLID_TTF));
}

///
/// Load Font Awesome OTF files. Should only be called once.
///
pub fn load_font_fontawesome_otf() {
    let mut font_system = font_system().write().unwrap();
    font_system.load_font(Cow::from(FONT_DATA_FA_REGULAR_TTF));
    font_system.load_font(Cow::from(FONT_DATA_FA_BRANDS_TTF));
    font_system.load_font(Cow::from(FONT_DATA_FA_SOLID_TTF));
}

///
/// Create an iced `text` element containing the specified Font Awesome icon.
///
/// Uses `FONT_FA_REGULAR`.
///
pub fn iced_text_icon_regular<'a, Message>(code: &str, size: u16) -> Element<'a, Message> {
    text(fa_unicode_char(&code))
        .font(FONT_FA_REGULAR)
        .size(size)
        .into()
}

///
/// Create an iced `text` element containing the specified Font Awesome icon.
///
/// Uses `FONT_FA_SOLID`.
///
pub fn iced_text_icon_solid<'a, Message>(code: &str, size: u16) -> Element<'a, Message> {
    text(fa_unicode_char(&code))
        .font(FONT_FA_SOLID)
        .size(size)
        .into()
}

///
/// Convert the Font Awesome `code` into the corresponding Unicode char.
///
pub fn fa_unicode_char(code: &str) -> char {
    let code_u32 = u32::from_str_radix(&code, 16).unwrap();
    let unicode_char = char::from_u32(code_u32).unwrap();
    unicode_char
}

///
/// The "Regular" version of Font Awesome version 6 Free.
///
pub const FONT_FA_REGULAR: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};

///
/// The "Solid" version of Font Awesome version 6 Free.
///
pub const FONT_FA_SOLID: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    weight: Weight::Black, // Solid weights are bold
    ..Font::DEFAULT
};

//
// Font data
//

///
/// Font data for Font Awesome Regular OTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_REGULAR_OTF: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-regular-400.otf");

///
/// Font data for Font Awesome Brands OTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_BRANDS_OTF: &[u8] =
    include_bytes!("../fonts/font-awesome-6-brands-regular-400.otf");

///
/// Font data for Font Awesome Solid OTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_SOLID_OTF: &[u8] =
    include_bytes!("../fonts/font-awesome-6-free-solid-900.otf");

///
/// Font data for Font Awesome Regular TTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_REGULAR_TTF: &[u8] = include_bytes!("../fonts/fa-regular-400.ttf");

///
/// Font data for Font Awesome Brands TTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_BRANDS_TTF: &[u8] = include_bytes!("../fonts/fa-brands-400.ttf");

///
/// Font data for Font Awesome Solid TTF. Loaded using `include_bytes!()`.
///
pub const FONT_DATA_FA_SOLID_TTF: &[u8] = include_bytes!("../fonts/fa-solid-900.ttf");

//
// File operations
//

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

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-check`
pub const FA_ICON_CIRCLE_CHECK: &str = "f058";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-xmark`
pub const FA_ICON_CIRCLE_XMARK: &str = "f057";

//
// Settings/Options/Utility
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/bars`
pub const FA_ICON_BARS: &str = "f0c9";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/gear`
///
/// Only available in SOLID variant.
pub const FA_ICON_GEAR: &str = "f013";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/screwdriver-wrench`
///
/// Only available in SOLID variant.
pub const FA_ICON_SCREWDRIVER_WRENCH: &str = "f7d9";

//
// ID cards and badges
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/id-card`
pub const FA_ICON_ID_CARD: &str = "f2c2";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/id-card-clip`
pub const FA_ICON_ID_CARD_CLIP: &str = "f47f";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/id-badge`
pub const FA_ICON_ID_BADGE: &str = "f2c1";

//
// Users
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/user`.
pub const FA_ICON_USER: &str = "f007";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/users`.
pub const FA_ICON_USERS: &str = "f0c0";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/users-between-lines`.
pub const FA_ICON_USERS_BETWEEN_LINES: &str = "e591";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/users-line`.
pub const FA_ICON_USERS_LINE: &str = "e592";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/users-rectangle`.
pub const FA_ICON_USERS_RECTANGLE: &str = "e594";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/user_minus`
pub const FA_ICON_USER_MINUS: &str = "f503";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/user_plus`
pub const FA_ICON_USER_PLUS: &str = "f234";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/user_xmark`
pub const FA_ICON_USER_XMARK: &str = "f235";

//
// Edit
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/pen-to-square`
pub const FA_ICON_EDIT: &str = "f044";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/minus`
pub const FA_ICON_MINUS: &str = "f068";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/square-minus`
pub const FA_ICON_SQUARE_MINUS: &str = "f146";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-minus`
pub const FA_ICON_CIRCLE_MINUS: &str = "f056";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/shield-minus`
pub const FA_ICON_SHIELD_MINUS: &str = "e249";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/plus`
pub const FA_ICON_PLUS: &str = "2b";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/square-plus`
pub const FA_ICON_SQUARE_PLUS: &str = "f0fe";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-plus`
pub const FA_ICON_CIRCLE_PLUS: &str = "f055";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/shield-plus`
pub const FA_ICON_SHIELD_PLUS: &str = "e24a";

//
// List Icons
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list`
pub const FA_ICON_LIST: &str = "f03a";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list-ul`
pub const FA_ICON_LIST_UL: &str = "f0ca";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list-ol`
pub const FA_ICON_LIST_OL: &str = "f0cb";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list-tree`
pub const FA_ICON_LIST_TREE: &str = "e1d2";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list-timeline`
pub const FA_ICON_LIST_TIMELINE: &str = "e1d1";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/list-radio`
pub const FA_ICON_LIST_RADIO: &str = "e1d0";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/table-list`
pub const FA_ICON_TABLE_LIST: &str = "f00b";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/square-list`
pub const FA_ICON_SQUARE_LIST: &str = "e489";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/rectangle-list`
pub const FA_ICON_RECTANGLE_LIST: &str = "f022";

//
// Info
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/info`
pub const FA_ICON_INFO: &str = "f129";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-info`
pub const FA_ICON_CIRCLE_INFO: &str = "f05a";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/square-info`
pub const FA_ICON_SQUARE_INFO: &str = "f30f";

//
// Question
//

/// Font Awesome Unicode string for `https://fontawesome.com/icons/question`
pub const FA_ICON_QUESTION: &str = "3f";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/circle-question`
pub const FA_ICON_CIRCLE_QUESTION: &str = "f059";

/// Font Awesome Unicode string for `https://fontawesome.com/icons/square-question`
pub const FA_ICON_SQUARE_QUESTION: &str = "f2fd";

//
// FaIcon Enum
//

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
    IdCard,
    IdCardClip,
    IdBadge,
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
            FaIcon::IdCard => "f2c2",
            FaIcon::IdCardClip => "f47f",
            FaIcon::IdBadge => "f2c1",
        }
    }
}
