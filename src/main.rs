use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect, Alignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Cell, Gauge, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Clear,
    },
    DefaultTerminal, Frame,
};
use chrono::{Local, NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "todos.json";

const INFO_TEXT: [&str; 3] = [
    "ESC: quit | ↑/↓: navigate | Space: toggle complete | N: new task | E: edit | D: delete",
"S: sort by date | T: sort by target | C: sort by completion | Enter: confirm edit",
"Progress tracked automatically - overdue tasks shown in red, completed in green",
];

#[derive(Clone, PartialEq)]
enum SortMode {
    CreatedDate,
    TargetDate,
    Completion,
}

#[derive(Clone, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    title: String,
    description: String,
    target_date: NaiveDate,
    created_date: NaiveDate,
    completed: bool,
}

impl TodoItem {
    fn new(id: usize, title: String, description: String, target_date: NaiveDate) -> Self {
        Self {
            id,
            title,
            description,
            target_date,
            created_date: Local::now().date_naive(),
            completed: false,
        }
    }

    fn is_overdue(&self) -> bool {
        !self.completed && Local::now().date_naive() > self.target_date
    }

    fn get_status_color(&self) -> Color {
        if self.completed {
            Color::Green
        } else if self.is_overdue() {
            Color::Red
        } else {
            Color::White
        }
    }

    fn get_row_style(&self) -> Style {
        let color = self.get_status_color();
        if self.completed {
            Style::default().fg(color).add_modifier(Modifier::DIM)
        } else if self.is_overdue() {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        }
    }
}

#[derive(PartialEq)]
enum AppMode {
    Normal,
    AddTask,
    EditTask,
}

#[derive(Default)]
struct TaskForm {
    title: String,
    description: String,
    target_date: String,
    field_index: usize, // 0: title, 1: description, 2: date
}

impl TaskForm {
    fn clear(&mut self) {
        self.title.clear();
        self.description.clear();
        self.target_date.clear();
        self.field_index = 0;
    }

    fn current_field_mut(&mut self) -> &mut String {
        match self.field_index {
            0 => &mut self.title,
            1 => &mut self.description,
            2 => &mut self.target_date,
            _ => &mut self.title,
        }
    }

    fn next_field(&mut self) {
        self.field_index = (self.field_index + 1) % 3;
    }

    fn prev_field(&mut self) {
        self.field_index = if self.field_index == 0 { 2 } else { self.field_index - 1 };
    }
}

struct App {
    state: TableState,
    items: Vec<TodoItem>,
    scroll_state: ScrollbarState,
    mode: AppMode,
    form: TaskForm,
        sort_mode: SortMode,
        next_id: usize,
        edit_id: Option<usize>,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            state: TableState::default().with_selected(0),
            items: Vec::new(),
            scroll_state: ScrollbarState::new(0),
            mode: AppMode::Normal,
            form: TaskForm::default(),
                sort_mode: SortMode::CreatedDate,
                next_id: 1,
                edit_id: None,
        };

        // Load tasks from file
        app.load_tasks();
        app.update_scroll_state();

        // If no tasks loaded and file doesn't exist, start with empty list
        if app.items.is_empty() && app.next_id == 1 {
            app.next_id = 1;
        }

        app
    }

    fn load_tasks(&mut self) {
        if Path::new(SAVE_FILE).exists() {
            match fs::read_to_string(SAVE_FILE) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<TodoItem>>(&content) {
                        Ok(tasks) => {
                            self.items = tasks;
                            // Set next_id to be higher than any existing id
                            self.next_id = self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
                        }
                        Err(_) => {
                            // If JSON is corrupted, start fresh
                            self.items = Vec::new();
                            self.next_id = 1;
                        }
                    }
                }
                Err(_) => {
                    // If can't read file, start fresh
                    self.items = Vec::new();
                    self.next_id = 1;
                }
            }
        }
    }

    fn save_tasks(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.items) {
            let _ = fs::write(SAVE_FILE, json);
        }
    }

    fn update_scroll_state(&mut self) {
        self.scroll_state = ScrollbarState::new(self.items.len());
    }

    fn sort_items(&mut self) {
        match self.sort_mode {
            SortMode::CreatedDate => {
                self.items.sort_by(|a, b| b.created_date.cmp(&a.created_date));
            }
            SortMode::TargetDate => {
                self.items.sort_by(|a, b| a.target_date.cmp(&b.target_date));
            }
            SortMode::Completion => {
                self.items.sort_by(|a, b| a.completed.cmp(&b.completed));
            }
        }
    }

    fn next_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
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

    fn toggle_completed(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(item) = self.items.get_mut(selected) {
                item.completed = !item.completed;
                self.save_tasks(); // Save after toggling completion
            }
        }
    }

    fn delete_selected(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.items.len() {
                self.items.remove(selected);
                if self.items.is_empty() {
                    self.state.select(None);
                } else if selected >= self.items.len() {
                    self.state.select(Some(self.items.len() - 1));
                }
                self.update_scroll_state();
                self.save_tasks(); // Save after deletion
            }
        }
    }

    fn start_add_task(&mut self) {
        self.mode = AppMode::AddTask;
        self.form.clear();
        self.edit_id = None;
    }

    fn start_edit_task(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(item) = self.items.get(selected) {
                self.mode = AppMode::EditTask;
                self.edit_id = Some(item.id);
                self.form.title = item.title.clone();
                self.form.description = item.description.clone();
                self.form.target_date = item.target_date.format("%Y-%m-%d").to_string();
                self.form.field_index = 0;
            }
        }
    }

    fn submit_form(&mut self) {
        if let Ok(target_date) = NaiveDate::parse_from_str(&self.form.target_date, "%Y-%m-%d") {
            match self.mode {
                AppMode::AddTask => {
                    let item = TodoItem::new(
                        self.next_id,
                        self.form.title.clone(),
                                             self.form.description.clone(),
                                             target_date,
                    );
                    self.items.push(item);
                    self.next_id += 1;
                    self.update_scroll_state();
                    self.save_tasks(); // Save after adding
                }
                AppMode::EditTask => {
                    if let Some(edit_id) = self.edit_id {
                        if let Some(item) = self.items.iter_mut().find(|i| i.id == edit_id) {
                            item.title = self.form.title.clone();
                            item.description = self.form.description.clone();
                            item.target_date = target_date;
                            self.save_tasks(); // Save after editing
                        }
                    }
                }
                _ => {}
            }
            self.sort_items();
        }
        self.mode = AppMode::Normal;
    }

    fn cancel_form(&mut self) {
        self.mode = AppMode::Normal;
        self.form.clear();
        self.edit_id = None;
    }

    fn get_progress(&self) -> (usize, usize) {
        let completed = self.items.iter().filter(|item| item.completed).count();
        let total = self.items.len();
        (completed, total)
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.mode {
                        AppMode::Normal => {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                KeyCode::Down => self.next_row(),
                                KeyCode::Up => self.previous_row(),
                                KeyCode::Char(' ') => self.toggle_completed(),
                                KeyCode::Char('n') | KeyCode::Char('N') => self.start_add_task(),
                                KeyCode::Char('e') | KeyCode::Char('E') => self.start_edit_task(),
                                KeyCode::Char('d') | KeyCode::Char('D') => self.delete_selected(),
                                KeyCode::Char('s') | KeyCode::Char('S') => {
                                    self.sort_mode = SortMode::CreatedDate;
                                    self.sort_items();
                                }
                                KeyCode::Char('t') | KeyCode::Char('T') => {
                                    self.sort_mode = SortMode::TargetDate;
                                    self.sort_items();
                                }
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    self.sort_mode = SortMode::Completion;
                                    self.sort_items();
                                }
                                _ => {}
                            }
                        }
                        AppMode::AddTask | AppMode::EditTask => {
                            match key.code {
                                KeyCode::Esc => self.cancel_form(),
                                KeyCode::Enter => self.submit_form(),
                                KeyCode::Tab => self.form.next_field(),
                                KeyCode::BackTab => self.form.prev_field(),
                                KeyCode::Char(c) => {
                                    self.form.current_field_mut().push(c);
                                }
                                KeyCode::Backspace => {
                                    self.form.current_field_mut().pop();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let main_layout = Layout::vertical([
            Constraint::Length(3), // Progress bar
                                           Constraint::Min(5),    // Table
                                           Constraint::Length(5), // Footer
        ]);
        let chunks = main_layout.split(frame.area());

        self.render_progress_bar(frame, chunks[0]);
        self.render_table(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);

        if self.mode == AppMode::AddTask || self.mode == AppMode::EditTask {
            self.render_form_popup(frame);
        }
    }

    fn render_progress_bar(&self, frame: &mut Frame, area: Rect) {
        let (completed, total) = self.get_progress();
        let progress = if total > 0 { completed as f64 / total as f64 } else { 0.0 };

        let progress_text = format!("Progress: {}/{} tasks completed", completed, total);
        let gauge = Gauge::default()
        .block(Block::bordered().title("Todo Progress"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .percent((progress * 100.0) as u16)
        .label(progress_text);

        frame.render_widget(gauge, area);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header = ["Title", "Description", "Target Date", "Status"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .height(1);

        let rows = self.items.iter().map(|item| {
            let status = if item.completed { "✓ Done" } else { "○ Pending" };
            let status_color = item.get_status_color();

            Row::new(vec![
                Cell::from(item.title.clone()),
                     Cell::from(item.description.clone()),
                     Cell::from(item.target_date.format("%Y-%m-%d").to_string()),
                     Cell::from(status).style(Style::default().fg(status_color)),
            ])
            .style(item.get_row_style())
            .height(1)
        });

        let sort_indicator = match self.sort_mode {
            SortMode::CreatedDate => " [Sorted by Date]",
            SortMode::TargetDate => " [Sorted by Target]",
            SortMode::Completion => " [Sorted by Status]",
        };

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(30),
                               Constraint::Percentage(40),
                               Constraint::Percentage(15),
                               Constraint::Percentage(15),
            ],
        )
        .header(header)
        .block(Block::bordered().title(format!("Todo List{}", sort_indicator)))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(table, area, &mut self.state);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_text = Text::from_iter(INFO_TEXT.iter().map(|&s| Line::from(s)));
        let footer = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::bordered().title("Controls"))
        .alignment(Alignment::Center);

        frame.render_widget(footer, area);
    }

    fn render_form_popup(&self, frame: &mut Frame) {
        let area = frame.area();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        frame.render_widget(Clear, popup_area);

        let title = if self.mode == AppMode::AddTask {
            "Add New Task"
        } else {
            "Edit Task"
        };

        let form_layout = Layout::vertical([
            Constraint::Length(3),
                                           Constraint::Length(3),
                                           Constraint::Length(3),
                                           Constraint::Min(2),
        ]);
        let form_chunks = form_layout.split(popup_area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }));

        let popup_block = Block::bordered()
        .title(title)
        .style(Style::default().bg(Color::Black));
        frame.render_widget(popup_block, popup_area);

        // Title field
        let title_style = if self.form.field_index == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let title_input = Paragraph::new(self.form.title.as_str())
        .block(Block::bordered().title("Title").style(title_style))
        .style(title_style);
        frame.render_widget(title_input, form_chunks[0]);

        // Description field
        let desc_style = if self.form.field_index == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let desc_input = Paragraph::new(self.form.description.as_str())
        .block(Block::bordered().title("Description").style(desc_style))
        .style(desc_style);
        frame.render_widget(desc_input, form_chunks[1]);

        // Target date field
        let date_style = if self.form.field_index == 2 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let date_input = Paragraph::new(self.form.target_date.as_str())
        .block(Block::bordered().title("Target Date (YYYY-MM-DD)").style(date_style))
        .style(date_style);
        frame.render_widget(date_input, form_chunks[2]);

        // Instructions
        let instructions = Paragraph::new("Tab/Shift+Tab: Navigate | Enter: Save | Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
        frame.render_widget(instructions, form_chunks[3]);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

