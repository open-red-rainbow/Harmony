use super::channels::{Channel, StageInstance, VoiceState};
use super::messages::{Emoji, Sticker};
use super::snowflakes::Snowflake;
use super::users::{Member, PresenceUpdate};

pub struct Guild {
    id: Snowflake,                                     // guild id
    name: &'static str, // guild name (2-100 characters, excluding trailing and leading whitespace)
    icon: &'static str, // icon hash
    icon_hash: Option<&'static str>, //icon hash, returned when in the template object
    splash: &'static str, // splash hash
    discovery_splash: &'static str, // discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    owner: Option<bool>,            // ** true if the user is the owner of the guild
    owner_id: Snowflake,            // id of owner
    permissions: Option<&'static str>, // ** total permissions for the user in the guild (excludes overwrites)
    region: Option<&'static str>,      // *** voice region id for the guild (deprecated)
    afk_channel_id: Snowflake,         // id of afk channel
    afk_timeout: i32,                  // afk timeout in seconds
    widget_enabled: Option<bool>,      //true if the server widget is enabled
    widget_channel_id: Option<Snowflake>, //the channel id that the widget will generate an invite to, or null if set to no invite
    verification_level: i32,              // verification level required for the guild
    default_message_notifications: i32,   // default message notifications level
    explicit_content_filter: i32,         // explicit content filter level
    roles: &'static [roles::Role],        // roles in the guild
    emojis: &'static [Emoji],             // custom guild emojis
    features: &'static [GuildFeature],    // enabled guild features
    mfa_level: i32,                       // required MFA level for the guild
    application_id: Snowflake, // application id of the guild creator if it is bot-created
    system_channel_id: Snowflake, // the id of the channel where guild notices such as welcome messages and boost events are posted
    system_channel_flags: i32,    // system channel flags
    rules_channel_id: Snowflake, // the id of the channel where Community guilds can display rules and/or guidelines
    joined_at: Option<super::DiscordTimestamp>, // * when this guild was joined at
    large: Option<bool>,         // * true if this is considered a large guild
    unavailable: Option<bool>,   // * true if this guild is unavailable due to an outage
    member_count: Option<i32>,   // * total number of members in this guild
    voice_states: Option<&'static [VoiceState]>, // * states of members currently in voice channels; lacks the guild_id key
    members: Option<&'static [Member]>,          // * users in the guild
    channels: Option<&'static [Channel]>,        // * channels in the guild
    threads: Option<&'static [Channel]>, // * all active threads in the guild that current user has permission to view
    presences: Option<&'static [PresenceUpdate]>, // * presences of the members in the guild, will only include non-offline members if the size is greater than large threshold
    max_presences: Option<i32>, //the maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    max_members: Option<i32>,   //the maximum number of members for the guild
    vanity_url_code: &'static str, // the vanity url code for the guild
    description: &'static str,  // the description of a Community guild
    banner: &'static str,       // banner hash
    premium_tier: i32,          // premium tier (Server Boost level)
    premium_subscription_count: Option<i32>, //the number of boosts this guild currently has
    preferred_locale: &'static str, // the preferred locale of a Community guild; used in server discovery and notices from Discord; defaults to "en-US"
    public_updates_channel_id: Snowflake, // the id of the channel where admins and moderators of Community guilds receive notices from Discord
    max_video_channel_users: Option<i32>, //the maximum amount of users in a video channel
    approximate_member_count: Option<i32>, //approximate number of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    approximate_presence_count: Option<i32>, //approximate number of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    welcome_screen: Option<WelcomeScreen>, //the welcome screen of a Community guild, shown to new members, returned in an Invite's guild object
    nsfw_level: i32,                       // guild NSFW level
    stage_instances: Option<&'static [StageInstance]>, // * Stage instances in the guild
    stickers: Option<&'static [Sticker]>,  //custom guild stickers
    guild_scheduled_events: [events::GuildScheduledEvent], // * the scheduled events in the guild
}

#[derive(strum_macros::Display, Debug)]
enum GuildFeature {
    #[strum(serialize = "ANIMATED_ICON")]
    AnimatedIcon, // guild has access to set an animated guild icon
    #[strum(serialize = "BANNER")]
    Banner, // guild has access to set a guild banner image
    #[strum(serialize = "COMMERCE")]
    Commerce, // guild has access to use commerce features (i.e. create store channels)
    #[strum(serialize = "COMMUNITY")]
    Community, // guild can enable welcome screen, Membership Screening, stage channels and discovery, and receives community updates
    #[strum(serialize = "DISCOVERABLE")]
    Discoverable, // guild is able to be discovered in the directory
    #[strum(serialize = "FEATURABLE")]
    Featurable, // guild is able to be featured in the directory
    #[strum(serialize = "INVITE_SPLASH")]
    InviteSplash, // guild has access to set an invite splash background
    #[strum(serialize = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled, // guild has enabled Membership Screening
    #[strum(serialize = "MONETIZATION_ENABLED")]
    MonetizationEnabled, // guild has enabled monetization
    #[strum(serialize = "MORE_STICKERS")]
    MoreStickers, // guild has increased custom sticker slots
    #[strum(serialize = "NEWS")]
    News, // guild has access to create news channels
    #[strum(serialize = "PARTNERED")]
    Partnered, // guild is partnered
    #[strum(serialize = "PREVIEW_ENABLED")]
    PreviewEnabled, // guild can be previewed before joining via Membership Screening or the directory
    #[strum(serialize = "PRIVATE_THREADS")]
    PrivateThreads, // guild has access to create private threads
    #[strum(serialize = "ROLE_ICONS")]
    RoleIcons, // guild is able to set role icons
    #[strum(serialize = "SEVEN_DAY_THREAD_ARCHIVE")]
    SevenDayThreadArchive, // guild has access to the seven day archive time for threads
    #[strum(serialize = "THREE_DAY_THREAD_ARCHIVE")]
    ThreeDayThreadArchive, // guild has access to the three day archive time for threads
    #[strum(serialize = "TICKETED_EVENTS_ENABLED")]
    TicketedEventsEnabled, // guild has enabled ticketed events
    #[strum(serialize = "VANITY_URL")]
    VanityUrl, // guild has access to set a vanity URL
    #[strum(serialize = "VERIFIED")]
    Verified, // guild is verified
    #[strum(serialize = "VIP_REGIONS")]
    VIPRegions, // guild has access to set 384kbps bitrate in voice (previously VIP voice servers)
    #[strum(serialize = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled, // guild has enabled the welcome screen
}

pub mod roles {
    pub struct Role {
        id: super::Snowflake,
    }
}

struct WelcomeScreen {
    description: &'static str,
    welcome_channels: &'static [WelcomeScreenChannel],
}

struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: &'static str,
    emoji_id: Snowflake,
    emoji_name: &'static str,
}

pub mod events {
    use super::{super::DiscordTimestamp, Snowflake};

    pub enum PrivacyLevel {
        GuildOnly = 2,
    }

    pub enum Status {
        Scheduled = 1,
        Active = 2,
        Completed = 3,
        Canceled = 4,
    }

    struct EntityMetadata {
        location: &'static str,
    }

    struct Common {
        id: Snowflake,
        guild_id: Snowflake,
        creator_id: Option<Snowflake>,
        name: &'static str,
        description: Option<&'static str>,
        scheduled_start_time: DiscordTimestamp,
        privacy_level: PrivacyLevel,
        status: Status,
        entity_id: Option<Snowflake>,
        creator: Option<super::super::users::User>,
        user_count: i32,
    }

    pub enum GuildScheduledEvent {
        StageInstance {
            d: Common,
            channel_id: Snowflake,
            scheduled_end_time: DiscordTimestamp,
        },
        Voice {
            d: Common,
            channel_id: Snowflake,
            scheduled_end_time: DiscordTimestamp,
        },
        External {
            d: Common,
            entity_metadata: EntityMetadata,
            scheduled_end_time: Option<DiscordTimestamp>,
        },
    }
}
