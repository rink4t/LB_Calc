use ratatui::widgets::{Table, TableState};

//|-----------------{EntryLine( . .)φ}------------------|
pub struct TableComp {
    table_state: TableState, 
}

impl TableComp {
    pub fn new() -> TableComp {
        TableComp { table_state: TableState::default() }
    }
}