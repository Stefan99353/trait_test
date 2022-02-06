mod db_user;
mod user_dtos;
use async_trait::async_trait;
use sea_orm::entity::{ActiveModelTrait, EntityTrait};
use sea_orm::{
    Condition, ConnectionTrait, DbConn, DbErr, IntoActiveModel, PrimaryKeyTrait, QueryFilter,
};

#[async_trait]
pub trait CrudModelTrait
where
    Self: From<<Self::Entity as EntityTrait>::Model> + Sized,
{
    type Entity: EntityTrait;
    type ActiveModel: ActiveModelTrait + Send;
    type CreateModel: IntoActiveModel<Self::ActiveModel> + Send;
    type UpdateModel: IntoActiveModel<Self::ActiveModel> + Send;

    async fn get(
        id: <<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        db: &DbConn,
    ) -> Result<Option<Self>, DbErr> {
        Self::Entity::find_by_id(id)
            .one(db)
            .await
            .map(|res| res.map(|m| m.into()))
    }

    async fn create(cm: Self::CreateModel, db: &DbConn) -> Result<Self, DbErr> {
        cm.into_active_model().insert(db).await.map(|m| m.into())
    }

    async fn update(um: Self::UpdateModel, db: &DbConn) -> Result<Self, DbErr> {
        um.into_active_model().update(db).await.map(|m| m.into())
    }

    async fn delete<'a, C>(condition: Condition, db: &'a C) -> Result<u64, DbErr>
    where
        C: ConnectionTrait<'a>,
    {
        Self::Entity::delete_many()
            .filter(condition)
            .exec(db)
            .await
            .map(|res| res.rows_affected)
    }
}

fn main() {
    println!("Hello, world!");
}
