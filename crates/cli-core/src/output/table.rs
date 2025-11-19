use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement, presets::UTF8_FULL};

pub struct TableFormatter;

impl TableFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn create_table() -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);
        table
    }

    pub fn header_cell(text: impl ToString) -> Cell {
        Cell::new(text.to_string())
            .fg(Color::Cyan)
            .add_attribute(Attribute::Bold)
    }

    pub fn value_cell(text: impl ToString) -> Cell {
        Cell::new(text.to_string())
    }

    pub fn highlight_cell(text: impl ToString) -> Cell {
        Cell::new(text.to_string())
            .fg(Color::Green)
            .add_attribute(Attribute::Bold)
    }
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self::new()
    }
}
