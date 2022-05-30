using GroceriesApi.Models;

namespace GroceriesApi.Repositories
{
    public class GroceriesRepository : IGroceriesRepository
    {
        private static readonly DbContext _context = new DbContext();

        private static int _identifier = 0;

        public GroceriesRepository()
        {
            
        }

        public IEnumerable<Item> GetItemsAsync()
        {
            return _context.Items;
        }

        public void AddItem(Item item)
        {
            item.Id = _identifier;

            _identifier = _identifier + 1;

            _context.Items.Add(item);
        }

        public void UpdateItem(Item item)
        {
            _context.Items.RemoveAll(i => i.Id == item.Id);

            _context.Items.Add(item);
        }

        public void Delete(int id)
        {
            _context.Items.RemoveAll(i => i.Id == id);
        }
    }
}