use mongodb::{
	Client,
	Collection,
	Database,
};
use tokio::sync::RwLock;

pub struct DatabasePool {
	database: RwLock<Database>,
}

impl DatabasePool {
	pub async fn new() -> Self {
		let uri: String = dotenv::var("MONGODB_URI").unwrap();
		let client: Client = Client::with_uri_str(&uri).await.expect("Failed to connect");
		let node_name: String = dotenv::var("NODE_NAME").unwrap();

		let database: Database = client.database(&node_name);

		DatabasePool {
			database: RwLock::new(database),
		}
	}

	pub async fn get_collection<T>(&self, name: &str) -> Collection<T> {
		self.database.read().await.collection(name)
	}

	pub async fn get_collection_mut<T>(&self, name: &str) -> Collection<T> {
		self.database.write().await.collection(name)
	}
}
