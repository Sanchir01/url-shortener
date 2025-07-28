use crate::feature::auth::entity::UserDB;
use async_trait::async_trait;
use mockall::automock;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Error, Pool, Postgres, query_as};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_all_users(&self) -> Result<Vec<UserDB>, Error>;
    async fn create_user(
        &self,
        title: String,
        email: String,
        password: Vec<u8>,
    ) -> Result<UserDB, Error>;
    async fn get_user_by_email(&self, email: String) -> Result<Option<UserDB>, Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    primary_db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new_user_repository(primary_db: Pool<Postgres>) -> Self {
        Self { primary_db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn get_all_users(&self) -> Result<Vec<UserDB>, Error> {
        let (query, _) = Query::select()
            .columns([
                "id",
                "title",
                "email",
                "password",
                "role",
                "created_at",
                "updated_at",
                "version",
            ])
            .from("users")
            .build(PostgresQueryBuilder);

        let all_users = query_as::<_, UserDB>(&query)
            .fetch_all(&self.primary_db)
            .await
            .map_err(|err| {
                eprintln!("❌ Error fetching users: {:?}", err);
                err
            })?;

        Ok(all_users)
    }
    async fn create_user(
        &self,
        title: String,
        email: String,
        password: Vec<u8>,
    ) -> Result<UserDB, Error> {
        let (query, args) = Query::insert()
            .into_table(Alias::new("users"))
            .columns([
                Alias::new("title"),
                Alias::new("email"),
                Alias::new("password"),
            ])
            .values_panic([
                title.clone().into(),
                email.clone().into(),
                password.clone().into(),
            ])
            .returning(Query::returning().columns([
                Alias::new("id"),
                Alias::new("title"),
                Alias::new("email"),
                Alias::new("password"),
                Alias::new("role"),
                Alias::new("created_at"),
                Alias::new("updated_at"),
                Alias::new("version"),
            ]))
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx::query_as_with::<_, UserDB, _>(&query, args)
            .fetch_one(&self.primary_db)
            .await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserDB>, Error> {
        let (query, args) = Query::select()
            .columns([
                "id",
                "title",
                "email",
                "password",
                "role",
                "created_at",
                "updated_at",
                "version",
            ])
            .from("users")
            .and_where(Expr::col((Alias::new("users"), Alias::new("email"))).eq(email))
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx::query_as_with::<_, UserDB, _>(&query, args)
            .fetch_optional(&self.primary_db)
            .await
            .map_err(|err| {
                eprintln!("❌ Error fetching user by email: {:?}", err);
                err
            })?;

        Ok(user)
    }
}
