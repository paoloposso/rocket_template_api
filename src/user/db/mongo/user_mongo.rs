use crate::user::{errors::CustomError, models::user::User, repository::UserDbTrait};
use mongodb::{error::Result as MongoResult, Client, bson::{doc, oid::ObjectId}};

const DB_NAME: &str = "users_test";
const COLLECTION_NAME: &str = "users";

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
    async fn get_by_id(&self, id: &str) -> Result<User, CustomError> {
        let db = self.client.database(DB_NAME);

        let collection: mongodb::Collection<User> = db.collection(COLLECTION_NAME);

        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_e) => return Err(CustomError::GenericError("ID is not valid".to_owned())),
        };

        let filter = doc! {
            "_id": object_id,
        };

        let query_result = collection.find_one(filter, None).await?;

        if query_result.is_none() {
            return Err(CustomError::UserNotFound);
        }

        match query_result {
            Some(user_result) => {
                Ok(User {
                    id: Some(id.to_owned()),
                    name: user_result.name,
                    email: user_result.email,
                    password: user_result.password,
                })
            },
            None => Err(CustomError::UserNotFound),
        }
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        let db = self.client.database(DB_NAME);

        let collection = db.collection(COLLECTION_NAME);

        let doc = doc! {
            "name": user.name,
            "email": user.email,
            "password": user.password,
        };

        let inserted = collection.insert_one(doc.clone(), None).await;

        if let Err(err) = inserted {
            return Err(CustomError::from(err));
        }

        match inserted {
            Ok(result) => {
                match result.inserted_id.as_object_id() {
                    Some(object_id) => Ok(object_id.to_hex()),
                    None => Err(CustomError::GenericError("Inserted ID is not ObjectId".to_owned())),
                }
            },
            Err(err) => Err(CustomError::from(err)),
        }
    }

    async fn delete(&self, _id: &str) -> Result<(), CustomError> {
        Ok(())
    }
}
    