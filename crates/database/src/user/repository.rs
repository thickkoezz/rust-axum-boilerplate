use crate::{Database, user::model::User};
use async_trait::async_trait;
use mongodb::{
  bson::{doc, oid::ObjectId},
  results::{DeleteResult, InsertOneResult, UpdateResult},
};
use std::{str::FromStr, sync::Arc};
use tokio_stream::StreamExt;
use utils::AppResult;

#[allow(clippy::module_name_repetitions)]
pub type DynUserRepository = Arc<dyn UserRepositoryTrait>;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
  async fn create_user(
    &self,
    name: &str,
    email: &str,
    password: &str,
  ) -> AppResult<InsertOneResult>;
  async fn get_all_users(&self) -> AppResult<Vec<User>>;

  async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>>;

  async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>>;

  async fn update_user(&self, id: &str, name: &str, email: &str) -> AppResult<UpdateResult>;

  async fn change_password(&self, id: &str, password: &str) -> AppResult<UpdateResult>;

  async fn delete_user(&self, id: &str) -> AppResult<DeleteResult>;
}

#[async_trait]
impl UserRepositoryTrait for Database {
  #[tracing::instrument(name = "Create User", skip(self, name, email, password))]
  async fn create_user(
    &self,
    name: &str,
    email: &str,
    password: &str,
  ) -> AppResult<InsertOneResult> {
    let new_doc = User {
      id: Some(ObjectId::new()),
      name: name.to_string(),
      email: email.to_string(),
      password: password.to_string(),
    };
    let result = self.user_col.insert_one(new_doc).await?;
    Ok(result)
  }

  #[tracing::instrument(name = "Get All Users", skip(self))]
  async fn get_all_users(&self) -> AppResult<Vec<User>> {
    let filter = doc! {};
    let mut cursor = self.user_col.find(filter).await?;
    let mut users: Vec<User> = Vec::new();
    while let Some(doc) = cursor.next().await {
      users.push(doc?);
    }
    Ok(users)
  }

  #[tracing::instrument(name = "Get User By Id", skip(self, id))]
  async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
    let id = ObjectId::from_str(id)?;
    let filter = doc! {"_id": id};
    let user = self.user_col.find_one(filter).await?;
    Ok(user)
  }

  #[tracing::instrument(name = "Get User By Email", skip(self, email))]
  async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
    let filter = doc! {"email": email};
    let user = self.user_col.find_one(filter).await?;
    Ok(user)
  }

  #[tracing::instrument(name = "Update User", skip(self, id, name, email))]
  async fn update_user(&self, id: &str, name: &str, email: &str) -> AppResult<UpdateResult> {
    let id = ObjectId::from_str(id)?;
    let filter = doc! {"_id": id};
    let new_doc = doc! { "$set": { "name": name, "email": email } };
    let result = self.user_col.update_one(filter, new_doc).await?;
    Ok(result)
  }

  #[tracing::instrument(name = "Change Password", skip(self, id, password))]
  async fn change_password(&self, id: &str, password: &str) -> AppResult<UpdateResult> {
    let id = ObjectId::from_str(id)?;
    let filter = doc! {"_id": id};
    let new_doc = doc! { "$set": { "password": password } };
    let result = self.user_col.update_one(filter, new_doc).await?;
    Ok(result)
  }

  #[tracing::instrument(name = "Delete User", skip(self, id))]
  async fn delete_user(&self, id: &str) -> AppResult<DeleteResult> {
    let id = ObjectId::from_str(id)?;
    let filter = doc! {"_id": id};
    let result = self.user_col.delete_one(filter).await?;
    Ok(result)
  }
}
