extern crate x11_clipboard;

use std::time::{Duration, Instant};
use x11_clipboard::Clipboard;

#[test]
fn test_store_and_load() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;
    let atom_property = clipboard.setter.atoms.property;

    clipboard
        .store(atom_clipboard, atom_utf8string, data.as_bytes())
        .unwrap();

    let output = clipboard
        .load(atom_clipboard, atom_utf8string, atom_property, None)
        .unwrap();
    assert_eq!(output, data.as_bytes());

    let data = format!("{:?}", Instant::now());
    clipboard
        .store(atom_clipboard, atom_utf8string, data.as_bytes())
        .unwrap();

    let output = clipboard
        .load(atom_clipboard, atom_utf8string, atom_property, None)
        .unwrap();
    assert_eq!(output, data.as_bytes());

    let output = clipboard
        .load(atom_clipboard, atom_utf8string, atom_property, None)
        .unwrap();
    assert_eq!(output, data.as_bytes());

    let dur = Duration::from_secs(3);
    let output = clipboard
        .load(atom_clipboard, atom_utf8string, atom_property, dur)
        .unwrap();
    assert_eq!(output, data.as_bytes());
}

#[test]
fn test_list_targets() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;

    clipboard
        .store(atom_clipboard, atom_utf8string, data.as_bytes())
        .unwrap();

    let output = clipboard
        .list_target_names(atom_clipboard, Duration::from_millis(100).into())
        .unwrap();
    assert_eq!(
        [b"TARGETS".to_vec(), b"UTF8_STRING".to_vec(),].as_slice(),
        output
    );
}

#[test]
fn test_clear() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;

    clipboard
        .store(atom_clipboard, atom_utf8string, data.as_bytes())
        .unwrap();

    clipboard.clear(atom_clipboard).unwrap();
    let output = clipboard
        .list_target_names(atom_clipboard, Duration::from_millis(100).into())
        .unwrap();
    assert!(output.is_empty());
}

#[test]
fn test_store_multiple() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;
    let targets = vec![
        (atom_utf8string, data.as_bytes()),
        (
            clipboard.getter.get_atom("test2", false).unwrap(),
            data.as_bytes(),
        ),
    ];

    clipboard.store_multiple(atom_clipboard, targets).unwrap();

    let output = clipboard
        .list_target_names(atom_clipboard, Duration::from_millis(100).into())
        .unwrap();
    assert_eq!(
        [
            b"TARGETS".as_slice(),
            b"UTF8_STRING".as_slice(),
            b"test2".as_slice(),
        ]
        .as_slice(),
        output
    );
}
