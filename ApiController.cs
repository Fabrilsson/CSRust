using System;
using Microsoft.AspNetCore.Mvc;

namespace MemoryTestWebApi.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class ApiController : ControllerBase
    {
        [HttpGet("bigstring")]
        public ActionResult<string> Get(int id, string asdasd)
        {
            return new String("asdasd", 10 * 1024);
        }
    }
}
