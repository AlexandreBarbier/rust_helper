use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;

use mongodb::{Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Spork {
    pub _id: ObjectId,
    pub version: i32,
    pub start_height: i64,
    pub access_url: String,
    pub live_start: Option<i64>,
    pub latest_requested_block: i64,
    pub end_height: Option<i64>,
}

impl Spork {
    pub fn new(
        start_height: i64,
        end_height: Option<i64>,
        access_url: String,
        version: i32,
    ) -> Self {
        Spork {
            _id: ObjectId::new(),
            version,
            start_height,
            access_url,
            latest_requested_block: start_height,
            end_height,
            ..Default::default()
        }
    }

    pub async fn create(
        client: &Client,
        start_height: i64,
        access_url: String,
        version: i32,
        end_height: Option<i64>,
    ) -> Self {
        let sp = Spork::new(start_height, end_height, access_url, version);
        sp.clone().save(client).await;
        sp
    }

    pub async fn save(self, client: &Client) {
        let _ = Spork::get_collection(client).insert_one(self).await;
    }

    pub async fn update_requested_block(
        self,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) {
        let s_col = Spork::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        let doc_update = mongo_doc! {
            "$set" : {
                "latest_requested_block": self.latest_requested_block
            }
        };
        let _ = match session {
            Some(s) => s_col.update_one(q, doc_update).session(s).await,
            _ => s_col.update_one(q, doc_update).await,
        };
    }

    pub async fn get(client: &Client, height: i64) -> Option<Spork> {
        let s_col = Spork::get_collection(client);
        s_col
            .find_one(mongo_doc! { "start_height": { "$lt": height } })
            .sort(mongo_doc! { "start_height": -1 })
            .await
            .unwrap()
    }
}
