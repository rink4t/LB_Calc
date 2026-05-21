use engine::properties::{Properties};
use ratatui::{Frame, layout::{Alignment, Rect}, text::Text, widgets::{Block, Borders, Padding, Paragraph}};

use crate::components::DrawableComp;

//|-----------------{Flags ( . .)φ}------------------|

#[derive(PartialEq, Eq)]
enum Flag {
    PrntProps,
    NPrntProps,
}

//|-----------------{Properties Component ( . .)φ}------------------|
pub struct PropsComp{
    props: Properties,
    flag: Flag,
}

impl PropsComp {
    pub fn update(&mut self, props: Properties) {
        self.props = props;
        self.flag = Flag::PrntProps;
    }
}

impl Default for PropsComp {
    fn default() -> Self {
        Self { props: engine::properties::Properties::default(), flag: Flag::NPrntProps }
    }
}

impl DrawableComp for PropsComp {
    fn draw(&self, f: &mut Frame, rect: Rect) {
        if self.flag == Flag::PrntProps {
            let props = Paragraph::new(Text::from(    
                vec![
                    format!("Tautology: {}", self.props.tautology).into(),
                    format!("Contradiction: {}", self.props.contradiction).into(),
                    format!("Satisfactory: {}", self.props.satisfactory).into(),
                    format!("Contingent: {}", self.props.contingent).into(),
                    format!("Equivalent: {}", self.props.equivalent).into(),
                ]
            )).block(Block::new().borders(Borders::ALL));
            
            f.render_widget(props, rect);
        }else {
            let text = "__φ(。。) Props?";
            let text_lines = text.lines().count() as u16;
            let block_height = rect.height.saturating_sub(2);
            let ver_padding = block_height.saturating_sub(text_lines) / 2;

            let text = Paragraph::new(text).block(Block::new().borders(Borders::ALL).padding(Padding::vertical(ver_padding))).alignment(Alignment::Center);
            f.render_widget(text, rect);
        }

    }
}