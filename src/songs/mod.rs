#![allow(proc_macro_derive_resolution_fallback)]

pub mod handlers;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableSong {
    pub name: Option<String>,
}

impl InsertableSong {
    fn from_song(songs: Song) -> InsertableSong {
        InsertableSong {
            name: songs.name,
        }
    }
}
