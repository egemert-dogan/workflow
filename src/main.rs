// SIMPLE Workflow Script for Hyprland

// Import libraries
use hyprland::dispatch::{Dispatch, DispatchType};
use std::{process::exit, thread, time};

struct Window {
    command: String,
    workspace: i32,
}

impl Window {
    fn open(&self) {
        // Open the app
        match Dispatch::call(DispatchType::Exec(&self.command.to_string())) {
            Ok(_) => {
                println!("Launching app: {}", &self.command);
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
        // Sleep 3s
        println!("Sleeping for 3 seconds");
        thread::sleep(time::Duration::from_millis(3000));
        // Send app to the workspace
        match Dispatch::call(DispatchType::MoveToWorkspaceSilent(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(self.workspace),
            None,
        )) {
            Ok(_) => {
                println!("Moving app to workspace {}", &self.workspace);
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

fn main() {
    // Define the editor
    let editor = Window {
        command: String::from("alacritty --hold -e nvim"),
        workspace: 1,
    };
    // Define the browser
    let browser = Window {
        command: String::from("firefox"),
        workspace: 2,
    };
    // Define the music player
    let music = Window {
        command: String::from("youtube-music"),
        workspace: 3,
    };
    // Define the note taking app
    let note = Window {
        command: String::from("md.obsidian.Obsidian"),
        workspace: 4,
    };
    // Open editor and sleep 1s
    editor.open();
    thread::sleep(time::Duration::from_millis(1000));
    // Open browser and sleep 1s
    browser.open();
    thread::sleep(time::Duration::from_millis(1000));
    // Open music player and sleep 1 s
    music.open();
    thread::sleep(time::Duration::from_millis(1000));
    // Open note taking app
    note.open();

    // Exit
    println!("Done.");
    exit(0);
}
