use super::snowflakes::Snowflake;

pub enum Permission {
    CreateInstantInvite = 0,
    KickMembers = 1,
    BanMembers = 2,
    Administrator = 3,
    ManageChannels = 4,
    ManageGuild = 5,
    AddReactions = 6,
    ViewAuditLog = 7,
    PrioritySpeaker = 8,
    Stream = 9,
    ViewChannel = 10,
    SendMessages = 11,
    SendTtsMessages = 12,
    ManageMessages = 13,
    EmbedLinks = 14,
    AttachFiles = 15,
    ReadMessageHistory = 16,
    MentionEveryone = 17,
    UseExternalEmojis = 18,
    ViewGuildInsights = 19,
    Connect = 20,
    Speak = 21,
    MuteMembers = 22,
    DeafenMembers = 23,
    MoveMembers = 24,
    UseVad = 25,
    ChangeNickname = 26,
    ManageNicknames = 27,
    ManageRoles = 28,
    ManageWebhooks = 29,
    ManageEmojisAndStickers = 30,
    UseApplicationCommands = 31,
    RequestToSpeak = 32,
    ManageEvents = 33,
    ManageThreads = 34,
    CreatePublicThreads = 35,
    CreatePrivateThreads = 36,
    UseExternalStickers = 37,
    SendMessagesInThreads = 38,
    StartEmbeddedActivities = 39,
}

pub mod overwrites {
    use super::Snowflake;

    pub struct ChannelOverwrite {
        id: Snowflake,
        overwrite_type: OverwriteType,
        allow: u64,
        deny: u64,
    }

    enum OverwriteType {
        Role,
        Member,
    }

    impl ChannelOverwrite {
        pub fn allow(&mut self, permission: super::Permission, to: bool) {
            let bitmask = ((to as u8) << permission as u8) as u64;
            self.allow = self.allow | bitmask;
            self.deny = self.deny & !bitmask;
        }

        pub fn deny(&mut self, permission: super::Permission, to: bool) {
            let bitmask = ((to as u8) << permission as u8) as u64;
            self.allow = self.allow & !bitmask;
            self.deny = self.deny | bitmask;
        }

        pub fn inherit(&mut self, permission: super::Permission, to: bool) {
            let bitmask = ((to as u8) << permission as u8) as u64;
            self.allow = self.allow & !bitmask;
            self.deny = self.deny & !bitmask;
        }
    }

}
