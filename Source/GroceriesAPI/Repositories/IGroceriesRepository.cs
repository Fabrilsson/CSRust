using GroceriesApi.Models;

namespace GroceriesApi.Repositories
{
    public interface IGroceriesRepository
    {
        IEnumerable<Item> GetItemsAsync();

        void AddItem(Item item);

        void UpdateItem(Item item);

        void Delete(int Id);
    }
}