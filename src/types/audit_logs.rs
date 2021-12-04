use super::guilds::events::{PrivacyLevel, Status};
use super::guilds::roles::Role;
use super::messages::stickers::StickerFormat;
use super::permissions::overwrites::ChannelOverwrite;
use super::snowflakes::Snowflake;

pub enum AuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
}

union IntStr {
    i: i32,
    s: &'static str,
}

enum AuditLogChange {
    AfkChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // afk channel changed
    AfkTimeout {
        new: i32,
        old: i32,
    }, // * guild // afk timeout duration changed
    Allow {
        new: &'static str,
        old: &'static str,
    }, // * role // a permission on a text or voice channel was allowed for a role
    ApplicationId {
        new: Snowflake,
        old: Snowflake,
    }, // * channel // application id of the added or removed webhook or bot
    Archived {
        new: bool,
        old: bool,
    }, // * thread // thread is now archived/unarchived
    Asset {
        new: &'static str,
        old: &'static str,
    }, // * sticker // empty &'static str
    AutoArchiveDuration {
        new: i32,
        old: i32,
    }, // * thread // auto archive duration changed
    Available {
        new: bool,
        old: bool,
    }, // * sticker // availability of sticker changed
    AvatarHash {
        new: &'static str,
        old: &'static str,
    }, // * user // user avatar changed
    BannerHash {
        new: &'static str,
        old: &'static str,
    }, // * guild // guild banner changed
    Bitrate {
        new: i32,
        old: i32,
    }, // * channel // voice channel bitrate changed
    ChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * invite or guild scheduled event // channel for invite code or guild scheduled event changed
    Code {
        new: &'static str,
        old: &'static str,
    }, // * invite // invite code changed
    Color {
        new: i32,
        old: i32,
    }, // * role // role color changed
    Deaf {
        new: bool,
        old: bool,
    }, // * user // user server deafened/undeafened
    DefaultAutoArchiveDuration {
        new: i32,
        old: i32,
    }, // * channel // default auto archive duration for newly created threads changed
    DefaultMessageNotifications {
        new: i32,
        old: i32,
    }, // * guild // default message notification level changed
    Deny {
        new: &'static str,
        old: &'static str,
    }, // * role // a permission on a text or voice channel was denied for a role
    Description {
        new: &'static str,
        old: &'static str,
    }, // * guild or sticker or guild scheduled event // description changed
    DiscoverySplashHash {
        new: &'static str,
        old: &'static str,
    }, // * guild // discovery splash changed
    EnableEmoticons {
        new: bool,
        old: bool,
    }, // * integration // integration emoticons enabled/disabled
    EntityType {
        new: i32,
        old: i32,
    }, // * guild scheduled event // entity type of guild scheduled event was changed
    ExpireBehavior {
        new: i32,
        old: i32,
    }, // * integration // integration expiring subscriber behavior changed
    ExpireGracePeriod {
        new: i32,
        old: i32,
    }, // * integration // integration expire grace period changed
    ExplicitContentFilter {
        new: i32,
        old: i32,
    }, // * guild // change in whose messages are scanned and deleted for explicit content in the server
    FormatType {
        new: StickerFormat,
        old: StickerFormat,
    }, // * sticker // format type of sticker changed
    GuildId {
        new: Snowflake,
        old: Snowflake,
    }, // * sticker // guild sticker is in changed
    Hoist {
        new: bool,
        old: bool,
    }, // * role // role is now displayed/no longer displayed separate from online users
    IconHash {
        new: &'static str,
        old: &'static str,
    }, // * guild or role // icon changed
    Id {
        new: Snowflake,
        old: Snowflake,
    }, // * any // the id of the changed entity - sometimes used in conjunction with other keys
    InviterId {
        new: Snowflake,
        old: Snowflake,
    }, // * invite // person who created invite code changed
    Location {
        new: &'static str,
        old: &'static str,
    }, // * guild scheduled event // change in channel id for guild scheduled event
    Locked {
        new: bool,
        old: bool,
    }, // * thread // thread is now locked/unlocked
    MaxAge {
        new: i32,
        old: i32,
    }, // * invite // how long invite code lasts changed
    MaxUses {
        new: i32,
        old: i32,
    }, // * invite // change to max number of times invite code can be used
    Mentionable {
        new: bool,
        old: bool,
    }, // * role // role is now mentionable/unmentionable
    MfaLevel {
        new: i32,
        old: i32,
    }, // * guild // two-factor auth requirement changed
    Mute {
        new: bool,
        old: bool,
    }, // * user // user server muted/unmuted
    Name {
        new: &'static str,
        old: &'static str,
    }, // * any // name changed
    Nick {
        new: &'static str,
        old: &'static str,
    }, // * user // user nickname changed
    Nsfw {
        new: bool,
        old: bool,
    }, // * channel // channel nsfw restriction changed
    OwnerId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // owner changed
    PermissionOverwrites {
        new: &'static [ChannelOverwrite],
        old: &'static [ChannelOverwrite],
    }, // * channel // permissions on a channel changed
    Permissions {
        new: &'static str,
        old: &'static str,
    }, // * role // permissions for a role changed
    Position {
        new: i32,
        old: i32,
    }, // * channel // text or voice channel position changed
    PreferredLocale {
        new: &'static str,
        old: &'static str,
    }, // * guild // preferred locale changed
    PrivacyLevel {
        new: PrivacyLevel,
        old: PrivacyLevel,
    }, // * stage instance or guild scheduled event // privacy level of the stage instance changed
    PruneDeleteDays {
        new: i32,
        old: i32,
    }, // * guild // change in number of days after which inactive and role-unassigned members are kicked
    PublicUpdatesChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // id of the public updates channel changed
    RateLimitPerUser {
        new: i32,
        old: i32,
    }, // * channel // amount of seconds a user has to wait before sending another message changed
    Region {
        new: &'static str,
        old: &'static str,
    }, // * guild // region changed
    RulesChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // id of the rules channel changed
    SplashHash {
        new: &'static str,
        old: &'static str,
    }, // * guild // invite splash page artwork changed
    Status {
        new: Status,
        old: Status,
    }, // * guild scheduled event // status of guild scheduled event was changed
    SystemChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // id of the system channel changed
    Tags {
        new: &'static str,
        old: &'static str,
    }, // * sticker // related emoji of sticker changed
    Temporary {
        new: bool,
        old: bool,
    }, // * invite // invite code is temporary/never expires
    Topic {
        new: &'static str,
        old: &'static str,
    }, // * channel or stage instance // text channel topic or stage instance topic changed
    Type {
        new: IntStr,
        old: IntStr,
    }, // * any // type of entity created
    UnicodeEmoji {
        new: &'static str,
        old: &'static str,
    }, // * role // role unicode emoji changed
    UserLimit {
        new: i32,
        old: i32,
    }, // * voice channel // new user limit in a voice channel
    Uses {
        new: i32,
        old: i32,
    }, // * invite // number of times invite code used changed
    VanityUrlCode {
        new: &'static str,
        old: &'static str,
    }, // * guild // guild invite vanity url changed
    VerificationLevel {
        new: i32,
        old: i32,
    }, // * guild // required verification level changed
    WidgetChannelId {
        new: Snowflake,
        old: Snowflake,
    }, // * guild // channel id of the server widget changed
    WidgetEnabled {
        new: bool,
        old: bool,
    }, // * guild // server widget enabled/disable
    Add {
        new: &'static [Role],
        old: &'static [Role],
    }, // * guild // new role added
    Remove {
        new: &'static [Role],
        old: &'static [Role],
    }, // * guild // role removed
}

enum AuditEntryInfo {
    ChannelOverwrite {
        id: Snowflake,
        role_name: &'static str,
        entity_type: &'static str,
    },
    MemberDisconnect {
        count: &'static str,
    },
    MemberMove {
        channel_id: Snowflake,
        count: &'static str,
    },
    MemberPrune {
        delete_member_days: &'static str,
        members_removed: &'static str,
    },
    MessageBulkDelete {
        count: &'static str,
    },
    MessageDelete {
        channel_id: Snowflake,
        count: &'static str,
    },
    MessagePinned {
        channel_id: Snowflake,
        message_id: Snowflake,
    },
    StageInstance {
        channel_id: Snowflake,
    },
}

struct AuditLogEntry {
    target_id: Snowflake,
    changes: Option<&'static [AuditLogChange]>,
    user_id: Option<Snowflake>,
    id: Snowflake,
    action_type: AuditLogEvent,
    options: Option<AuditEntryInfo>,
    reason: &'static str,
}

pub struct AuditLog {}
