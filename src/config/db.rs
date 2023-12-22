use mongodb::{ Client, Database, Collection };

pub async fn database() -> Database {    
    let uri: String = dotenv::var("MONGODB_URI").unwrap();
    let client: Client = Client::with_uri_str(&uri).await.expect("Failed to connect");
    let node_name: String = dotenv::var("NODE_NAME").unwrap();

    client.database(&node_name)
}

pub async fn collection<T>(name: &str) -> Collection<T> {
    let database: Database = database().await;
    database.collection(name)
}