use bson::Document;
use mongodb::Client;

pub trait ModelCollection
where
    Self: Sized,
{
    fn get_col_name() -> String;
    fn get_db_name() -> String;
    fn get_collection(client: &Client) -> mongodb::Collection<Self>
    where
        Self: Send,
        Self: Sync,
    {
        client
            .database(Self::get_db_name().as_str())
            .collection::<Self>(Self::get_col_name().as_str())
    }
    fn get_collection_document(client: &Client) -> mongodb::Collection<Document> {
        client
            .database(Self::get_db_name().as_str())
            .collection::<Document>(Self::get_col_name().as_str())
    }
}
