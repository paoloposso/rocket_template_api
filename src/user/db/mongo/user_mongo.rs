use crate::user::{errors::CustomError, models::user::User, repository::UserDbTrait};
use mongodb::{error::Result as MongoResult, Client, bson::{doc, oid::ObjectId}};

const DB_NAME: &str = "users_test";
const COLLECTION_NAME: &str = "users";

pub struct UserMongo {
    client: Client,
}

impl UserMongo {
    pub async fn new(uri: &str) -> MongoResult<Self> {
        let client = Client::with_uri_str(uri).await?;
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
            Err(_e) => return Err(CustomError::GenericError("ID is not valid".into())),
        };

        if let Some(query_result) = collection.find_one(doc! {"_id": object_id}, None).await? {
            return Ok(User {
                id: Some(id.into()),
                name: query_result.name,
                email: query_result.email,
                password: query_result.password,
            });
        }

        Err(CustomError::UserNotFound)
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
                    None => Err(CustomError::GenericError("Inserted ID is not ObjectId".into())),
                }
            },
            Err(err) => Err(CustomError::from(err)),
        }
    }

    async fn delete(&self, _id: &str) -> Result<(), CustomError> {
        Ok(())
    }
}
    