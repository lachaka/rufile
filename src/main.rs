use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, error, fs, io};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::backend::TermionBackend;
use tui::layout::{
    Constraint,
    Direction,
    Layout
};
use tui::style::{
    Color,
    Modifier,
    Style
};
use tui::text::{
    Span,
    Spans
};
use tui::Terminal;
use tui::widgets::{
    Block,
    BorderType,
    Borders, List,
    ListItem,
    ListState,
    Paragraph
};

mod event;
mod entry;

use event::{Event, Events};
use entry::file_data::FileData;

fn main() -> Result<(), Box<dyn error::Error>> {
    let events = Events::new();

    let cwd = env::current_dir().unwrap();
    let mut path = cwd;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut file_list_state = ListState::default();
    file_list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .horizontal_margin(1)
                .direction(Direction::Vertical)
                .constraints([
                        Constraint::Length(3),
                        Constraint::Min(3)
                    ].as_ref()
                )
                .split(f.size());
            
            let search = Paragraph::new("Search for a file")
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green))
                        .title(Span::styled(" Search ", 
                        Style::default().fg(Color::LightGreen)))
                        .border_type(BorderType::Thick),
            );
            f.render_widget(search, chunks[0]);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref()
                )
                .split(chunks[1]);
            
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(12),
                        Constraint::Length(6),
                        Constraint::Length(2)
                    ].as_ref()
                )
                .split(main_chunks[1]);

            let (list, mut paragraphs) = render_files(&file_list_state, &path);
        
            f.render_stateful_widget(list, main_chunks[0], &mut file_list_state);
            f.render_widget(paragraphs.remove(1), right[1]);
            f.render_widget(paragraphs.remove(0), right[0]);
        })?;

        match events.rx.recv()? {
            Event::Input(input) => match input {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Up => {
                    if let Some(selected) = file_list_state.selected() {
                        let files_count = read_dir(&path).unwrap().len(); 
                        if selected > 0 {
                            file_list_state.select(Some(selected - 1));
                        } else {
                            file_list_state.select(Some(files_count - 1));
                        }
                    }
                }
                Key::Down => {
                    if let Some(selected) = file_list_state.selected() {
                        let files_count = read_dir(&path).unwrap().len(); 
                        if selected >= files_count - 1 {
                            file_list_state.select(Some(0));
                        } else {
                            file_list_state.select(Some(selected + 1));
                        }
                    }
                }
                Key::Right => {
                    open_file(&mut path, &mut file_list_state);
                }
                Key::Left => {
                    path.pop();
                    env::set_current_dir(&path).expect("invalid path");
                    file_list_state.select(Some(0));
                }
                _ => {}
            },
            Event::Tick => {},
        }
    }

    Ok(())
}

fn read_dir(path: &PathBuf) -> Result<Vec<FileData>, io::Error> {
    let mut files: Vec<FileData> = Vec::<FileData>::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        
        if let Ok(entry_data) = FileData::new(entry) {
            files.push(entry_data);
        }
    }

    Ok(files)
}

fn render_files<'a>(file_list_state: &ListState, path: &PathBuf) 
        -> (List<'a>, Vec<Paragraph<'a>>) {
    
    let title = format!(" {} ", path.to_str().unwrap());
    let files = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(title)
        .border_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Thick);

    let file_list = read_dir(&path).expect("cannot list files");

    let items: Vec<_> = file_list
        .iter()
        .map(|file| {
            ListItem::new(Spans::from(vec![Span::styled(
                file.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items)
        .block(files)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">");

    let selected_file = file_list
        .get(file_list_state
            .selected()
            .expect("file not selected"),
        )
        .expect("there is always selected file");

    let mut paragraphs = vec!();
    paragraphs.push(render_preview(selected_file));
    paragraphs.push(render_info(selected_file));

    (list, paragraphs)
}

fn render_preview<'a>(selected_file: &FileData) -> Paragraph<'a> {
    let mut preview = String::from("");
    if let Ok(text) = selected_file.preview() {
        preview = text;
    }

    Paragraph::new(preview)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightBlue))
                .title(" Preview ")
                .border_type(BorderType::Thick),
    )
}

fn render_info<'a>(selected_file: &FileData) -> Paragraph<'a> {
    Paragraph::new(selected_file.info())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Info ")
                .border_type(BorderType::Thick),
    )
}

fn open_file(path: &mut PathBuf, selected_file: &mut ListState) {
    if let Ok(files) = read_dir(&path) {
        if let Some(selected) = selected_file.selected() {
            let file = &files[selected];
            if file.is_file() {
                path.push(&file.name);
                Command::new("xdg-open")
                    .arg(&file.name)
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("failed opening file");
                path.pop();
            } else if file.is_dir() {
                path.push(&file.name);

                let files = read_dir(path).expect("files not loaded");
                selected_file.select(if files.len() > 0 {Some(0)} else {None});
                env::set_current_dir(&path).expect("invalid path");
            }
        }
    }
}