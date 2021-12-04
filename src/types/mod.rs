pub mod audit_logs;
pub mod channels;
pub mod guilds;
pub mod messages;
pub mod permissions;
pub mod users;

pub mod snowflakes {
    pub type Snowflake = i64;

    use chrono::NaiveDateTime;

    const DISCORD_EPOCH: i64 = 1_420_070_400_000; // seconds 

    pub fn snowflake_timestamp(snowflake: Snowflake) -> NaiveDateTime {
        let s = (snowflake >> 22) + DISCORD_EPOCH;
        let s = s / 1000;
        let ms = (s % 1000) as u32;
        NaiveDateTime::from_timestamp(s, ms)
    }

    pub fn new(timestamp: NaiveDateTime) -> Snowflake {
        let ms = timestamp.timestamp_millis() - DISCORD_EPOCH;
        ms << 22
    }
}

pub struct DiscordTimestamp;