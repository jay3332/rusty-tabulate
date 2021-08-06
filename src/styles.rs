use crate::{ LineStyle, WallStyle, Style, AlignType };


const _default_style: Style = Style {
    prefix: None,
    suffix: None,
    above: Some(LineStyle {
        left: Some("+"),
        middle: Some("-"),
        sep: Some("+"),
        right: Some("+")
    }),
    header_sep: Some(LineStyle {
        left: Some("+"),
        middle: Some("-"),
        sep: Some("+"),
        right: Some("+")
    }),
    row_sep: None,
    below: Some(LineStyle {
        left: Some("+"),
        middle: Some("-"),
        sep: Some("+"),
        right: Some("+")
    }),
    header_wall: Some(WallStyle {
        left: Some("|"),
        sep: Some("|"),
        right: Some("|")
    }),
    row_wall: Some(WallStyle {
        left: Some("|"),
        sep: Some("|"),
        right: Some("|")
    }),
    header_align: AlignType::CENTER,
    row_align: AlignType::CENTER,
    padding: 1
};


pub fn get_style(style: &'static str) -> Result<Style, &'static str> {
    match style {
        "default" => Ok(_default_style),
        _ => Err(format!("Unknown style: {}", style))
    }
} 
