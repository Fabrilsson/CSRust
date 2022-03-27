extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;
use std::fs::File;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "CS.pest"]
pub struct CSParser;

fn main() {
    let contents = fs::read_to_string("ApiController.cs")
    .expect("Something went wrong reading the file");

    let text = contents.replace("\u{feff}", "");

    println!("With text:\n{}", contents);

    let successful_parse = CSParser::parse(Rule::main, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        match_pairs(pair);
    }
}

fn match_pairs(iter: Pair<Rule>) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::class_code => match_class_code_pairs(elem),
            Rule::namespace_code_block => match_pairs(elem),
            Rule::using_code_block => println!("Letter:  {}", elem.as_str()),
            Rule::namespace_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
            Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        };
    }
}

fn match_class_code_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action => match_action_pairs(elem),
            Rule::attribute => println!("teste2:  {}", elem.as_str()),
            Rule::public_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::class_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
            Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_action_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters => match_parameters_pairs(elem),
            Rule::code => match_code_pairs(elem),
            Rule::attribute => println!("teste2:  {}", elem.as_str()),
            Rule::public_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::return_type => println!("teste2:  {}", elem.as_str()),
            Rule::async_return_type => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::right_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
            Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_code_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem),
            Rule::return_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::right_parenthesis => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_parameters_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameter => println!("teste2:  {}", elem.as_str()),
            Rule::action_parameter => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_new_instance_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameters => match_parameters_pairs(elem),
            Rule::new_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::right_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::semicolon => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}