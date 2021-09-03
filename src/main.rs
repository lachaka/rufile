use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, error, fs, io};

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::Terminal;
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
use tui::widgets::{
    Block,
    BorderType,
    Borders, List,
    ListItem,
    ListState,
    Paragraph
};

mod entry;
mod event;
mod command_input;

use entry::file_data::FileData;
use event::{Event, Events};
use command_input::input::{CommandHandler, InputMode};

fn main() -> Result<(), Box<dyn error::Error>> {
    let events: Events = Events::new();
    let mut command: CommandHandler = CommandHandler::default();

    let mut path: PathBuf = env::current_dir().unwrap();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut marked_file = ListState::default();
    marked_file.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .horizontal_margin(1)
                .direction(Direction::Vertical)
                .constraints([
                        Constraint::Min(3),
                        Constraint::Length(1),
                    ].as_ref()
                )
                .split(f.size());
            
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref()
                )
                .split(chunks[0]);
            
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

            let (list, mut paragraphs) = 
                render_files(&mut marked_file, &path);
        
            f.render_stateful_widget(list, main_chunks[0], &mut marked_file);
            f.render_widget(paragraphs.remove(1), right[1]);
            f.render_widget(paragraphs.remove(0), right[0]);

            let input_chunk = render_input_field(&command);
            f.render_widget(input_chunk, chunks[1]);

            match command.input_mode {
                InputMode::Editing => {
                    f.set_cursor(
                        chunks[1].x + command.input.len() as u16,
                        chunks[1].y,
                    )
                }
                _ => {}
            }
        })?;
        
        match events.rx.recv()? {
            Event::Input(input) => match command.input_mode {
                InputMode::Normal | InputMode::Error => match input {
                    Key::Char('q') | Key::Ctrl('c') => break,
                    Key::Up => on_up_pressed(&path, &mut marked_file),
                    Key::Down => on_down_pressed(&path, &mut marked_file),
                    Key::Right => on_right_pressed(&mut path, &mut marked_file),
                    Key::Left => on_left_pressed(&mut path, &mut marked_file),
                    Key::Char(':') => {
                        command.input.push(':');
                        command.input_mode = InputMode::Editing;
                    }
                    _ => {}
                }
                InputMode::Editing => match input {
                    Key::Char('\n') => call_command(&mut command, &path, &marked_file),
                    Key::Char(c) => command.input.push(c),
                    Key::Backspace => { command.input.pop(); }
                    Key::Esc => {
                        command.input.drain(..);
                        command.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
            },
            Event::Tick => {},
        }
    }

    Ok(())
}

fn render_files<'a>(marked_file: &mut ListState, path: &PathBuf) 
        -> (List<'a>, Vec<Paragraph<'a>>) {
    let files = read_dir(&path).unwrap();

    let file_list_view = render_file_list(path, &files);
    
    let mut selected_file: Option<&FileData>  = None;
    
    if marked_file.selected().is_none() {
        if files.len() > 0 {
            marked_file.select(Some(0));
        }
    }
    
    if let Some(idx) = marked_file.selected() {
        selected_file = files.get(idx);
    }

    let mut paragraphs = vec!();
    paragraphs.push(render_preview(selected_file));
    paragraphs.push(render_info(selected_file));

    (file_list_view, paragraphs)
}

fn render_file_list<'a>(path: &PathBuf, files: &Vec<FileData>) -> List<'a> {
    let title = format!(" {} ", path.to_string_lossy());
    
    let files_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(title)
        .border_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Thick);

    let items: Vec<_> = files
        .iter()
        .map(|file| {
            ListItem::new(Spans::from(vec![Span::styled(
                file.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let file_view = List::new(items)
        .block(files_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">");

    file_view
}

fn render_preview<'a>(selected_file: Option<&FileData>) -> Paragraph<'a> {
    let mut preview = String::from("");
    if let Some(file) = selected_file {
        if let Ok(text) = file.preview() {
            preview = text;
        }
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

fn render_info<'a>(selected_file: Option<&FileData>) -> Paragraph<'a> {
    let mut info = String::from("");
    if let Some(file) = selected_file {
        info = file.info();
    }

    Paragraph::new(info)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Info ")
                .border_type(BorderType::Thick),
        )
}

fn render_input_field(command: &CommandHandler) -> Paragraph {
    let text = match command.input_mode {
        InputMode::Error => {
            Spans::from(vec![Span::styled("Invalid command", 
                        Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::REVERSED))
            ])
        },
        _ => Spans::from(command.input.as_ref())
    };

    Paragraph::new(text)
        .style(match command.input_mode {
            InputMode::Error => Style::default(),
            _ => Style::default(),
        })
        .block(Block::default())
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

fn open_file(file_name: &str) {
    Command::new("xdg-open")
        .arg(file_name)
        .stderr(Stdio::null())
        .spawn().ok();
}

fn open_dir(path: &mut PathBuf, marked_file: &mut ListState) -> io::Result<()> {
    let files = read_dir(path)?;

    if files.len() > 0 {
        marked_file.select(Some(0));
    } else {
        marked_file.select(None);
    }

    env::set_current_dir(&path)?;

    Ok(())
}

fn on_right_pressed(path: &mut PathBuf, marked_file: &mut ListState) {
    if let Ok(files) = read_dir(&path) {
        if let Some(selected) = marked_file.selected() {
            let file = &files[selected];
            path.push(&file.name);

            if file.is_file() {
               open_file(&file.name);
               path.pop();
            } else if file.is_dir() {
               open_dir(path, marked_file).ok();
            }
        }
    }
}

fn on_left_pressed(path: &mut PathBuf, marked_file: &mut ListState) {
    path.pop();
    env::set_current_dir(&path).ok();
    marked_file.select(Some(0));
}

fn on_up_pressed(path: &PathBuf, marked_file: &mut ListState) {
    if let Some(selected) = marked_file.selected() {
        let files_count = read_dir(&path).unwrap().len(); 
        if selected > 0 {
            marked_file.select(Some(selected - 1));
        } else {
            marked_file.select(Some(files_count - 1));
        }
    }
}

fn on_down_pressed(path: &PathBuf, marked_file: &mut ListState) {
    if let Some(selected) = marked_file.selected() {
        let files_count = read_dir(&path).unwrap().len(); 
        if selected >= files_count - 1 {
            marked_file.select(Some(0));
        } else {
            marked_file.select(Some(selected + 1));
        }
    }
}

fn call_command(command: &mut CommandHandler, path: &PathBuf, marked_file: &ListState) {
    match marked_file.selected() {
        Some(selected) => {
            let files = read_dir(&path);
            if let Ok(files) = files {
                command.exec(Some(&files[selected].name));
            } 
        }
        None => command.exec(None) 
    }
}