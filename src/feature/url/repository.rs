use crate::domain::url::Url;
use async_trait::async_trait;
use mockall::{automock, predicate::*};
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres, query_as};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UrlRepositoryTrait: Send + Sync {
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>;
    async fn add_url(&self, url: String, aliase: String) -> Result<(), sqlx::Error>;
    async fn delete_url(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct UrlRepository {
    primary_db: Pool<Postgres>,
}

impl UrlRepository {
    pub fn new_url_repository(primary_db: Pool<Postgres>) -> Self {
        Self { primary_db }
    }
}

#[async_trait]
impl UrlRepositoryTrait for UrlRepository {
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error> {
        let (sql, _) = Query::select()
            .columns(["id", "alias", "url"])
            .from("url")
            .build(PostgresQueryBuilder);
        let urls = query_as::<_, Url>(&sql)
            .fetch_all(&self.primary_db)
            .await
            .map_err(|err| {
                eprintln!("❌ Error fetching urls: {:?}", err);
                err
            })?;
        Ok(urls)
    }
    async fn add_url(&self, url: String, aliase: String) -> Result<(), sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("url"))
            .columns([Alias::new("url"), Alias::new("alias")])
            .values_panic([url.clone().into(), aliase.clone().into()])
            .build(PostgresQueryBuilder);

        println!("sql string value {:?}", values);
        sqlx::query(&sql)
            .bind(&url)
            .bind(&aliase)
            .execute(&self.primary_db)
            .await?;

        Ok(())
    }
    async fn delete_url(&self, id: Uuid) -> Result<(), sqlx::Error> {
        let (sql, _) = Query::delete()
            .from_table("url")
            .and_where(Expr::col("id").eq(Expr::val(id.to_string())))
            .build(PostgresQueryBuilder);

        sqlx::query(&sql).bind(id).execute(&self.primary_db).await?;
        Ok(())
    }
}
