mod db_user;
mod user_dtos;

use async_trait::async_trait;
use sea_orm::entity::{ActiveModelTrait, EntityTrait};
use sea_orm::{ActiveModelBehavior, Condition, ConnectionTrait, DbErr, IntoActiveModel, PrimaryKeyTrait, QueryFilter};

#[async_trait]
pub trait CrudModelTrait<E, A, C, U>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity=E> + ActiveModelBehavior + Send + 'static,
        C: IntoActiveModel<A> + Send + 'static,
        U: IntoActiveModel<A> + Send + 'static,
        Self: From<<E as sea_orm::EntityTrait>::Model> + Sized,
{
    async fn get<'a, DB>(
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        db: &'a DB,
    ) -> Result<Option<Self>, DbErr> where
        DB: ConnectionTrait<'a>, {
        E::find_by_id(id)
            .one(db)
            .await
            .map(|res| res.map(|m| m.into()))
    }

    async fn create<'a, DB>(cm: C, db: &'a DB) -> Result<Self, DbErr>  where
        DB: ConnectionTrait<'a>, {
        cm.into_active_model().insert(db).await.map(|m| Self::from(m))
    }

    async fn update<'a, DB>(um: U, db: &'a DB) -> Result<Self, DbErr>  where
        DB: ConnectionTrait<'a>, {
        um.into_active_model().update(db).await.map(|m| Self::from(m))
    }

    async fn delete<'a, DB>(condition: Condition, db: &'a DB) -> Result<u64, DbErr> where
        DB: ConnectionTrait<'a>,
    {
        E::delete_many()
            .filter(condition)
            .exec(db)
            .await
            .map(|res| res.rows_affected)
    }
}

fn main() {
    println!("Hello, world!");
}
