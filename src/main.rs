use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use todo_cli::tasks::adapters::persistence::json_file_task_repository::JsonFileTaskRepository;
use todo_cli::tasks::adapters::tui::app::App;
use todo_cli::tasks::adapters::tui::event::handle_events;
use todo_cli::tasks::adapters::tui::ui::draw;
use todo_cli::tasks::ports::outputs::task_repository::TaskRepository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo: JsonFileTaskRepository = JsonFileTaskRepository::new()?;
    run(repo)
}

fn run<R: TaskRepository + Clone>(repo: R) -> Result<(), Box<dyn std::error::Error>> {
    let mut app: App<R> = App::new(repo)?;
    let mut terminal: DefaultTerminal = ratatui::init();
    let mut table_state: TableState = TableState::default();
    let result = loop_app(&mut terminal, &mut app, &mut table_state);
    ratatui::restore(); // SIEMPRE se ejecuta, haya error o no
    result
}
fn loop_app<R: TaskRepository + Clone>(
    terminal: &mut DefaultTerminal,
    app: &mut App<R>,
    table_state: &mut TableState,
) -> Result<(), Box<dyn std::error::Error>> {
    while !app.should_quit {
        table_state.select(Some(app.selected));
        terminal.draw(|frame| draw(frame, app, table_state))?;
        handle_events(app)?;
    }
    Ok(())
}
