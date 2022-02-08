use chrono::NaiveDateTime;
use sea_orm::{ActiveValue, IntoActiveModel};
use crate::CrudModelTrait;
use crate::db_user::ActiveModel;

#[derive(Clone, Debug, Default)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<crate::db_user::Model> for User {
    fn from(model: crate::db_user::Model) -> User {
        User {
            id: model.id,
            email: model.email,
            inserted_at: model.inserted_at,
            updated_at: model.updated_at,
        }
    }
}

impl CrudModelTrait<crate::db_user::Entity, crate::db_user::ActiveModel, UserCreate, UserUpdate> for User {}

#[derive(Clone, Debug, Default)]
pub struct UserCreate {
    pub email: String,
    pub password: String,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl IntoActiveModel<crate::db_user::ActiveModel> for UserCreate {
    fn into_active_model(self) -> crate::db_user::ActiveModel {
        let mut active_model = crate::db_user::ActiveModel {
            email: ActiveValue::Set(self.email),
            ..Default::default()
        };

        // Hashing password
        let password_hash = self.password;
        active_model.password_hash = ActiveValue::Set(password_hash);

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(Some(inserted_at));
        }
        if let Some(updated_at) = self.updated_at {
            active_model.updated_at = ActiveValue::Set(Some(updated_at));
        }

        active_model
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserUpdate {
    pub id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub inserted_at: Option<Option<NaiveDateTime>>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl IntoActiveModel<crate::db_user::ActiveModel> for UserUpdate {
    fn into_active_model(self) -> ActiveModel {
        let mut active_model = crate::db_user::ActiveModel {
            id: ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(email) = self.email {
            active_model.email = ActiveValue::Set(email);
        }
        if let Some(password) = self.password {
            // Hashing password
            let password_hash = password;
            active_model.password_hash = ActiveValue::Set(password_hash);
        }

        if let Some(inserted_at) = self.inserted_at {
            active_model.inserted_at = ActiveValue::Set(inserted_at);
        }
        if let Some(updated_at) = self.updated_at {
            active_model.updated_at = ActiveValue::Set(updated_at);
        }

        active_model
    }
}
