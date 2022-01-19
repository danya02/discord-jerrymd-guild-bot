use std::env;

mod consts;
mod on_message;
mod on_guild_member;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::GuildId, guild::Member, user::User},
    prelude::*,
    client::bridge::gateway::GatewayIntents,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Called when a message is created.
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(guild_id) = msg.guild_id {
            if guild_id == consts::GUILD_ID {
               // This message is in the guild we're interested in, calling the message handler.
               on_message::on_message(ctx, msg).await
            }
        } else {
            // Message was received not over gateway or in a private context, ignoring.
            ()
        }
    }

    /// Called when a user stops being a guild member
    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, maybe_member_data: Option<Member>){
        // If the event happens outside of the guild of interest, ignore it.
        if guild_id != consts::GUILD_ID {
            return;
        }

       on_guild_member::removal(ctx, guild_id, user, maybe_member_data).await
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected! Running with configuration {:?}", ready.user.name, consts::CONFIGURATION_NAME);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES;

    let mut client =
        Client::builder(&token).event_handler(Handler).intents(intents).await.expect("Err creating client");

    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
