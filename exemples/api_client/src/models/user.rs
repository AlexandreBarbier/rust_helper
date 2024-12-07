use log::error;
use mongo::models::common::ModelCollection;
use rust_helpers::mongo::{
    self,
    models::mongo_doc,
    mongodb::{
        bson::{self, oid::ObjectId},
        error::Error,
        results::{InsertOneResult, UpdateResult},
        Client,
    },
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

impl User {
    fn new(name: String, email: String) -> Self {
        User {
            _id: ObjectId::new(),
            name,
            email,
            description: None,
        }
    }

    pub async fn get_or_create(
        name: String,
        email: String,
        client: &Client,
    ) -> Option<(Self, bool)> {
        let user_col = User::get_collection(client);
        println!("{:?}", user_col);
        if let Ok(user) = user_col.find_one(mongo_doc! {"email": email.clone()}).await {
            match user {
                Some(usr) => Some((usr, false)),
                _ => {
                    let new = User::new(name, email);
                    let _ = new.clone().save(client).await;
                    println!("{:?}", new);
                    Some((new, true))
                }
            }
        } else {
            println!("error");
            error!("error");
            None
        }
    }

    pub async fn find(id: String, client: &Client) -> Option<Self> {
        match User::get_collection(client)
            .find_one(mongo_doc! {"_id":id.clone()})
            .await
        {
            Ok(user) => user,
            _ => None,
        }
    }

    pub async fn save(self, client: &Client) -> Result<InsertOneResult, Error> {
        let user_col = User::get_collection(client);
        user_col.insert_one(self).await
    }

    pub async fn update(self, client: &Client) -> Result<UpdateResult, Error> {
        let user_col = User::get_collection(client);
        user_col
            .update_one(
                mongo_doc! {"_id": self._id.to_hex()},
                mongo_doc! {
                    "$set": bson::to_document(&self).unwrap()
                },
            )
            .upsert(true)
            .await
    }
}

impl ModelCollection for User {
    fn get_col_name() -> String {
        "users".to_string()
    }

    fn get_db_name() -> String {
        "tests".to_string()
    }
}
