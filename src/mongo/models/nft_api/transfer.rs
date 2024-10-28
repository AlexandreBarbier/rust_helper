use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use log::error;
use mongodb::bson::DateTime;
use mongodb::{error::Error, results::InsertOneResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ModelCollection)]
pub struct Transfer {
    pub _id: ObjectId,
    pub date: DateTime,
    pub from: String,
    pub to: String,
    pub transaction: String,
    pub nft: Option<ObjectId>,
    pub block_height: Option<i64>,
    pub nft_id: i64,
    pub contract: ObjectId,
}

impl Default for Transfer {
    fn default() -> Self {
        Transfer::new(
            DateTime::now(),
            "0x0".to_string(),
            "0x0".to_string(),
            ObjectId::new(),
            "".to_string(),
            None,
            None,
            -1,
        )
    }
}
impl Transfer {
    pub fn new(
        date: DateTime,
        from: String,
        to: String,
        contract: ObjectId,
        transaction: String,
        nft: Option<ObjectId>,
        block_height: Option<i64>,
        nft_id: i64,
    ) -> Self {
        Transfer {
            _id: ObjectId::new(),
            date,
            from,
            to,
            transaction,
            nft,
            block_height,
            nft_id,
            contract,
        }
    }

    pub async fn create(
        date: DateTime,
        from: String,
        to: String,
        transaction: String,
        nft: Option<ObjectId>,
        nft_id: i64,
        block_height: Option<i64>,
        contract: ObjectId,
        client: &Client,
    ) -> Result<InsertOneResult, Error> {
        let transfer = Transfer {
            _id: ObjectId::new(),
            date,
            from,
            to,
            transaction,
            nft,
            block_height,
            nft_id,
            contract,
        };
        Transfer::get_collection(client).insert_one(transfer).await
    }

    pub async fn find(
        date: DateTime,
        from: String,
        to: String,
        nft: ObjectId,
        transaction: String,
        client: &Client,
    ) -> Option<Transfer> {
        let transfer_col = Transfer::get_collection(client);

        match transfer_col
            .find_one(mongo_doc! {
                "date": date.clone(),
                "from": from.clone(),
                "to": to.clone(),
                "nft": nft.clone(),
                "transaction": transaction.clone()
            })
            .await
        {
            Ok(Some(c)) => Some(c),
            Err(e) => {
                error!("{:?}", e);
                return None;
            }
            _ => None,
        }
    }

    pub async fn get_or_create(
        date: DateTime,
        from: String,
        to: String,
        transaction: String,
        nft_id: i64,
        block_height: Option<i64>,
        contract: ObjectId,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Option<(Transfer, bool)> {
        let transfer_col = Transfer::get_collection(client);

        match transfer_col
            .find_one(mongo_doc! {
                "date": date.clone(),
                "from": from.clone(),
                "to": to.clone(),
                "nft_id": nft_id,
                "contract": contract.clone(),
            })
            .await
        {
            Ok(Some(c)) => Some((c, false)),
            _ => {
                let transfer = Transfer {
                    _id: ObjectId::new(),
                    date,
                    from,
                    to,
                    transaction,
                    nft: None,
                    block_height,
                    nft_id,
                    contract,
                };
                let res = match session {
                    Some(x) => {
                        Transfer::get_collection(client)
                            .insert_one(transfer.clone())
                            .session(x)
                            .await
                    }
                    _ => {
                        Transfer::get_collection(client)
                            .insert_one(transfer.clone())
                            .await
                    }
                };
                if res.is_err() {
                    println!("transfer {:?}", res.err());
                }
                Some((transfer, true))
            }
        }
    }

    pub async fn update_nft(
        &self,
        nft: ObjectId,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) {
        let s_col = Transfer::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        let doc_update = mongo_doc! {
            "$set" : {
                "nft": nft
            }
        };
        let res = match session {
            Some(s) => s_col.update_one(q, doc_update).session(s).await,
            _ => s_col.update_one(q, doc_update).await,
        };
        if res.is_err() {
            error!("{:?}", res.err());
        }
    }

    pub async fn save(&self, client: &Client) -> Result<InsertOneResult, Error> {
        Transfer::get_collection(client).insert_one(self).await
    }
}
