use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command("drop")]
#[only_in(guild)]
#[allowed_roles("Tournament Admin")]
#[description("Drops a player from the tournament.")]
async fn drop(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    todo!()
}

