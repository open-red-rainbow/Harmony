pub enum GatewayEvents {
	Dispatch = 0
	Heartbeat = 1
	Identify = 2
	PresenceUpdate = 3
	VoiceStateUpdate = 4	
	Resume = 6	
	Reconnect = 7	
	RequestGuildMembers = 8
	InvalidSession = 9
	Hello = 10	
	HeartbeatACK = 11
}

pub enum GatewayDispatch {
	Ready {}
	Resumed
	ChannelCreate
	ChannelUpdate
	ChannelDelete
	ChannelPinsUpdate
	ThreadCreate
	ThreadUpdate
	ThreadDelete
	ThreadListSync
	ThreadMemberUpdate
	ThreadMembersUpdate
	GuildCreate
	GuildUpdate
	GuildDelete
	GuildBanAdd
	GuildBanRemove
	GuildEmojisUpdate
	GuildStickersUpdate
	GuildIntegrationsUpdate
	GuildMemberAdd
	GuildMemberRemove
	GuildMemberUpdate
	GuildMembersChunk
	GuildRoleCreate
	GuildRoleUpdate
	GuildRoleDelete
	IntegrationCreate
	IntegrationUpdate
	IntegrationDelete
	InteractionCreate
	InviteCreate
	InviteDelete
	MessageCreate
	MessageUpdate
	MessageDelete
	MessageDeleteBulk
	MessageReactionAdd
	MessageReactionRemove
	MessageReactionRemoveAll
	MessageReactionRemoveEmoji
	PresenceUpdate
	StageInstanceCreate
	StageInstanceDelete
	StageInstanceUpdate
	TypingStart
	UserUpdate
	VoiceStateUpdate
	VoiceServerUpdate
	WebhooksUpdate
}