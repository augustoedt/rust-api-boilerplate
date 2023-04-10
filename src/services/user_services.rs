use serde::Serialize;

#[derive(Serialize)]
pub struct UsersResponse {
    pub users: Vec<String>,
}

pub struct UserService;

impl UserService {
    pub fn get_users() -> UsersResponse {
        let users = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
            "David".to_string(),
        ];
        UsersResponse { users }
    }
}
