// SIMPLE Workflow Script

use hyprland::dispatch::{Dispatch, DispatchType};
use std::{thread, time};

struct Window {
    command: String,
    workspace: i32,
}

impl Window {
    fn open(&self) {
        // Opening the app
        match Dispatch::call(DispatchType::Exec(&self.command.to_string())) {
            Ok(_) => {
                println!("Launching {}", &self.command);
            }
            Err(e) => {
                eprintln!("Error {e}");
            }
        }
        // Sleeping 2000ms
        thread::sleep(time::Duration::from_millis(2000));
        // Moving app to desired workspace
        match Dispatch::call(DispatchType::MoveToWorkspaceSilent(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(self.workspace),
            None,
        )) {
            Ok(_) => {
                println!("Moving to workspace {}", &self.workspace);
            }
            Err(e) => {
                eprintln!("Error {e}");
            }
        }
    }
}

fn main() {
    let editor = Window {
        command: String::from("alacritty --hold -e nvim"),
        workspace: 1,
    };
    let browser = Window {
        command: String::from("firefox"),
        workspace: 2,
    };
    let music = Window {
        command: String::from("youtube-music"),
        workspace: 3,
    };
    let note = Window {
        command: String::from("md.obsidian.Obsidian"),
        workspace: 4,
    };
    //println!("Opening, {}", editor.command);
    editor.open();
    thread::sleep(time::Duration::from_millis(1000));
    browser.open();
    thread::sleep(time::Duration::from_millis(1000));
    music.open();
    thread::sleep(time::Duration::from_millis(1000));
    note.open();
}
