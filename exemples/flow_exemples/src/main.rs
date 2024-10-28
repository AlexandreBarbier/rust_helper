use rust_helpers::flow_rs::access::{GetTransactionRequest, PingRequest, SendTransactionRequest};
use rust_helpers::flow_rs::cadence_json::ValueOwned;
use rust_helpers::flow_rs::proto::{
    access::{
        access_api_client::AccessApiClient, BlockResponse, GetBlockByHeightRequest,
        GetCollectionByIdRequest, GetLatestBlockRequest, GetTransactionsByBlockIdRequest,
    },
    execution::GetTransactionResultRequest,
};
use rust_helpers::flow_rs::FlowNetwork;
use rust_helpers::mongo::{self, models::Spork};
use rust_helpers::redis::client::Redis;

#[tokio::main]
async fn main() {
    let m_client = mongo::client::create().await;
    let r_client = Redis::new();
    let sp = Spork::get(&m_client, 12020512).await.unwrap();
    println!("{:?}", sp);
    // loop {
    //     let r = match client
    //         .get_block_by_height(GetBlockByHeightRequest {
    //             height: latest_block_height,
    //             full_block_response: true,
    //         })
    //         .await
    //     {
    //         Ok(x) => x.into_inner(),
    //         Err(e) => {
    //             println!("{:?}", e);
    //             continue;
    //         }
    //     };
    //     let block = r.clone().block.unwrap();

    //     latest_block_height += 1;

    //     println!(
    //         "block height: {}, id: {:?}",
    //         block.height,
    //         hex::encode(block.id.clone())
    //     );

    //     let k = block.id.clone();
    //     for collection in block.collection_guarantees {
    //         let col = match client
    //             .get_collection_by_id(GetCollectionByIdRequest {
    //                 id: collection.collection_id,
    //             })
    //             .await
    //         {
    //             Ok(x) => x.into_inner(),
    //             Err(e) => {
    //                 println!("{:?}", e);
    //                 continue;
    //             }
    //         };

    //         for tr_id in col.collection.clone().unwrap().clone().transaction_ids {
    //             let tran = match client
    //                 .get_transaction_result(GetTransactionRequest {
    //                     id: tr_id,
    //                     block_id: k.clone(),
    //                     collection_id: col.collection.clone().unwrap().id.clone(),
    //                 })
    //                 .await
    //             {
    //                 Ok(x) => x.into_inner(),
    //                 Err(e) => {
    //                     println!("{:?}", e);
    //                     continue;
    //                 }
    //             };
    //             for event in tran.events {
    //                 if let ValueOwned::Event(event_payload) =
    //                     serde_json::from_slice(&event.payload).unwrap()
    //                 {
    //                     println!("{:?}", event_payload.id);
    //                 }
    //             }
    //         }
    //     }
    // }
}
