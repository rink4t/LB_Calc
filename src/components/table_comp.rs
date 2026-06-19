use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{layout::{Alignment, Constraint}, style::{Color, Style, Stylize}, text::{Line}, widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table}};

use crate::components::{Component, DrawableComp};

//|-----------------{Flags >ᴗ<}------------------|

#[derive(PartialEq, Eq)]
enum Flag {
    ShowTable,
    ShowMsg,
}

//|-----------------{Table Comp( . .)φ}------------------|

pub struct TableComp {
    ids: Vec<String>,
    colums: Vec<Vec<bool>>, 
    flag: Flag,
    focus: bool,
    offset: usize,
    height: usize,
    selected: usize,
    selected_colum: usize,
}

impl TableComp {
    pub fn _new(ids: Vec<String>, colums: Vec<Vec<bool>>, focus: bool) -> TableComp {
        TableComp { ids, colums, flag: Flag::ShowMsg, focus: focus, offset: 0, height: 0, selected: 0, selected_colum: 0}
    }

    pub fn update(&mut self, ids: Vec<String>, colums: Vec<Vec<bool>>) {
        self.ids = ids;
        self.colums = colums;
        self.offset = 0;
        self.selected = 0;
        self.flag = Flag::ShowTable;
    }

    pub fn no_table(&mut self) {
        self.ids.clear();
        self.colums.clear();
        self.flag = Flag::ShowMsg;
    }

    fn next_row(&mut self) {

        if self.flag == Flag::ShowMsg { return; }

        let rows_t = self.colums[0].len();
        let next_row = if self.selected + 1 < rows_t { self.selected + 1 } else { self.selected };
    
        self.selected = next_row;
        
        if next_row >= self.offset + self.height {
            self.offset = (next_row.saturating_sub(self.height)) + 1;
        }
        //println!("{} h{}", self.height, height);
        if self.offset + self.height > rows_t {
            self.offset = rows_t.saturating_sub(self.height);       
        }
    }

    fn prev_row(&mut self) {

        if self.selected == 0 {return;}

        if self.selected <= self.offset.saturating_sub(self.height) {
            self.offset = self.offset.saturating_sub(1);
        }

        self.selected = self.selected.saturating_sub(1);

        if self.selected < self.offset {
            self.offset = self.selected;
        }
    }

    fn next_colum(&mut self) {
        if self.colums.is_empty() { return; }

        if self.selected_colum >= self.ids.len().saturating_sub(1) {
            return;
        }

        self.selected_colum += 1;

    }

    fn previous_column(&mut self) {
        if self.colums.is_empty() { return; }

        if self.selected_colum == 0 {
            return;
        }

        self.selected_colum -= 1;
    }

    fn go_end(&mut self) {
        if self.colums.is_empty() { return; }

        let rows_t = self.colums[0].len();

        self.selected = (rows_t).saturating_sub(1);
        
        if self.selected >= self.offset + self.height {
            self.offset = (self.selected.saturating_sub(self.height)) + 1;
        }
  
        if self.offset + self.height > rows_t {
            self.offset = rows_t.saturating_sub(self.height);       
        }
    }

    fn go_top(&mut self){
        if self.colums.is_empty() || self.selected == 0{ return; }

        self.selected = 0;

        if self.selected < self.offset {
            self.offset = self.selected;
        }

    }

    pub fn get_keybinds(&self) -> String{
        "↑ ↓ → ← : To navegate ".to_string()
    }
}

impl Default for TableComp {
    fn default() -> Self {
        TableComp { ids: Vec::new(), colums: Vec::new(), flag: Flag::ShowMsg, focus: false, offset: 0, height: 0, selected: 0, selected_colum: 0}
    }
}

impl DrawableComp for TableComp {
    fn draw(&mut self, f: &mut ratatui::prelude::Frame, rect: ratatui::prelude::Rect) {
        //Now cats guide us (=^-ω-^=)
        
        let height = (rect.height.saturating_sub(3)) as usize;
        self.height = height;

        if self.flag == Flag::ShowTable {
            let header = Row::new(self.ids.iter().map(|item| Cell::from(Line::from(item.clone()).alignment(Alignment::Center)))).style(
                if self.focus {
                    Style::new().bg(Color::Rgb(36, 26, 26))
                } else {
                    Style::default()
                }
            );    

            let start = self.offset;
            let rows_t = self.colums[0].len();
            let end = (start + height).min(rows_t);

            let rows: Vec<Row> = (start..end).map(|row_indx|{
                let cells: Vec<Line> = self.colums
                .iter().enumerate().map(|(col_indx,col)| {
                    
                    let val = if col[row_indx] {"T"} else {"F"};
                    let color = if col[row_indx] { Color::Green } else { Color::Red };
                    let mut line = Line::from(val).fg(color).alignment(Alignment::Center);

                    if row_indx == self.selected && col_indx == self.selected_colum {
                        line = line.bg(Color::Rgb(99, 9, 0))
                    }

                    line
                }).collect(); 
                let mut row = Row::new(cells);
                if row_indx == self.selected {
                    row = row.bg(Color::Rgb(36, 10, 7));
                }
                row
                })
            .collect();

            let widths: Vec<Constraint> = self.ids.iter().map(|cell| 
                if cell.chars().count() >= 2 {
                    Constraint::Min(cell.chars().count() as u16)
                }else {
                    Constraint::Min(3)
                }
                
            ).collect();

            let table = Table::new(rows, widths).block(Block::new().borders(Borders::ALL)).header(header);
            f.render_widget(table, rect);

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
                KeyCode::Up => { self.prev_row(); },
                KeyCode::Down => { self.next_row(); },
                KeyCode::Left => { self.previous_column(); }
                KeyCode::Right => { self.next_colum(); }
                KeyCode::End => { self.go_end(); }
                KeyCode::Home => { self.go_top(); }
                _ => {},
            }
        }

        Ok(())    
    }

    fn focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}