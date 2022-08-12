use crate::domain::pomodoro::*;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Session,
    Stats,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Session => 0,
            MenuItem::Stats => 1,
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &impl Pomodoro) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tabTitles = vec!["Session", "Stats"];
    let titles = tabTitles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Pomodoro Timer"),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(0);
    f.render_widget(tabs, chunks[0]);

    draw_first_tab(f, app, chunks[1]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &impl Pomodoro, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(15),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &impl Pomodoro, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Graphs");
    f.render_widget(block, area);

    //TODO Progress
    let label = format!("{:}%", (app.progress() * 100.0) as i16);
    let gauge = Gauge::default()
        .block(Block::default().title("Gauge:"))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(app.progress());
    f.render_widget(gauge, chunks[0]);

    /*
    let sparkline = Sparkline::default()
        .block(Block::default().title("Sparkline:"))
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline.points)
        .bar_set(if app.enhanced_graphics {
            symbols::bar::NINE_LEVELS
        } else {
            symbols::bar::THREE_LEVELS
        });
    f.render_widget(sparkline, chunks[1]);
    */

    let line_gauge = LineGauge::default()
        .block(Block::default().title("LineGauge:"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .line_set(symbols::line::THICK)
        .ratio(app.progress());
    f.render_widget(line_gauge, chunks[2]);
}
