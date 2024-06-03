use crate::user::{errors::CustomError, models::{use_case::user::{GetAllUserResponse, GetUserResponse}, user::User}, repository::UserDbTrait};
use mongodb::{error::Result as MongoResult, Client, bson::{doc, Document}, Collection};
use rocket::futures::TryStreamExt;

const COLLECTION_NAME: &str = "users";

pub struct UserMongo {
    client: Client,
    db_name: String,
}

impl UserMongo {
    pub async fn new(uri: &str, db_name: &str) -> MongoResult<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client, db_name: db_name.into() })
    }
}

#[async_trait]
impl UserDbTrait for UserMongo {
    async fn get_all(&self) -> Result<GetAllUserResponse, CustomError> {
        let db = self.client.database(&self.db_name);

        let collection: mongodb::Collection<User> = db.collection(COLLECTION_NAME);
        // let mut cursor = collection.(doc! {}, None).await?;
        let mut cursor = collection.find(doc! {}, None).await?;
        let mut users: Vec<User> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            println!("{:?}", serde_json::to_value(&result).unwrap());
            users.push(result);
        }
        if !users.is_empty() {
            let user_responses: Vec<GetUserResponse> = users.into_iter().map(|user| GetUserResponse {
                id: user.id.unwrap_or_else(|| "_id".to_string()),
                name: user.name,
                email: user.email,
            }).collect();
            return Ok(GetAllUserResponse {
                data: user_responses
            });
        }

        Err(CustomError::UserNotFound)
    }
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        let db = self.client.database(&self.db_name);

        let collection: mongodb::Collection<User> = db.collection(COLLECTION_NAME);

        // let object_id = match ObjectId::parse_str(id) {
        //     Ok(oid) => oid,
        //     Err(_e) => return Err(CustomError::GenericError("ID is not valid".into())),
        // };

        if let Some(query_result) = collection.find_one(doc! {"id": id}, None).await? {
            return Ok(GetUserResponse {
                id: id.into(),
                name: query_result.name,
                email: query_result.email,
            });
        }

        Err(CustomError::UserNotFound)
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        let db = self.client.database(&self.db_name);

        let collection = db.collection(COLLECTION_NAME);
        let uuid = uuid::Uuid::new_v4();
        let doc = doc! {
            "id": uuid.to_string(),
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
                    Some(uuid) => Ok(uuid.to_string()),
                    None => Err(CustomError::GenericError("Inserted ID is not ObjectId".into())),
                }
            },
            Err(err) => Err(CustomError::from(err)),
        }
    }

    async fn delete(&self, id: &str) -> Result<(), CustomError> {
        let db = self.client.database(&self.db_name);

        let collection: Collection<Document> = db.collection(COLLECTION_NAME);

        // let object_id = match ObjectId::parse_str(id) {
        //     Ok(oid) => oid,
        //     Err(_e) => return Err(CustomError::GenericError("ID is not valid".into())),
        // };

        let deleted = collection.delete_one(doc! {"id": id}, None).await;

        if let Err(err) = deleted {
            return Err(CustomError::from(err));
        }

        Ok(())
    }
}
    