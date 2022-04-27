// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod components;

pub mod user_input {
    /// A simple example demonstrating how to handle user input. This is
    /// a bit out of the scope of the library as it does not provide any
    /// input handling out of the box. However, it may helps some to get
    /// started.
    ///
    /// This is a very simple example:
    ///   * A input box always focused. Every character you type is registered
    ///   here
    ///   * Pressing Backspace erases a character
    ///   * Pressing Enter pushes the current input in the history of previous
    ///   messages
    use copypasta::{ClipboardContext, ClipboardProvider};
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::{
        error::Error,
        io,
        ops::{Range, RangeInclusive},
    };
    use tui::{
        backend::{Backend, CrosstermBackend},
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans, Text},
        widgets::{Block, Borders, List, ListItem, Paragraph},
        Frame, Terminal,
    };
    use unicode_width::UnicodeWidthStr;

    use super::components::input::InputState;

    enum InputMode {
        Normal,
        Editing,
    }

    /// App holds the state of the application
    struct App {
        /// Current value of the input box
        input: InputState,
        /// Current input mode
        input_mode: InputMode,
        /// History of recorded messages
        messages: Vec<String>,
    }

    impl Default for App {
        fn default() -> App {
            App {
                input: InputState::new(),
                input_mode: InputMode::Normal,
                messages: Vec::new(),
            }
        }
    }

    pub fn execute() -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let app = App::default();
        let res = run_app(&mut terminal, app);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
            return Err(err);
        }

        Ok(())
    }

    fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
        let mut clipboard = match ClipboardContext::new() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        loop {
            terminal.draw(|f| ui(f, &app))?;

            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match (key.code, key.modifiers) {
                        (KeyCode::Enter, _) => {
                            app.messages.push(app.input.enter_current());
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                            app.input.insert_char(c);
                        }
                        (KeyCode::Char(c), KeyModifiers::CONTROL) => {
                            match c {
                                'a' => {
                                    app.input.move_pos_home();
                                }
                                'd' => {
                                    app.input.move_pos_end();
                                }
                                'c' => {
                                    if let Some(text) = app.input.get_selected_text() {
                                        let _res = clipboard.set_contents(text);
                                    }
                                }
                                'v' => {
                                    if let Ok(text) = clipboard.get_contents() {
                                        app.input.push_str(&text);
                                    }
                                }
                                'x' => {
                                    if let Some(selected) = app.input.remove_selected() {
                                        let _res = clipboard.set_contents(selected);
                                    }
                                }
                                _ => {}
                            };
                        }
                        (KeyCode::Backspace, _) => {
                            app.input.backspace_char();
                        }
                        (KeyCode::Delete, _) => {
                            app.input.delete_char();
                        }
                        (KeyCode::Esc, _) => {
                            app.input_mode = InputMode::Normal;
                        }
                        (KeyCode::Left, _) => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.input.move_pos_home();
                            } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.input.grow_selection_left();
                            } else {
                                app.input.move_pos_left();
                            }
                        }
                        (KeyCode::Right, _) => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.input.move_pos_end();
                            } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.input.grow_selection_right();
                            } else {
                                app.input.move_pos_right();
                            }
                        }
                        (KeyCode::Home, _) => {
                            app.input.move_pos_home();
                        }
                        (KeyCode::End, _) => {
                            app.input.move_pos_end();
                        }
                        (KeyCode::Up, _) => {
                            app.input.search_history();
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn ui<B: Backend>(f: &mut Frame<'_, B>, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let (msg, style) = match app.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);

        let input = Paragraph::new(app.input.get_text())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[1]);
        match app.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + app.input.get_pos() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                );
            }
        }

        let messages: Vec<ListItem<'_>> = app
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let messages =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(messages, chunks[2]);
    }
}

pub mod list {
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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
        layout::{Constraint, Corner, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, Borders, List, ListItem, ListState},
        Frame, Terminal,
    };

    struct StatefulList<T> {
        state: ListState,
        items: Vec<T>,
    }

    impl<T> StatefulList<T> {
        fn with_items(items: Vec<T>) -> StatefulList<T> {
            StatefulList {
                state: ListState::default(),
                items,
            }
        }

        fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }

        fn previous(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }

        fn unselect(&mut self) {
            self.state.select(None);
        }
    }

    /// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
    /// around `ListState`. Keeping track of the items state let us render the associated widget with its state
    /// and have access to features such as natural scrolling.
    ///
    /// Check the event handling at the bottom to see how to change the state on incoming events.
    /// Check the drawing logic for items on how to specify the highlighting style for selected items.
    struct App<'a> {
        items: StatefulList<(&'a str, usize)>,
        events: Vec<(&'a str, &'a str)>,
    }

    impl<'a> App<'a> {
        fn new() -> App<'a> {
            App {
                items: StatefulList::with_items(vec![
                    ("Item0", 1),
                    ("Item1", 2),
                    ("Item2", 1),
                    ("Item3", 3),
                    ("Item4", 1),
                    ("Item5", 4),
                    ("Item6", 1),
                    ("Item7", 3),
                    ("Item8", 1),
                    ("Item9", 6),
                    ("Item10", 1),
                    ("Item11", 3),
                    ("Item12", 1),
                    ("Item13", 2),
                    ("Item14", 1),
                    ("Item15", 1),
                    ("Item16", 4),
                    ("Item17", 1),
                    ("Item18", 5),
                    ("Item19", 4),
                    ("Item20", 1),
                    ("Item21", 2),
                    ("Item22", 1),
                    ("Item23", 3),
                    ("Item24", 1),
                ]),
                events: vec![
                    ("Event1", "INFO"),
                    ("Event2", "INFO"),
                    ("Event3", "CRITICAL"),
                    ("Event4", "ERROR"),
                    ("Event5", "INFO"),
                    ("Event6", "INFO"),
                    ("Event7", "WARNING"),
                    ("Event8", "INFO"),
                    ("Event9", "INFO"),
                    ("Event10", "INFO"),
                    ("Event11", "CRITICAL"),
                    ("Event12", "INFO"),
                    ("Event13", "INFO"),
                    ("Event14", "INFO"),
                    ("Event15", "INFO"),
                    ("Event16", "INFO"),
                    ("Event17", "ERROR"),
                    ("Event18", "ERROR"),
                    ("Event19", "INFO"),
                    ("Event20", "INFO"),
                    ("Event21", "WARNING"),
                    ("Event22", "INFO"),
                    ("Event23", "INFO"),
                    ("Event24", "WARNING"),
                    ("Event25", "INFO"),
                    ("Event26", "INFO"),
                ],
            }
        }

        /// Rotate through the event list.
        /// This only exists to simulate some kind of "progress"
        fn on_tick(&mut self) {
            let event = self.events.remove(0);
            self.events.push(event);
        }
    }

    pub fn execute() -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new();
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
            return Err(Box::new(err));
        }

        Ok(())
    }

    fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        mut app: App<'_>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui(f, &mut app))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.items.unselect(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Up => app.items.previous(),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        // Iterate through all elements in the `items` app and append some debug text to it.
        let items: Vec<ListItem<'_>> = app
            .items
            .items
            .iter()
            .map(|i| {
                let mut lines = vec![Spans::from(i.0)];
                for _ in 0..i.1 {
                    lines.push(Spans::from(Span::styled(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                }
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, chunks[0], &mut app.items.state);

        // Let's do the same for the events.
        // The event list doesn't have any state and only displays the current state of the list.
        let events: Vec<ListItem<'_>> = app
            .events
            .iter()
            .rev()
            .map(|&(event, level)| {
                // Colorcode the level depending on its type
                let s = match level {
                    "CRITICAL" => Style::default().fg(Color::Red),
                    "ERROR" => Style::default().fg(Color::Magenta),
                    "WARNING" => Style::default().fg(Color::Yellow),
                    "INFO" => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                };
                // Add a example datetime and apply proper spacing between them
                let header = Spans::from(vec![
                    Span::styled(format!("{:<9}", level), s),
                    Span::raw(" "),
                    Span::styled(
                        "2020-01-01 10:00:00",
                        Style::default().add_modifier(Modifier::ITALIC),
                    ),
                ]);
                // The event gets its own line
                let log = Spans::from(vec![Span::raw(event)]);

                // Here several things happen:
                // 1. Add a `---` spacing line above the final list entry
                // 2. Add the Level + datetime
                // 3. Add a spacer line
                // 4. Add the actual event
                ListItem::new(vec![
                    Spans::from("-".repeat(chunks[1].width as usize)),
                    header,
                    Spans::from(""),
                    log,
                ])
            })
            .collect();
        let events_list = List::new(events)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .start_corner(Corner::BottomLeft);
        f.render_widget(events_list, chunks[1]);
    }
}

pub mod canvas {
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Style},
        text::Span,
        widgets::{
            canvas::{Canvas, Map, MapResolution, Rectangle},
            Block, Borders,
        },
        Frame, Terminal,
    };

    struct App {
        x: f64,
        y: f64,
        ball: Rectangle,
        playground: Rect,
        vx: f64,
        vy: f64,
        dir_x: bool,
        dir_y: bool,
    }

    impl App {
        fn new() -> App {
            App {
                x: 0.0,
                y: 0.0,
                ball: Rectangle {
                    x: 10.0,
                    y: 30.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Yellow,
                },
                playground: Rect::new(10, 10, 100, 100),
                vx: 1.0,
                vy: 1.0,
                dir_x: true,
                dir_y: true,
            }
        }

        #[allow(clippy::cast_lossless)]
        fn on_tick(&mut self) {
            if self.ball.x < self.playground.left() as f64
                || self.ball.x + self.ball.width > self.playground.right() as f64
            {
                self.dir_x = !self.dir_x;
            }
            if self.ball.y < self.playground.top() as f64
                || self.ball.y + self.ball.height > self.playground.bottom() as f64
            {
                self.dir_y = !self.dir_y;
            }

            if self.dir_x {
                self.ball.x += self.vx;
            } else {
                self.ball.x -= self.vx;
            }

            if self.dir_y {
                self.ball.y += self.vy;
            } else {
                self.ball.y -= self.vy;
            }
        }
    }

    pub fn execute() -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new();
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
            return Err(box err);
        }

        Ok(())
    }

    fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        mut app: App,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui(f, &app))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Down => {
                            app.y += 1.0;
                        }
                        KeyCode::Up => {
                            app.y -= 1.0;
                        }
                        KeyCode::Right => {
                            app.x += 1.0;
                        }
                        KeyCode::Left => {
                            app.x -= 1.0;
                        }
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<'_, B>, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("World"))
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::White,
                    resolution: MapResolution::High,
                });
                ctx.print(
                    app.x,
                    -app.y,
                    Span::styled("You are here", Style::default().fg(Color::Yellow)),
                );
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);
        f.render_widget(canvas, chunks[0]);
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Pong"))
            .paint(|ctx| {
                ctx.draw(&app.ball);
            })
            .x_bounds([10.0, 110.0])
            .y_bounds([10.0, 110.0]);
        f.render_widget(canvas, chunks[1]);
    }
}
