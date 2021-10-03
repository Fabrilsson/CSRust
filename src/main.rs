extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "CS.pest"]
pub struct CSParser;

fn main() {
    let contents = fs::read_to_string("ApiController.cs")
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let successful_parse = CSParser::parse(Rule::main, "using System; using Microsoft.AspNetCore.Mvc; namespace MemoryTestWebApi.Controllers { [TesteAttribute] [TesteAttribute2(\"asdasd\")] public main () { if (asdasd > asdasd) } }");
    println!("{:?}", successful_parse);
}
