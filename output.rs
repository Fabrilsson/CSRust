use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = Vec<Item>;

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct DbContext
{
   pub items: Arc<RwLock<Items>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct Item
{
   pub id: i32,
   pub name: String,
   pub quantity: i32,
   pub value: f64,
}

impl DbContext { 
	fn new() -> Self {
		 DbContext { 
			items: Arc::new(RwLock::new(HashMap::new()))
		}
	}
}

async fn getitemsasyncasdasd (_context: DbContext) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = Vec::new();

    let r = _context.items.read();
    for value in r.iter() {
        result.push(value);
    }

	Ok(warp::reply::json(&result))
}

#[tokio::main]
async fn main() {
	let _context = DbContext::new()
	let _context_dbcontext = warp::any().map(move || _context.clone());

	let get_items = warp::get()
		.and(warp::path("v1"))
		.and(warp::path("groceries"))
		.and(warp::path::end())