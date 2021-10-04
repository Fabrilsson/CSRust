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

    let mut asdasd = contents.replace("\u{feff}", "");

    asdasd = asdasd.replace("\n", "");
    asdasd = asdasd.replace("\r", "");

    println!("With text:\n{}", contents);

    let successful_parse = CSParser::parse(Rule::main, &asdasd).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    let mut count = 1;

    for pair in successful_parse {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::using_code_block => println!("Letter:  {}", inner_pair.as_str()),
                Rule::namespace_code_block => println!("teste:  {}", inner_pair.as_str()),
                _ => unreachable!()
            };

            if count == 1 || count == 2 {
                count = count + 1;

                continue;
            }

            for elem in inner_pair.into_inner() {
                match elem.as_rule() {
                    Rule::namespace_key_work => println!("teste2:  {}", elem.as_str()),
                    _ => unreachable!()
                };
            }
        }
    }
}
