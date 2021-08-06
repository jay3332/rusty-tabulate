pub struct LineStyle {
    left: Option<&'static str>,
    middle: Option<&'static str>,
    sep: Option<&'static str>,
    right: Option<&'static str>,
}

pub struct WallStyle {
    left: Option<&'static str>,
    sep: Option<&'static str>,
    right: Option<&'static str>
}

#[non_exhaustive]
pub struct AlignType;

impl AlignType {
    pub const LEFT: u8 = 0;
    pub const CENTER: u8 = 1;
    pub const RIGHT: u8 = 2;
}

pub struct Style {
    prefix: Option<&'static str>,
    suffix: Option<&'static str>,
    above: Option<LineStyle>,
    header_sep: Option<LineStyle>,
    row_sep: Option<LineStyle>,
    below: Option<LineStyle>,
    header_wall: Option<WallStyle>,
    row_wall: Option<WallStyle>,
    header_align: u8,
    row_align: u8,
    padding: u8
}
