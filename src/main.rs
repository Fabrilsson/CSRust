extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CSParser;

struct Code {
    usings: Vec<String>,
    types_and_structs: Vec<String>,
    methods: Vec<String>
}

impl Code {
    fn new() -> Self {
        Code {
            usings: Vec::new(),
            types_and_structs: Vec::new(),
            methods: Vec::new()
        }
    }

    fn add_using(&mut self, value: String){
        self.usings.push(value)
    }

    fn add_type_or_struct(&mut self, value: String){
        self.types_and_structs.push(value)
    }

    fn add_method(&mut self, value: String){
        self.methods.push(value)
    }
}

#[derive(PartialEq)]
pub enum Step {
    models,
    repositories,
    controllers
}

fn main() {

    let mut code = Code::new();

    parse_models_contents(&mut code, &Step::models);

    write_all_to_file(&mut code);

    //parse_controller_contents();

}

fn parse_models_contents(code: &mut Code, step: &Step) {

    let controller_contents = fs::read_to_string("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Models/Item.cs")
    .expect("Something went wrong reading the file");

    let text = controller_contents.replace("\u{feff}", "");

    println!("With text:\n{}", controller_contents);

    let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        match_pairs(pair, code, step);
    }
}

fn parse_controller_contents(code: &mut Code, step: &Step) {

    let controller_contents = fs::read_to_string("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Controllers/GroceriesController.cs")
    .expect("Something went wrong reading the file");

    let text = controller_contents.replace("\u{feff}", "");

    println!("With text:\n{}", controller_contents);

    let successful_parse = CSParser::parse(Rule::parse_controller_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        match_pairs(pair, code, step);
    }
}

fn match_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::namespace_code_block => match_namespace_code_block(elem, code, step),
            Rule::using_code_block => match_using_code_block(elem, step),
            _ => unreachable!()
        };
    }
}

fn match_namespace_code_block(iter: Pair<Rule>, code: &mut Code, step: &Step) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::class_code => 
            {
                if *step == Step::models {
                    match_class_models_code_pairs(elem, code, step)
                }
                else {
                    match_class_code_pairs(elem, code, step)
                }
            },
            Rule::namespace_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
            Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        };
    }
}

fn match_using_code_block(iter: Pair<Rule>, step: &Step) {

    if *step == Step::models {
        return;
    }

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::using_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),            
            _ => unreachable!()
        };
    }
}

fn match_class_models_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step){

    if *step == Step::models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => match_constructor_pairs(elem),
                Rule::action => match_action_pairs(elem),
                Rule::properties => println!("teste2:  {}", elem.as_str()),
                Rule::attribute => println!("teste2:  {}", elem.as_str()),
                Rule::public_key_word => 
                {
                    code.add_type_or_struct(String::from("#[derive(Debug, Deserialize, Serialize, Clone)] \n"));
                    code.add_type_or_struct(String::from("pub "));
                },
                Rule::class_key_word => 
                {
                    code.add_type_or_struct(String::from("struct "));
                },
                Rule::identifier =>
                {
                    code.add_type_or_struct(String::from(elem.as_str()));
                },
                Rule::left_bracers => 
                {
                    code.add_type_or_struct(String::from("\n{"));
                },
                Rule::right_bracers => 
                {
                    code.add_type_or_struct(String::from("}\n"));
                },
                _ => unreachable!()
            }
        }
    }
}

fn match_class_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step) {
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor => match_constructor_pairs(elem),
            Rule::action => match_action_pairs(elem),
            Rule::properties => println!("teste2:  {}", elem.as_str()),
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

fn match_constructor_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor_parameters => match_parameters_pairs(elem),
            Rule::code => match_code_pairs(elem),
            Rule::public_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::left_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::right_parenthesis => println!("teste2:  {}", elem.as_str()),
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
            Rule::attribute => if elem.as_str() != "[HttpGet]" {return} else {println!("teste2:  {}", elem.as_str())},
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
            Rule::method_call => println!("teste2:  {}", elem.as_str()),
            Rule::async_method_call => println!("teste2:  {}", elem.as_str()),
            Rule::assignment => println!("teste2:  {}", elem.as_str()),
            Rule::return_key_word => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_parameters_pairs(iter: Pair<Rule>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameter => println!("teste2:  {}", elem.as_str()),
            Rule::action_parameter => println!("teste2:  {}", elem.as_str()),
            Rule::constructor_parameter => println!("teste2:  {}", elem.as_str()),
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

fn write_all_to_file(code: &mut Code) {
    let mut f = File::create("/home/fabrilsson/Documents/repo/CSRust/output.rs")
    .expect("Something went wrong reading the file");

    for elem in &code.usings {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &code.types_and_structs {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &code.methods {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }
}