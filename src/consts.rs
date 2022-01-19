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

#[cfg(not(feature="production"))]
/// The role ID corresponding to the "unverified" state.
///
/// In the guild, there is a verification process:
/// 1. All users get the "Unverified" role on joining (managed by Dyno)
/// 2. The "Unverified" role gives write access to:
///  - a verification channel in which users can send a message to lose the role (managed by Dyno)
///  - some honeypot channels (they can be written to, but doing so is suspicious)
/// 3. If a user sends the correct verification message in the correct channel (see
///    `VERIFY_CHANNEL_ID`), then they lose the role (managed by Dyno)
/// 4. If a user sends a message to a honeypot channel, or sends the wrong message to the
///    verification channel, they:
///    - lose the "Unverified" role
///    - gain the "Suspicious" role
///    - a channel is created such that only they have explicit access to it.
/// 5. If the user loses the "Suspicious" role or leaves while they have that role, the channel is
///    deleted.
pub const UNVERIFIED_ROLE_ID: u64 = 933122032835657778;

#[cfg(feature="production")]
pub const UNVERIFIED_ROLE_ID: u64 = 932565235561287700;


#[cfg(not(feature="production"))]
pub const SUSPICIOUS_ROLE_ID: u64 = 933122346825429054;

#[cfg(feature="production")]
pub const SUSPICIOUS_ROLE_ID: u64 = 933122675772108860;


#[cfg(feature="production")]
pub const VERIFY_CHANNEL_ID: u64 = 932566212217536543;
#[cfg(not(feature="production"))]
pub const VERIFY_CHANNEL_ID: u64 = 933123422488231976;


pub const VERIFY_COMMAND: &str = "?member";


#[cfg(not(feature="production"))]
pub const INTERVIEW_CATEGORY: u64 = 933307552521924668;

#[cfg(feature="production")]
pub const INTERVIEW_CATEGORY: u64 = 933307056155426836;


//#[cfg(not(feature="production"))]
//#[cfg(feature="production")]

