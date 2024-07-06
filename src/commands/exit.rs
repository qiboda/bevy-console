use bevy::app::AppExit;
use bevy::prelude::*;
use clap::Parser;

use crate as bevy_console;
use crate::ConsoleCommand;

/// Exits the app
#[derive(Parser, ConsoleCommand)]
#[command(name = "exit")]
pub(crate) struct ExitCommand {
    #[clap(short, long)]
    exit_code: u8,
}

pub(crate) fn exit_command(
    mut exit: ConsoleCommand<ExitCommand>,
    mut exit_writer: EventWriter<AppExit>,
) {
    if let Some(Ok(exit_cmd)) = exit.take() {
        exit_writer.send(AppExit::from_code(exit_cmd.exit_code));
        exit.ok();
    }
}
