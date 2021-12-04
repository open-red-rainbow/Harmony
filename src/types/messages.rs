use crate::types::snowflakes::Snowflake;
use crate::types::users::User;

pub use stickers::Sticker;
pub use emojis::Emoji;

pub mod stickers {
    use super::Snowflake;
    use super::User;

    pub struct Sticker {
        id: Snowflake,
        pack_id: Option<Snowflake>,
        name: &'static str,
        description: Option<&'static str>,
        tags: Option<&'static str>,
        m_type: Option<StickerType>,
        format_type: StickerFormat, 
        available: Option<bool>,
        guild_id: Option<Snowflake>,
        user: Option<User>,
        sort_value: Option<i32>,
    }

    enum StickerType {
        STANDARD = 1,
        GUILD = 2,
    }

    pub enum StickerFormat {
        PNG = 1,
        APNG = 2,
        LOTTIE = 3,
    }

    pub struct StickerPack {
        id: Snowflake,
        name: &'static str,
        sku_id: Snowflake,
        cover_sticker_id: Option<Snowflake>,
        description: &'static str,
        stickers: [Sticker],
    }
}

pub mod emojis {
    use super::Snowflake;

    pub struct Emoji {
        id: Snowflake,
    }

}
