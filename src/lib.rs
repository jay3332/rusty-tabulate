mod models;
mod styles;

pub use models::{AlignType, LineStyle, Style, WallStyle};
pub use styles::get_style;

pub struct Tabulator {
    pub style: Style,
    pub headers: Vec<&'static str>,
    pub rows: Vec<Vec<&'static str>>,
}

impl Tabulator {
    pub fn new(style: Style) -> Tabulator {
        Tabulator {
            style,
            headers: Vec::<&'static str>::new(),
            rows: Vec::<Vec<&'static str>>::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<&'static str>) -> () {
        self.rows.push(row);
    }

    pub fn render(&mut self) -> &'static str {
        "s"
    }
}

macro_rules! tabulator {
    ($style: expr, $headers: expr, $rows: expr) => {{
        let mut tabulator: Tabulator = Tabulator::new($style);
        tabulator.headers = $headers;
        for row in $rows {
            tabulator.add_row(row);
        }
        tabulator
    }};
    ($style: expr, $headers: expr) => {{
        let mut tabulator: Tabulator = Tabulator::new($style);
        tabulator.headers = $headers;
        tabulator
    }};
    ($style: expr) => {
        Tabulator::new($style)
    };
    () => {
        Tabulator::new(get_style("default").unwrap())
    };
}

pub fn test() {
    let tab = tabulator!(
        get_style("default").unwrap(),
        vec!["header1", "header2"],
        vec![vec!["test", "test2"], vec!["test3", "test4"]]
    );
    println!("{}", tab.render());
}
