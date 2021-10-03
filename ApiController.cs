using System;
using Microsoft.AspNetCore.Mvc;

namespace MemoryTestWebApi.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class ApiController : ControllerBase
    {
        [HttpGet("bigstring")]
        public ActionResult<string> Get()
        {
            return new String('x', 10 * 1024);
        }
    }
}
