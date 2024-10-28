use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
    pub profile: Option<ObjectId>,
}

impl Owner {
    pub fn new(address: String) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
            profile: None,
        }
    }

    pub async fn get_or_create(
        client: &Client,
        address: String,
        save: bool,
        session: Option<&mut ClientSession>,
    ) -> (Self, bool) {
        let owner_col = Owner::get_collection(client);
        let address = match address.as_str() {
            "null" => "0x0".to_string(),
            _ => address,
        };
        match owner_col.find_one(mongo_doc! {"address": &address}).await {
            Ok(Some(owner)) => (owner, false),
            _ => {
                let new_owner = Owner::new(address);
                if save {
                    let res = match session {
                        Some(s) => owner_col.insert_one(&new_owner).session(s).await,
                        _ => owner_col.insert_one(&new_owner).await,
                    };
                    if res.is_err() {
                        println!("owner {:?}", res.err());
                    }
                }
                (new_owner, true)
            }
        }
    }

    pub async fn update(
        &self,
        operation: Document,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let o_col = Owner::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        match session {
            Some(s) => o_col.update_one(q, operation).session(s).await,
            _ => o_col.update_one(q, operation).await,
        }
    }
}
