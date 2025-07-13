// SIMPLE Workflow Script for Hyprland

use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use notify_rust::{Hint, Notification};
use std::{
    thread::{self, sleep},
    time,
};

struct Window {
    command: String,
    workspace: i32,
}

impl Window {
    async fn start(&self) {
        match Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
            self.workspace,
        ))) {
            Ok(_) => println!("Swicted to workspace"),
            Err(e) => eprintln!("{e}"),
        };

        sleep(time::Duration::from_millis(500));

        match Dispatch::call_async(DispatchType::Exec(&self.command.to_string())).await {
            Ok(_) => println!("ok2"),
            Err(e) => panic!("{e}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let editor = Window {
        command: String::from("alacritty -e nvim"),
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
        workspace: 3,
    };

    editor.start().await;
    browser.start().await;
    music.start().await;
    note.start().await;

    thread::sleep(time::Duration::from_millis(500));
    let _ = Notification::new()
        .summary("Workflow")
        .body("is ready to use.")
        .hint(Hint::Resident(true))
        .show();
}
