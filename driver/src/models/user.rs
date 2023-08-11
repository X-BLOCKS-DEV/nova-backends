use serde::{Serialize, Deserialize};
use validator::Validate;

use nova_app::models::user::{UserView, CreateUser, UpdateUserView};

#[derive(Debug, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUser {
    pub id: String,
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserView> for JsonUser {
    fn from(uv: UserView) -> Self {
        Self {
            id: uv.id,
            name: uv.name,
            email: uv.email,
            is_admin: uv.is_admin,
            password_hash: uv.password_hash,
            created_at: uv.created_at.to_string(),
            updated_at: uv.updated_at.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreateUser {
    #[validate(required(message = "`name` is null."))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(required(message = "`is_admin` is null."))]
    pub is_admin: Option<bool>,
    #[validate(required(message = "`password_hash` is null."))]
    pub password_hash: Option<String>,
}

impl From<JsonCreateUser> for CreateUser {
    fn from(jc: JsonCreateUser) -> Self {
        CreateUser {
            name: jc.name.unwrap(),
            email: jc.email.unwrap(),
            is_admin: jc.is_admin.unwrap(),
            password_hash: jc.password_hash.unwrap(),
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub password_hash: Option<String>,
}

impl JsonUpdateUser {
    pub fn validate(self, id: String) -> Result<UpdateUserView, Vec<String>> {
        let mut errors: Vec<String> = vec![];

        Ok(UpdateUserView::new(
            id,
            self.name,
            self.email,
            self.is_admin,
            self.password_hash
        ))
    }
}
