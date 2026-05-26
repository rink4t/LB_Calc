use crossterm::style::Stylize;
use engine::properties::{Properties};
use ratatui::{Frame, layout::{Alignment, Rect}, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Padding, Paragraph}};

use crate::components::DrawableComp;

//|-----------------{Flags >ᴗ<}------------------|

#[derive(PartialEq, Eq)]
enum Flag {
    ShowProps,
    ShowMsg,
}

//|-----------------{Properties Component ( . .)φ}------------------|
pub struct PropsComp{
    props: Properties,
    flag: Flag,
}

impl PropsComp {
    pub fn update(&mut self, props: Properties) {
        self.props = props;
        self.flag = Flag::ShowProps;
    }

    pub fn no_props(&mut self) {
        self.flag = Flag::ShowMsg;
    }
}

impl Default for PropsComp {
    fn default() -> Self {
        Self { props: engine::properties::Properties::default(), flag: Flag::ShowProps }
    }
}

impl DrawableComp for PropsComp {
    fn draw(&self, f: &mut Frame, rect: Rect) {
        if self.flag == Flag::ShowProps {
            let props = Paragraph::new(Text::from(    
                vec![
                    Line::from(" "),
                    Line::from(vec![Span::raw("Tautology: "), Span::styled(self.props.tautology.to_string(), Style::default()
                    .fg( if self.props.tautology {Color::Green}else{Color::Red}))]),
                    Line::from(" "),
                    Line::from(vec![Span::raw("Contradiction: "), Span::styled(self.props.contradiction.to_string(), Style::default()
                    .fg( if self.props.contradiction {Color::Green}else{Color::Red}))]),
                    Line::from(" "),
                    Line::from(vec![Span::raw("Satisfactory: "), Span::styled(self.props.satisfactory.to_string(), Style::default()
                    .fg( if self.props.satisfactory {Color::Green}else{Color::Red}))]),
                    Line::from(" "),
                    Line::from(vec![Span::raw("contingent: "), Span::styled(self.props.contingent.to_string(), Style::default()
                    .fg( if self.props.contingent {Color::Green}else{Color::Red}))]),
                    Line::from(" "),
                    Line::from(vec![Span::raw("Equivalent: "), Span::styled(self.props.equivalent.to_string(), Style::default()
                    .fg( if self.props.equivalent {Color::Green}else{Color::Red}))]),
                ]
            )).block(Block::new().borders(Borders::ALL));
            
            f.render_widget(props, rect);
        }else if self.flag == Flag::ShowMsg {
            let text = "__φ(。。) Props?";
            let text_lines = text.lines().count() as u16;
            let block_height = rect.height.saturating_sub(2);
            let ver_padding = block_height.saturating_sub(text_lines) / 2;

            let text = Paragraph::new(text).block(Block::new().borders(Borders::ALL).padding(Padding::vertical(ver_padding))).alignment(Alignment::Center);
            f.render_widget(text, rect);
        }

    }
}