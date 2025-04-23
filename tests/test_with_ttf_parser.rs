//!
//! Test flow:
//!
//! test_font_data_fa_regular
//!     --> test_font
//!         --> test_icon
//!
//! test_font_data_fa_solid
//!     --> test_font
//!         --> test_icon
//!

use ttf_parser::{Face, name_id};
use fa_iced::*;




#[test]
fn check_fa_font_names() {
    print_font_names(crate::FONT_DATA_FA_REGULAR_TTF);
    print_font_names(crate::FONT_DATA_FA_SOLID_TTF);
}

fn print_font_names(data: &[u8]) {
    let face = Face::parse(data, 0)
        .expect("Failed to parse font data");

    let family_name = face.names().into_iter()
        .find(|name| name.name_id == name_id::FULL_NAME)
        .and_then(|name| name.to_string())
        .unwrap_or_else(|| "UNKNOWN".into());

    println!("Full name: {}", family_name);

    let font_family = face.names().into_iter()
        .find(|name| name.name_id == name_id::FAMILY)
        .and_then(|name| name.to_string())
        .unwrap_or_else(|| "UNKNOWN".into());

    println!("Family: {}", font_family);

    let subfamily = face.names().into_iter()
        .find(|name| name.name_id == name_id::SUBFAMILY)
        .and_then(|name| name.to_string())
        .unwrap_or_else(|| "UNKNOWN".into());

    println!("Subfamily: {}", subfamily);
}

#[test]
fn output_all_names() {
    output_all_names_for(crate::FONT_DATA_FA_REGULAR_OTF);
    output_all_names_for(crate::FONT_DATA_FA_SOLID_OTF);
}

fn output_all_names_for(data: &[u8]) {
    let face = Face::parse(data, 0)
        .expect("Failed to parse font data");

    for name in face.names().into_iter() {
        println!(
            "Name ID: {:<2} | Platform: {:?} | Encoding: {:<2} | Language: {:<2} | Raw bytes: {:?}",
            name.name_id,
            name.platform_id,
            name.encoding_id,
            name.language_id,
            name.name
        );

        if let Some(string) = name.to_string() {
            println!(" → Decoded: {}", string);
        } else {
            println!(" → Could not decode.");
        }

        println!("---");
    }
}


#[test]
#[ignore]
fn test_font_data_fa_regular() {
    load_font_fontawesome();
    test_font(FONT_DATA_FA_REGULAR_OTF);
}

#[test]
fn test_font_data_fa_solid() {
    load_font_fontawesome();
    test_font(FONT_DATA_FA_SOLID_OTF);
}


fn test_font(font_data: &[u8]) {
    load_font_fontawesome();
    // Parse the font from raw bytes
    let face = Face::parse(font_data, 0)
        .expect("Failed to parse font data");

    test_icon(&face, FA_ICON_0);
    test_icon(&face, FA_ICON_1);
    test_icon(&face, FA_ICON_2);
    test_icon(&face, FA_ICON_3);
    test_icon(&face, FA_ICON_4);
    test_icon(&face, FA_ICON_5);
    test_icon(&face, FA_ICON_5);
    test_icon(&face, FA_ICON_6);
    test_icon(&face, FA_ICON_7);
    test_icon(&face, FA_ICON_8);
    test_icon(&face, FA_ICON_9);
    test_icon(&face, FA_ICON_NEW);
    test_icon(&face, FA_ICON_SAVE);
    test_icon(&face, FA_ICON_OPEN);
    test_icon(&face, FA_ICON_USER);
    test_icon(&face, FA_ICON_SCREWDRIVER_WRENCH);
    test_icon(&face, FA_ICON_BARS);
    test_icon(&face, FA_ICON_GEAR);
}


fn test_icon(face: &Face, icon: &str) {

    // Convert the Font Awesome user icon hex code into a char
    let code_u32 = u32::from_str_radix(icon, 16)
        .expect("Invalid icon code");
    let icon_char = char::from_u32(code_u32)
        .expect("Invalid unicode char");

    // Check that the glyph for the icon character exists
    let glyph_id = face.glyph_index(icon_char);

    assert!(
        glyph_id.is_some(),
        "Glyph for icon '{}' (U+{:X}) not found in font",
        icon_char, code_u32
    );
}
