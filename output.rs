use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HaspMap<i32, Item>;

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct Item
{
   pub id: i32,
   pub name: String,
   pub quantity: i32,
   pub value: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct Store
{
   pub items: Arc<RwLock<Items>>,
}

impl Store { 
	fn new() -> Self {
		 Store { 
			items: Arc::new(RwLock::new(HashMap::new()))
		}
	}
}

async fn getitemsasync (store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();

    let r = store.items.read();
    for (key,value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(&result))
}
