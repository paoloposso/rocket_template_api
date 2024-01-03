use crate::user::{errors::CustomError, models::user::User, repository::UserDbTrait};
use mongodb::{error::Result as MongoResult, Client, bson::{doc, self}};

const DB_NAME: &str = "users_test";

pub struct UserMongo {
    client: Client,
}

impl UserMongo {
    pub async fn new() -> MongoResult<Self> {
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl UserDbTrait for UserMongo {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError> {
        // let db = self.client.database(&DB_NAME.clone());
        // let collection = db.collection("users");

        // let filter = doc! {
        //     "id": id.clone(),
        // };

        // if let Some(document) = collection.find_one(filter, None).await? {
        //     let user: User = bson::from_document(document)?;
        //     Ok(user)
        // } else {
        //     Err(CustomError::UserNotFound)
        // }

        Err(CustomError::UserNotFound)
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        let db = self.client.database("users_test");

        let collection = db.collection("users");

        let doc = doc! {
            "id": user.id.clone(),
            "name": user.name,
            "email": user.email,
            "password": user.password,
        };

        if let Err(err) = collection.insert_one(doc, None).await {
            eprintln!("Failed to insert document: {:?}", err);
            return Err(CustomError::GenericError(err.to_string()));
        }

        Ok(user.id)
    }

    async fn delete(&self, id: String) -> Result<(), CustomError> {
        Ok(())
    }
}
    