use crate::bot::{Context, Error};
use poise::samples::HelpConfiguration;

/// show some help
#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    command: Option<String>,
) -> Result<(), Error> {
    let extra_text_at_bottom = "Type `!help command` for more info on a command.";

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
