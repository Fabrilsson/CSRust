using Microsoft.AspNetCore.Mvc;
using GroceriesApi.Models;
using GroceriesApi.Repositories;

namespace GroceriesApi.Controllers{

    [ApiController]
    [Route("v1/[controller]")]
    public class GroceriesController : ControllerBase
    {
        private readonly IGroceriesRepository _repository;

        public GroceriesController(IGroceriesRepository repository)
        {
            _repository = repository;
        }

        [HttpGet]
        public IActionResult Get()
        {
            var items = _repository.GetItemsAsync();

            return new OkObjectResult(items);
        }

        [HttpPut]
        public IActionResult Put(Item item)
        {
            _repository.UpdateItem(item);

            return new OkResult();
        }

        [HttpPost]
        public IActionResult Post(Item item)
        {
            _repository.AddItem(item);

            return new OkResult();
        }

        [HttpDelete]
        public IActionResult Delete(int Id)
        {
            _repository.Delete(Id);

            return new OkResult();
        }
    }
}