use std::sync::Arc;
use uuid::Uuid;

use crate::domain::*;
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;
use crate::validation::{normalize_url, validate_why};

pub struct VaultService {
    repo: Arc<dyn Repository>,
}

impl VaultService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self, filters: VaultFilters) -> DomainResult<Vec<VaultItem>> {
        self.repo.list_vault_items(filters).await
    }

    pub async fn get(&self, id: Uuid) -> DomainResult<VaultItem> {
        self.repo.get_vault_item(id).await
    }

    pub async fn create(&self, mut input: CreateVaultItem) -> DomainResult<VaultItem> {
        // Validate why (hard rule: must reject if missing)
        validate_why(&input.why)?;

        // If type is link, URL is required
        if input.item_type == VaultItemType::Link && input.url.is_none() {
            return Err(DomainError::validation("URL is required for link items"));
        }

        // Normalize URL and extract hostname
        if let Some(ref url) = input.url {
            let normalized = normalize_url(url)?;
            input.url = Some(normalized);
        }

        self.repo.create_vault_item(input).await
    }

    pub async fn update(&self, id: Uuid, update: UpdateVaultItem) -> DomainResult<VaultItem> {
        // Validate why if being updated
        if let Some(ref why) = update.why {
            validate_why(why)?;
        }

        self.repo.update_vault_item(id, update).await
    }

    pub async fn delete(&self, id: Uuid) -> DomainResult<()> {
        self.repo.delete_vault_item(id).await
    }

    pub async fn stale_items(&self, hours: i64) -> DomainResult<Vec<VaultItem>> {
        self.repo.stale_inbox_items(hours).await
    }
}
