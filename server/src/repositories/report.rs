use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::RepositoryResult, models::Report};

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Report>>;
    async fn create(&self) -> RepositoryResult<Report>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<Report>;
}

#[derive(Clone)]
pub struct PostgresReportRepository {
    pool: PgPool,
}

impl PostgresReportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReportRepository for PostgresReportRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Report>> {
        let query = "
            SELECT *
            FROM reports
            WHERE id = $1
            RETURNING *;  
        ";
        let report = sqlx::query_as::<_, Report>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(report)
    }

    async fn create(&self) -> RepositoryResult<Report> {
        let query = "
            INSERT INTO reports
            DEFAULT VALUES
            RETURNING *;
        ";
        let report = sqlx::query_as::<_, Report>(query)
            .fetch_one(&self.pool)
            .await?;
        Ok(report)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<Report> {
        let query = "
            DELETE FROM reports
            WHERE id = $1
            RETURNING *;  
        ";
        let report = sqlx::query_as::<_, Report>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(report)
    }
}
