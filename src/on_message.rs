use serenity::{
    model::{channel::Message,
            id::RoleId,
            id::GuildId,
            id::ChannelId,
            channel::ChannelType,
            ModelError,
            Permissions,
            channel::{PermissionOverwrite, PermissionOverwriteType},
    },
    prelude::*,
};

use eyre::{Result, WrapErr};

use crate::consts::*;

pub async fn on_message(ctx: Context, msg: Message) {
    if let Some(ref member) = msg.member {
        let unverified_role = RoleId(UNVERIFIED_ROLE_ID);
        
        // If the message was sent by a member with an Unverified role, dispatch to the appropriate
        // function.
        if member.roles.contains(&unverified_role){
            match on_message_by_unverified(ctx, msg).await {
                Err(error) => println!("Error while processing unverified message:\n{}", error),
                Ok(_) => (),
        };
    }


/*
    if msg.content == "!ping" {
        // Sending a message can fail, due to a network error, an
        // authentication error, or lack of permissions to post in the
        // channel, so log to stdout when some error happens, with a
        // description of it.
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }
    }*/
    }
}

async fn on_message_by_unverified(ctx: Context, msg: Message) -> Result<()> {
    // If the message is the verification command and it is sent in the correct channel,
    // then ignore it; the other bot will take care of removing the Unverified role.
    if msg.channel_id == ChannelId(VERIFY_CHANNEL_ID) {
        if msg.content.starts_with(VERIFY_COMMAND) {
            println!("Message ok!");
            return Ok(());
        }
    }

    println!("Unverified message {:?} not OK, quarantining user", &msg);

    // If either of the conditions is not met, we want to quarantine the user.
    // Start by swapping out the Unverified role for the Suspicious role:
    let unverified_role = RoleId(UNVERIFIED_ROLE_ID);
    let sus_role = RoleId(SUSPICIOUS_ROLE_ID);
    let guild_id = GuildId(GUILD_ID);

    let guild = guild_id.to_guild_cached(&ctx.cache).await.ok_or(ModelError::ItemMissing).wrap_err_with(|| format!("Failed to get guild {:?} from cache", guild_id))?;
    let mut member = guild_id.member(&ctx.http, &msg.author).await.wrap_err_with(|| format!("Failed to get member {:?} from guild {:?}", &msg.author.id, guild_id))?;

    member.add_role(&ctx.http, sus_role).await.wrap_err_with(|| format!("Failed to add sus role to member {:?}", member))?;
    member.remove_role(&ctx.http, unverified_role).await.wrap_err_with(|| format!("Failed to remove unverified role from member {:?}", member))?;

    // Now create a channel corresponding to the member ID
    // and grant the member access to it.

    let interview_channel_name = format!("interview-{}", member.user.id);

    let deny_all = PermissionOverwrite {
        allow: Permissions::empty(),
        deny:Permissions::READ_MESSAGES,
        // The @everyone role has the same ID as the guild it belongs to:
        // https://discord.com/developers/docs/topics/permissions#role-object
        kind: PermissionOverwriteType::Role(RoleId(guild_id.0)),
    };
    let allow_member = PermissionOverwrite {
        allow: Permissions::SEND_MESSAGES | Permissions::READ_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(member.user.id),
    };
    let permissions = vec![deny_all, allow_member];

    guild.create_channel(&ctx.http,
                         |c| c
                         .name(interview_channel_name)
                         .kind(ChannelType::Text)
                         .category(INTERVIEW_CATEGORY)
                         .permissions(permissions))
        .await.wrap_err("Failed to create interview channel")?;

    Ok(())

}

