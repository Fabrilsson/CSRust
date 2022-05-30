use warp::{http, Filter};
use serde::{Serialize, Deserialize};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DbContext
{
   pub items: Arc<RwLock<Vec<Item>>>,
}

impl DbContext {
	fn new0() -> Self {
		DbContext {
			items: Arc::new(RwLock::new(Vec::new())), 
		}
	}
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item
{
   pub id: i32,
   pub name: String,
   pub quantity: i32,
   pub value: f64,
}

impl Item {
	fn new0() -> Self {
		Item {
			id: 0, 
			name: String::from(""), 
			quantity: 0, 
			value: 0.0, 
		}
	}
}

async fn get (_context: DbContext) -> Result<impl warp::Reply, warp::Rejection> {
	let mut result = Vec::new();

                        let r = _context.items.read();
                        for value in r.iter() {
                        result.push(value);
                        }

	Ok(warp::reply::json(&result))
}


#[tokio::main]
async fn main() {
	let _context = DbContext::new0();
	let _context_dbcontext = warp::any().map(move || _context.clone());

	let get = warp::get()
	.and(warp::path("v1"))
	.and(warp::path("groceries"))
	.and(warp::path("get"))
	.and(warp::path::end())
	.and(_context_dbcontext.clone())
	.and_then(get);

	let routes = get;

	warp::serve(routes)
		.run(([127, 0, 0, 1], 3030))
		.await;
}