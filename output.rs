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
