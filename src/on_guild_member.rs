use serenity::model::{
    id::GuildId,
    id::UserId,
    user::User,
    guild::Member,
};
use serenity::prelude::*;

pub async fn removal(ctx: Context, guild_id: GuildId, user: User, _maybe_member: Option<Member>) {
    println!("{:?} left guild, cleaning up channels", &user);
    match guild_id.channels(&ctx.http).await {
        Err(error) => println!("Error while listing channels: \n{}", error),
        Ok(channels) => {
            let UserId(user_id) = user.id;
            let string_id = format!("{}", user_id);
            for (_chan_id, channel) in channels {
                if let Some(_) = channel.name.find(&string_id) {
                    println!("Deleting channel {:?}", &channel);
                    match channel.delete(&ctx.http).await {
                        Ok(_) => {},
                        Err(error) => println!("Error while deleting channel {}: \n{}", &channel, error)
                    };
                }
            }
        }
    };
}
