use crate::domain::pomodoro_timer::*;
use std::sync::mpsc;
use std::thread;
use notify_rust::Notification;

use crossterm::{
    event::{self, Event as CEvent, KeyCode, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Tabs},
    Frame, Terminal,
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

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn notification() {
    Notification::new()
        .summary("Pomodoro timer")
        .body("Time is up")
        .show().unwrap();
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    let mut app = PomodoroTimerImpl::new(notification);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let menu_titles = vec!["Session", "Stats"];
    let mut active_menu_item = MenuItem::Session;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let footer = Paragraph::new(
                "(p) pomodoro (s) short (l) long <SPACE> pause (<- ->) navigate"
            )
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Usage")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    Spans::from(vec![Span::styled(*t, Style::default().fg(Color::White))])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Jabuti Pomodoro").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Session => render_session(rect, &mut app, chunks[1]),
                MenuItem::Stats => render_stats(rect, &app, chunks[1]), 
            }
            rect.render_widget(footer, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('p') => app.start_pomodoro(),
                KeyCode::Char('s') => app.start_short_interval(),
                KeyCode::Char('l') => app.start_long_interval(),
                KeyCode::Char(' ') => app.pause_toggle(),
                KeyCode::Left => active_menu_item = MenuItem::Session,
                KeyCode::Right => active_menu_item = MenuItem::Stats,
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
fn render_session<B>(f: &mut Frame<B>, app: &mut impl PomodoroTimer, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(2),
                Constraint::Length(1),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .vertical_margin(6)
        .horizontal_margin(10)
        .split(area);
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let minutes = app.load_remaining_time() / 60;
    let seconds = app.load_remaining_time() % 60;

    let text = vec![
        Spans::from("Time Remaining"),
        Spans::from(""),
        Spans::from(format!("{:}:{:02}", minutes, seconds)),
    ];

    let remaining = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default());

    f.render_widget(remaining, chunks[0]);

    let label = format!("{:}%", app.load_progress());
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .percent(app.load_progress() as u16);
    f.render_widget(gauge, chunks[1]);
}

fn render_stats<B>(f: &mut Frame<B>, _app: &impl PomodoroTimer, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(2),
                Constraint::Length(1),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .vertical_margin(6)
        .horizontal_margin(10)
        .split(area);
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let wip = Paragraph::new("Work In Progress")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default());

    f.render_widget(wip, chunks[1]);
}