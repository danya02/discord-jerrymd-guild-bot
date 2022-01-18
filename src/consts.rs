#[cfg(not(feature="production"))]
/// The name of the configuration the app was compiled for.
/// Useful for checking how a build script is working.
pub const CONFIGURATION_NAME: &str = "testing";

#[cfg(feature="production")]
pub const CONFIGURATION_NAME: &str = "production";

#[cfg(not(feature="production"))]
/// The ID of the guild to react to events in; all other guilds will be ignored.
/// If not building for production, this is my main bot testing guild.
pub const GUILD_ID: u64 = 775744109359923221;

#[cfg(feature="production")]
pub const GUILD_ID: u64 = 930050496952139867;

