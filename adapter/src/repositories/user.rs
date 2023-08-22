use chrono::Utc;
use nova_core::models::Id;
use nova_core::models::user::{User, NewUser, UpdateUser, UpsertUser};
use nova_core::repositories::user::UserRepository;
use crate::models::user::{StoredUser, InsertStoredUser, UpdateStoredUser, UpsertStoredUser};
use crate::repositories::DatabaseRepositoryImpl;

use sqlx::query_as;

use async_trait::async_trait;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>> {
        let pool = self.db.0.clone();
        let sql = r#"
            select
                id,
                name,
                email,
                is_admin,
                password_hash,
                created_at,
                updated_at
            from
                users
            where
                id = $1
        "#;
        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_user {
            Some(su) => Ok(Some(su.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewUser) -> anyhow::Result<User> {
        let pool = self.db.0.clone();
        let user: InsertStoredUser = source.into();
        let id = user.id.clone();
        let insert_sql = r#"
            insert
            into
            users (id, name, email, is_admin, password_hash, created_at, updated_at)
            values ($1, $2, $3, $4, $5, $6, $7)
        "#;

        let now = Utc::now();

        let _ = sqlx::query(insert_sql)
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.is_admin)
            .bind(&user.password_hash)
            .bind(&now)
            .bind(&now)
            .execute(&*pool)
            .await?;

        let select_sql = r#"
            select
                id,
                name,
                email,
                is_admin,
                password_hash,
                created_at,
                updated_at
            from
                users
            where
                id = $1
        "#;

        let stored_user = query_as::<_, StoredUser>(select_sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_user.try_into()?)
    }

    async fn update(&self, source: UpdateUser) -> anyhow::Result<User> {
        let pool = self.db.0.clone();
        let user: UpdateStoredUser = source.into();
        let id = user.id.clone();

        let now = Utc::now();

        let update_sql = r#"
            update
                users as t
            set
                name = case when $2 is not null then $2 else c.name end,
                email = case when $3 is not null then $3 else c.email end,
                is_admin = case when $4 is not null then $4 else c.is_admin end,
                password_hash = case when $5 is not null then $5 else c.password_hash end,
                updated_at = $6
            from
                (select * from users where id = $1) as c
            where
                t.id = $1
        "#;

        let _ = sqlx::query(update_sql)
            .bind(user.id)
            .bind(user.name)
            .bind(user.email)
            .bind(user.is_admin)
            .bind(user.password_hash)
            .bind(now)
            .execute(&*pool)
            .await?;

        let sql = r#"
            select
                id,
                name,
                email,
                is_admin,
                password_hash,
                created_at,
                updated_at
            from
                users
            where
                id = $1
        "#;

        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_user.try_into()?)
    }

    async fn upsert(&self, source: UpsertUser) -> anyhow::Result<User> {
        let pool = self.db.0.clone();
        let user: UpsertStoredUser = source.into();
        let id = user.id.clone();

        let upsert_sql = r#"
            insert
            into
            users (id, name, email, is_admin, password_hash, created_at, updated_at)
            values ($1, $2, $3, $4, $5, $6, $7)
            on conflict on constraint pk_users_id
            do update set
                name = $2
                email = $3
                is_admin = $4
                password_hash = $5
                updated_at = $7
        "#;

        let now = Utc::now();
        let _ = sqlx::query(upsert_sql)
            .bind(user.id)
            .bind(user.name)
            .bind(user.email)
            .bind(user.is_admin)
            .bind(user.password_hash)
            .bind(now)
            .bind(now)
            .execute(&*pool)
            .await?;

        let sql = r#"
            select
                id,
                name,
                email,
                is_admin,
                password_hash,
                created_at,
                updated_at
            from
                users
            where
                id = $1
        "#;

        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_user.try_into()?)
    }

    async fn delete(&self, id: &Id<User>) -> anyhow::Result<Option<User>> {
        let pool = self.db.0.clone();

        let sql = r#"
            select
                id,
                name,
                email,
                is_admin,
                password_hash,
                created_at,
                updated_at
            from
                users
            where
                id = $1
        "#;

        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_user {
            Some(su) => {
                let delete_sql = r#"
                    delete from users where id = $1
                "#;

                let _ = sqlx::query(delete_sql)
                    .bind(id.value.to_string())
                    .execute(&*pool)
                    .await?;

                Ok(Some(su.try_into()?))
            }
            None => Ok(None),
        }
    }
}