use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{layout::{Alignment, Constraint}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, TableState}};

use crate::components::{Component, StatefullDrawableComp};

//|-----------------{Flags >ᴗ<}------------------|

#[derive(PartialEq, Eq)]
enum Flag {
    ShowTable,
    ShowMsg,
}

//|-----------------{Table Comp( . .)φ}------------------|

pub struct TableComp {
    table_state: TableState, 
    ids: Vec<String>,
    colums: Vec<Vec<bool>>, 
    flag: Flag,
    focus: bool,
}

impl TableComp {
    pub fn new(ids: Vec<String>, colums: Vec<Vec<bool>>, focus: bool) -> TableComp {
        TableComp { table_state: TableState::default(), ids, colums, flag: Flag::ShowMsg, focus: focus }
    }

    pub fn update(&mut self, ids: Vec<String>, colums: Vec<Vec<bool>>) {
        self.ids = ids;
        self.colums = colums;
        self.flag = Flag::ShowTable;
        self.table_state.select(Some(0));
    }

    pub fn no_table(&mut self) {
        self.ids.clear();
        self.colums.clear();
        self.flag = Flag::ShowMsg;
        //self.table_state.
    }

    pub fn get_keybinds(&self) -> String{
        "↑ ↓ → ← : To navegate ".to_string()
    }
}

impl Default for TableComp {
    fn default() -> Self {
        TableComp { table_state: TableState::new(), ids: Vec::new(), colums: Vec::new(), flag: Flag::ShowMsg, focus: false }
    }
}

impl StatefullDrawableComp for TableComp {
    fn draw(&mut self, f: &mut ratatui::prelude::Frame, rect: ratatui::prelude::Rect) {
        //Now cats guide us (=^-ω-^=)
        if self.flag == Flag::ShowTable {
            let header = Row::new(self.ids.iter().map(|item| Cell::from(Line::from(item.clone()).alignment(Alignment::Center)))).style(
                if self.focus {
                    Style::new().bg(Color::Rgb(36, 26, 26))
                } else {
                    Style::default()
                }
            );    
            let rows_n = self.colums[0].len();

            let rows: Vec<Row> = (0..rows_n).map(|row_indx|{let cells: Vec<Line> = self.colums.iter()
                .map(|col| if col[row_indx] {Line::from("T").fg(Color::Green).alignment(Alignment::Center)} else {Line::from("F").fg(Color::Red).alignment(Alignment::Center)})
                .collect(); Row::new(cells)}).collect();
            
            let widths: Vec<Constraint> = self.ids.iter().map(|cell| Constraint::Percentage(100/cell.chars().count() as u16))
                .collect();

            let table = Table::new(rows, widths).block(Block::new().borders(Borders::ALL)).header(header)
                .cell_highlight_style(Style::new().bg(Color::Rgb(59, 8, 3))).row_highlight_style(Style::new().bg(Color::Rgb(36, 10, 7))).highlight_symbol(">>");

            f.render_stateful_widget(table, rect, &mut self.table_state);

        }else if self.flag == Flag::ShowMsg {
            let text = "(^０^)ノ Hiiii";
            let text_lines = text.lines().count() as u16;
            let block_height = rect.height.saturating_sub(2);
            let ver_padding = block_height.saturating_sub(text_lines) / 2;

            let text = Paragraph::new(text).block(Block::new().borders(Borders::ALL).padding(Padding::vertical(ver_padding))).alignment(Alignment::Center);
            f.render_widget(text, rect);
        }
    }
}

impl Component for TableComp {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Up => {self.table_state.select_previous();},
                KeyCode::Down => {self.table_state.select_next();},
                KeyCode::Left => {self.table_state.select_previous_column();}
                KeyCode::Right => {self.table_state.select_next_column();}
                _ => {},
            }
        }

        Ok(())    
    }

    fn focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}