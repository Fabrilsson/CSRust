namespace GroceriesApi.Models
{
    public class DbContext
    {
        public List<Item> Items { get; set; }        

        public DbContext()
        {
            Items = new List<Item>();
        }
    }
}