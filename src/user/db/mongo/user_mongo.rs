use crate::{
    core::auth::create_jwt,
    user::{
        errors::CustomError,
        models::{
            use_case::user::{ GetAllUserResponse, GetUserResponse, UserCredential, UserProfile },
            user::User,
        },
        repository::UserDbTrait,
    },
};
use bcrypt::verify;
use mongodb::{ error::Result as MongoResult, Client, bson::{ doc, Document }, Collection };
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
    async fn login(&self, credential: UserCredential) -> Result<UserProfile, CustomError> {
        let db = self.client.database(&self.db_name);
        let collection: mongodb::Collection<User> = db.collection(COLLECTION_NAME);
        let user = collection.find_one(doc! { "email": credential.email }, None).await;

        if let Ok(Some(user)) = user {
            let my_id = user.id.clone();
            let my_password = user.password.clone();
            if verify(credential.password, &my_password).unwrap() {
                match create_jwt(my_id) {
                    Ok(token) => {
                        Ok(UserProfile {
                            data: GetUserResponse {
                                id: user.id.clone(),
                                name: user.name.clone(),
                                email: user.email.clone(),
                            },
                            token: token.clone(),
                        })
                    }
                    Err(err) => Err(CustomError::GenericError(err.to_string())),
                }
            } else {
                Err(CustomError::UserNotFound)
            }
        } else {
            Err(CustomError::UserNotFound)
        }
    }

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
            let user_responses: Vec<GetUserResponse> = users
                .into_iter()
                .map(|user| GetUserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                })
                .collect();
            return Ok(GetAllUserResponse {
                data: user_responses,
            });
        }

        Err(CustomError::UserNotFound)
    }
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        let db = self.client.database(&self.db_name);
        let collection: mongodb::Collection<User> = db.collection(COLLECTION_NAME);
        if let Some(query_result) = collection.find_one(doc! { "id": id }, None).await? {
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
        let email = user.email.clone();
        let id = user.id.clone();
        let doc =
            doc! {
            "id": user.id,
            "name": user.name,
            "email": user.email,
            "password": user.password,
        };
        let exist_user = collection.find_one(
            doc! { "$or":[{"id": id},{ "email": &email}] },
            None
        ).await;
        if let Ok(Some(_)) = exist_user {
            return Err(CustomError::EmailAlreadyExists);
        }

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
            }
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

        let deleted = collection.delete_one(doc! { "id": id }, None).await;

        if let Err(err) = deleted {
            return Err(CustomError::from(err));
        }

        Ok(())
    }
}
