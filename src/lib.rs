mod types;
mod gateway;

use types::users::User;

pub struct DiscordErr;

pub struct Bot {
    token: &'static str,
    user: &'static User
}

impl Bot {

    pub fn get_token(&self) -> &str {
        self.token
    }

    pub fn self_user(&self) -> &User {
        self.user
    }

    pub async fn run(&self) {
        let url = match gateway::resolve(self).await {
            Ok(u) => u,
            Err(_) => String::new()
        };
        gateway::connect(self, &url).await;
    }

}