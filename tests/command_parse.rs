use std::{path::Path};

use rufile::command_input::input::{CommandHandler, InputMode};

#[test]
fn test_copy_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":c".to_string();
    command.exec(None);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":c c".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":cc c c".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_cut_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":m".to_string();
    command.exec(None);
    assert_ne!(InputMode::Normal, command.input_mode);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":m m".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":mm m m".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_delete_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":d".to_string();
    command.exec(None);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":d d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":d d d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_edit_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":e".to_string();
    command.exec(None);
    assert_ne!(InputMode::Normal, command.input_mode);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e 7777".to_string();
    command.exec(None);
    assert_ne!(InputMode::Normal, command.input_mode);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e d d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e 10".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e 1000".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":e 1a0".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_rename_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":r".to_string();
    command.exec(None);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":r d d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_create_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":n".to_string();
    command.exec(None);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":n".to_string();
    command.exec(Some(&file_name));
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":n s".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":n s file".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":n d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);


    command.input = ":n f".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}

#[test]
fn test_other_command() {
    let file_name = String::from("temp");
    Path::new(&file_name);
    
    let mut command = CommandHandler::default();

    command.input = ":".to_string();
    command.exec(None);
    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
    
    command.input = ":1".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":h".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ": h d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":cp d".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = ":cc".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);

    command.input = "c".to_string();
    command.exec(Some(&file_name));

    assert_eq!(InputMode::Error, command.input_mode);
}