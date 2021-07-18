extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "CS.pest"]
pub struct CSParser;

fn main() {
    let successful_parse = CSParser::parse(Rule::main, "public main () { if (asdasd > asdasd) }");
    println!("{:?}", successful_parse);
}
