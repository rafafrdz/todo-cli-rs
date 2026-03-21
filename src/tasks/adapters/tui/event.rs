use crate::tasks::adapters::tui::app::{App, InputMode};
use crate::tasks::adapters::tui::errors::TuiResult;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn handle_events<R: TaskRepository + Clone>(app: &mut App<R>) -> TuiResult<()> {
    if let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        match app.input_mode {
            InputMode::Normal => handle_normal_mode(app, key)?,
            InputMode::Adding => handle_adding_mode(app, key)?,
            InputMode::Editing => handle_editing_mode(app, key)?,
            InputMode::ConfirmDelete => handle_confirm_delete_mode(app, key)?,
        }
    }
    Ok(())
}

fn handle_normal_mode<R: TaskRepository + Clone>(app: &mut App<R>, key: KeyEvent) -> TuiResult<()> {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('a') => {
            app.clear_status();
            app.input_mode = InputMode::Adding;
        }
        KeyCode::Char('e') => {
            if !app.tasks.is_empty() {
                app.clear_status();
                app.start_editing();
            }
        }
        KeyCode::Char('d') => {
            if !app.tasks.is_empty() {
                app.clear_status();
                app.input_mode = InputMode::ConfirmDelete;
            }
        }
        KeyCode::Char('x') => {
            if !app.tasks.is_empty() {
                app.cycle_todo_done()?;
            }
        }
        KeyCode::Char('f') => app.cycle_filter()?,
        KeyCode::Char('j') | KeyCode::Down => {
            app.clear_status();
            app.select_next()?;
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.clear_status();
            app.select_previous()?;
        }
        _ => (),
    };
    Ok(())
}

fn handle_adding_mode<R: TaskRepository + Clone>(app: &mut App<R>, key: KeyEvent) -> TuiResult<()> {
    match key.code {
        KeyCode::Enter => app.add_task()?,
        KeyCode::Esc => {
            app.input_buffer = String::new();
            app.input_mode = InputMode::Normal
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        _ => (),
    };
    Ok(())
}

fn handle_editing_mode<R: TaskRepository + Clone>(
    app: &mut App<R>,
    key: KeyEvent,
) -> TuiResult<()> {
    match key.code {
        KeyCode::Enter => app.edit_task()?,
        KeyCode::Esc => {
            app.input_buffer = String::new();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        _ => (),
    };
    Ok(())
}

fn handle_confirm_delete_mode<R: TaskRepository + Clone>(
    app: &mut App<R>,
    key: KeyEvent,
) -> TuiResult<()> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => app.delete_task()?,
        KeyCode::Char('n') | KeyCode::Esc => app.input_mode = InputMode::Normal,
        _ => (),
    };
    Ok(())
}
