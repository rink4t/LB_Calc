use ratatui::{layout::{Alignment, Constraint}, style::Style, symbols::block, widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, TableState}};

use crate::components::{StatefullDrawableComp};

//|-----------------{Table Comp( . .)φ}------------------|

pub struct TableComp {
    table_state: TableState, 
    ids: Vec<String>,
    colums: Vec<Vec<bool>>, 
}

impl TableComp {
    pub fn new(ids: Vec<String>, colums: Vec<Vec<bool>>) -> TableComp {
        TableComp { table_state: TableState::new().with_selected_cell(Some((0,1))), ids, colums }
    }

    pub fn update(&mut self, ids: Vec<String>, colums: Vec<Vec<bool>>) {
        self.ids = ids;
        self.colums = colums;
    }
}

impl Default for TableComp {
    fn default() -> Self {
        TableComp { table_state: TableState::new(), ids: Vec::new(), colums: Vec::new() }
    }
}

impl StatefullDrawableComp for TableComp {
    fn draw(&mut self, f: &mut ratatui::prelude::Frame, rect: ratatui::prelude::Rect) {

        //Now cats guide us (=^-ω-^=)
        if !self.ids.is_empty() && !self.colums.is_empty() {
            let header = Row::new(self.ids.clone());    
            let rows_n = self.colums[0].len();

            //println!("{} : {}", self.colums[0].len(), rows_n);

            let rows: Vec<Row> = (0..rows_n).map(|row_indx|{let cells: Vec<String> = self.colums.iter()
                .map(|col| col[row_indx].to_string()).collect(); Row::new(cells)}).collect();
            
            let widths: Vec<Constraint> = self.ids.iter().map(|cell| Constraint::Percentage(100/1 as u16))
                .collect();

            let table = Table::new(rows, widths).block(Block::new().borders(Borders::ALL)).header(header)
                .cell_highlight_style(Style::new().bg(ratatui::style::Color::Blue)).row_highlight_style(Style::new().cyan()).highlight_symbol(">>");

            f.render_stateful_widget(table, rect, &mut self.table_state);
        }else {

            let text = "(^０^)ノ Hiiii";
            let text_lines = text.lines().count() as u16;
            let block_height = rect.height.saturating_sub(2);
            let ver_padding = block_height.saturating_sub(text_lines) / 2;

            let text = Paragraph::new(text).block(Block::new().borders(Borders::ALL).padding(Padding::vertical(ver_padding))).alignment(Alignment::Center);
            f.render_widget(text, rect);
        }
    }
}