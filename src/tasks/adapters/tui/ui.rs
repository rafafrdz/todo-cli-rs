use crate::tasks::adapters::tui::app::{App, InputMode};
use crate::tasks::domain::task::TaskStatus;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Row, Table, TableState};
use ratatui::Frame;

pub fn draw<R: TaskRepository>(frame: &mut Frame, app: &App<R>, table_state: &mut TableState) {
    let layout = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(frame.area());
    let [main, command, status] = [layout[0], layout[1], layout[2]];

    render_table(frame, main, app, table_state);
    render_command(frame, command, app);
    render_status(frame, status, app);
    if app.input_mode == InputMode::Adding || app.input_mode == InputMode::Editing {
        render_input_popup(frame, app);
    }
}

fn render_table<R: TaskRepository>(
    frame: &mut Frame,
    area: Rect,
    app: &App<R>,
    table_state: &mut TableState,
) {
    let border_title = Block::bordered()
        .title(" TODO Tasks ")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    if app.tasks.is_empty() {
        let empty_msg = Paragraph::new("No tasks yet. Press [a] to add one.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .block(border_title);
        frame.render_widget(empty_msg, area);
        return;
    }

    let header = Row::new(["ID", "STATUS", "TITLE", "CREATED", "MODIFIED"])
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .tasks
        .iter()
        .map(|task| {
            let status_style = match task.status() {
                TaskStatus::Todo => Style::default().fg(Color::Yellow),
                TaskStatus::Done => Style::default().fg(Color::Green),
            };
            let status_label = match task.status() {
                TaskStatus::Todo => "[ ] TODO",
                TaskStatus::Done => "[x] DONE",
            };

            // Truncate UUID to first 8 chars
            let short_id = &format!("{}", task.task_id())[..8];

            // Format timestamps as "Mar 21 14:30"
            let created = task.created_at().format("%b %d %H:%M").to_string();
            let modified = task.modified_at().format("%b %d %H:%M").to_string();

            Row::new([
                Line::from(Span::styled(
                    short_id.to_string(),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(status_label, status_style)),
                Line::from(Span::raw(task.title())),
                Line::from(Span::styled(created, Style::default().fg(Color::DarkGray))),
                Line::from(Span::styled(modified, Style::default().fg(Color::DarkGray))),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),  // ID (truncated)
        Constraint::Length(10), // STATUS
        Constraint::Fill(1),    // TITLE (takes remaining space)
        Constraint::Length(12), // CREATED
        Constraint::Length(12), // MODIFIED
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(border_title)
        .column_spacing(2)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(table, area, table_state);
}

fn render_command<R: TaskRepository>(frame: &mut Frame, area: Rect, app: &App<R>) {
    let line = match app.input_mode {
        InputMode::Normal => Line::from(vec![
            Span::styled(" [a]", Style::default().fg(Color::Cyan).bold()),
            Span::raw("dd "),
            Span::styled("[e]", Style::default().fg(Color::Cyan).bold()),
            Span::raw("dit "),
            Span::styled("[d]", Style::default().fg(Color::Red).bold()),
            Span::raw("el "),
            Span::styled("[x]", Style::default().fg(Color::Yellow).bold()),
            Span::raw("done/todo "),
            Span::styled("[f]", Style::default().fg(Color::Magenta).bold()),
            Span::raw("ilter "),
            Span::styled("[q]", Style::default().fg(Color::Red).bold()),
            Span::raw("uit"),
        ]),
        InputMode::ConfirmDelete => Line::from(vec![
            Span::styled(
                " Delete this task? ",
                Style::default().fg(Color::Red).bold(),
            ),
            Span::styled("[y]", Style::default().fg(Color::Green).bold()),
            Span::raw("es "),
            Span::styled("[n]", Style::default().fg(Color::Red).bold()),
            Span::raw("o"),
        ]),
        _ => Line::from(vec![]),
    };
    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Black));
    frame.render_widget(paragraph, area);
}

fn render_status<R: TaskRepository>(frame: &mut Frame, area: Rect, app: &App<R>) {
    let filter_label = match app.filter {
        crate::tasks::application::use_cases::list_tasks::FilterTask::All => "All",
        crate::tasks::application::use_cases::list_tasks::FilterTask::Todo => "Todo",
        crate::tasks::application::use_cases::list_tasks::FilterTask::Done => "Done",
    };

    let mut spans = vec![
        Span::styled(" Filter: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            filter_label,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" | {} tasks", app.tasks.len()),
            Style::default().fg(Color::DarkGray),
        ),
    ];

    if let Some(ref msg) = app.status_message {
        spans.push(Span::styled(
            format!("  {msg}"),
            Style::default().fg(Color::Yellow).bold(),
        ));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Black));
    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);
    let horizontal = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1]);
    horizontal[1]
}
fn render_input_popup<R: TaskRepository>(frame: &mut Frame, app: &App<R>) {
    let area = centered_rect(50, 20, frame.area());
    frame.render_widget(Clear, area);
    let (title, label_style) = match app.input_mode {
        InputMode::Adding => (" New Task ", Style::default().fg(Color::Cyan)),
        InputMode::Editing => (" Edit Task ", Style::default().fg(Color::Yellow)),
        _ => unreachable!(),
    };
    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(label_style);
    let input_line = Line::from(vec![
        Span::styled(app.input_buffer.as_str(), Style::default().fg(Color::White)),
        Span::styled("█", label_style),
    ]);
    let help_line = Line::from(vec![
        Span::styled("[Enter]", Style::default().fg(Color::Green).bold()),
        Span::raw(" confirm  "),
        Span::styled("[Esc]", Style::default().fg(Color::Red).bold()),
        Span::raw(" cancel"),
    ]);
    let content = Paragraph::new(vec![Line::raw(""), input_line, Line::raw(""), help_line])
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(content, area);
}
