use crate::database::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub async fn builder() -> anyhow::Result<Self> {
        Ok(AppState {
            database: Database::builder().await?,
        })
    }
}
