extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::{self, File};
use std::io::Write;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CSParser;

struct Code {
    usings: Vec<String>,
    types: Vec<String>,
    structs: Vec<String>,
    methods: Vec<String>,
    methods_parameters: Vec<String>
}

struct TypeTable {
    types: Vec<Type>
}

#[derive(Debug, Clone, PartialEq)]
struct Type {
    name: String,
    type_name: String,
    rule: Rule,
    properties: Vec<Type>
}

impl TypeTable {
    fn new() -> Self {
        TypeTable {
            types: Vec::new()
        }
    }

    fn add_type(&mut self, t: Type) {
        if !self.types.contains(&t) {
            self.types.push(t);
        }
    }

    fn get_type_table(&mut self, name: String) -> Type {
        self.types.clone().into_iter().find(|a| a.name == name).expect("")
    }
}

impl Type {
    fn new() -> Self {
        Type {
            name: String::from(""),
            type_name: String::from(""),
            rule: Rule::identifier,
            properties: Vec::new()
        }
    }

    fn get_type_property(&mut self, name: String) -> Type {
        self.properties.clone().into_iter().find(|a| a.name == name).expect("")
    }
}

impl Code {
    fn new() -> Self {
        Code {
            usings: Vec::new(),
            types: Vec::new(),
            structs: Vec::new(),
            methods: Vec::new(),
            methods_parameters: Vec::new()
        }
    }

    fn add_using(&mut self, value: String){
        self.usings.push(value);
    }

    fn add_type(&mut self, value: &str){
        self.types.push(String::from(value));
    }

    fn add_struct(&mut self, value: &str){
        self.structs.push(String::from(value));
    }

    fn add_method(&mut self, value: String){
        self.methods.push(value);
    }

    fn add_method_parameter(&mut self, value: String){
        self.methods_parameters.push(value);
    }
}

#[derive(PartialEq, Debug)]
pub enum Step {
    models,
    repositories,
    controllers
}

fn main() {

    let mut code = Code::new();

    let mut type_table = TypeTable::new();

    // type_table.add_type(Type {name: String::from("teste"), rule: Rule::assignment, properties: Vec::new()});

    // let asdasd: Vec<Type> = type_table.types.into_iter().filter(|i| i.name == "teste").collect();

    // println!("asdasdasdad {}", asdasd.first().expect("").name);

    code.add_using(String::from("use warp::{http, Filter};\n"));
    code.add_using(String::from("use parking_lot::RwLock;\n"));
    code.add_using(String::from("use std::collections::HashMap;\n"));
    code.add_using(String::from("use std::sync::Arc;\n"));
    code.add_using(String::from("use serde::{Serialize, Deserialize};\n\n"));

    parse_models_contents(&mut code, &Step::models, &mut type_table);

    parse_repository_contents(&mut code, &Step::repositories, &mut type_table);

    write_all_to_file(&mut code, &mut type_table);

    //parse_controller_contents();

}

fn parse_models_contents(code: &mut Code, step: &Step, types: &mut TypeTable) {

    let contents = read_files("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Models/").expect("Error");

    for models_contents in contents {

        let text = models_contents.replace("\u{feff}", "");

        println!("With text:\n{}", models_contents);

        let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

        println!("{:?}", successful_parse);

        for pair in successful_parse {
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());

            match_pairs(pair, code, step, types);
        }
    }
}

fn parse_repository_contents(code: &mut Code, step: &Step, types: &mut TypeTable) {

    let models_contents = fs::read_to_string("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Repositories/GroceriesRepository.cs")
    .expect("Something went wrong reading the file");

    let text = models_contents.replace("\u{feff}", "");

    println!("With text:\n{}", models_contents);

    let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        match_pairs(pair, code, step, types);
    }
}

fn parse_controller_contents(code: &mut Code, step: &Step, types: &mut TypeTable) {

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

        match_pairs(pair, code, step, types);
    }
}

fn read_files(path: &str) -> std::io::Result<Vec<String>> {
    let mut files_contents: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        let content = fs::read_to_string(path).expect("Error");

        if !content.contains("interface")
        {
            files_contents.push(content);
        }
    }

    Ok(files_contents)
}

fn match_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::namespace_code_block => match_namespace_code_block(elem, code, step, types),
            Rule::using_code_block => match_using_code_block(elem),
            _ => unreachable!()
        };
    }
}

fn match_namespace_code_block(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::class_code => 
            {
                if *step == Step::models {
                    match_class_models_code_pairs(elem, code, step, types)
                }
                else if *step == Step::repositories {
                    match_repositories_code_pairs(elem, code, step, types)
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

fn match_using_code_block(iter: Pair<Rule>) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::using_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::identifier => println!("teste2:  {}", elem.as_str()),
            Rule::semicolon => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        };
    }
}

fn match_class_models_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    if *step == Step::models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => match_constructor_pairs(elem, code, step, class_name, types),
                Rule::action => match_action_pairs(elem, code, step, class_name, types, &mut properties),
                Rule::properties => { 
                    let property = match_properties_pairs(elem, code, step, class_name, types);
                    properties.push(property);
                },
                Rule::attribute => println!("teste2:  {}", elem.as_str()),
                Rule::public_key_word => 
                {
                    code.add_struct("\n#[derive(Debug, Deserialize, Serialize, Clone)] \n");
                    code.add_struct("pub ");
                },
                Rule::class_key_word => 
                {
                    code.add_struct("struct ");
                },
                Rule::identifier =>
                {
                    code.add_struct(elem.as_str());
                    class_name = elem.as_str();
                },
                Rule::left_bracers => 
                {
                    code.add_struct("\n{\n");
                },
                Rule::right_bracers => 
                {
                    code.add_struct("}\n");
                },
                _ => unreachable!()
            }
        }

        types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), rule: Rule::identifier, properties: properties })
    }
}

fn match_repositories_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    if *step == Step::repositories {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::public_key_word => println!("teste2:  {}", elem.as_str()),
                Rule::class_key_word => println!("teste2:  {}", elem.as_str()),
                Rule::properties => { 
                    let property = match_properties_pairs(elem, code, step, class_name, types);
                    properties.push(property);
                },
                Rule::identifier =>
                {
                    if class_name.is_empty() {
                        class_name = elem.as_str();
                    }
                },
                Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
                Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
                Rule::action => match_action_pairs(elem, code, step, class_name, types, &mut properties),
                _ => unreachable!()
            }
        }
    }
}

fn match_properties_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable) -> Type {

    let mut property_type: Vec<String> = Vec::new();

    let mut rust_prop_type = String::from("");

    let mut create_hashmap: bool = false;

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::attribute => 
            {
                if elem.as_str() == "[Key]"{
                    create_hashmap = true;
                }
            },
            Rule::public_key_word => 
            {
                if *step == Step::models{
                    code.add_struct("   pub ");
                }
            },
            Rule::private_key_word => 
            {
                if *step == Step::models{
                    code.add_struct("   pub ");
                }
            },
            Rule::static_key_word => {},
            Rule::readonly_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::assignment => return match_assignment_code_pairs(elem, code, step, &property_type.pop().expect(""), class_name, &mut rust_prop_type),
            Rule::property_type => { rust_prop_type = match_property_type_code_pairs(elem, code, step, &mut property_type) },
            Rule::identifier => 
            {
                let prop_type = property_type.pop().expect("");

                //let rust_prop_type = get_equivalent_rust_type(&prop_type);

                if create_hashmap {
                    code.add_type(&format!("type {}s = HaspMap<{}, {}>;\n", class_name, rust_prop_type, class_name));
                }

                code.add_struct(&format!("{}: {},\n", elem.as_str().to_lowercase(), rust_prop_type));

                return Type { name: String::from(elem.as_str()), type_name: prop_type, rule: Rule::identifier, properties: Vec::new() };
            },
            Rule::left_bracers => {}
            Rule::get_key_word => {}
            Rule::semicolon => {}
            Rule::set_key_word => {}
            Rule::right_bracers => {}
            _ => unreachable!()
        }
    }

    return Type::new();
}

fn match_constructor_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor_parameters => match_parameters_pairs(elem),
            Rule::code => match_code_pairs(elem, code, step, class_name, types),
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

fn match_action_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable, properties: &mut Vec<Type>){

    let mut return_type = "";

    types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), rule: Rule::identifier, properties: properties.to_vec() });

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters => match_parameters_pairs(elem),
            Rule::code => match_code_pairs(elem, code, step, class_name, types),
            Rule::attribute => if elem.as_str() != "[HttpGet]" {return} else {println!("teste2:  {}", elem.as_str())},
            Rule::public_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::action_return_type => println!("teste2:  {}", elem.as_str()),
            Rule::action_async_return_type => println!("teste2:  {}", elem.as_str()),
            Rule::method_return_type => {if *step == Step::repositories { return_type = "Result<impl warp::Reply, warp::Rejection>" }},
            Rule::identifier => {
                if *step == Step::repositories{

                    if !elem.as_str().contains("Get"){
                        return;
                    }

                    code.add_method(String::from(format!("async fn {} ", elem.as_str().to_lowercase())));
                }
            },
            Rule::left_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::right_parenthesis => println!("teste2:  {}", elem.as_str()),
            Rule::left_bracers => println!("teste2:  {}", elem.as_str()),
            Rule::right_bracers => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }
}

fn match_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem),
            Rule::method_call => println!("teste2:  {}", elem.as_str()),
            Rule::async_method_call => println!("teste2:  {}", elem.as_str()),
            Rule::assignment => println!("teste2:  {}", elem.as_str()),
            Rule::return_key_word => println!("teste2:  {}", elem.as_str()),
            Rule::property_call => {

                let properties: Vec<&str> = elem.as_str().split(".").collect();

                let mut type_table = types.get_type_table(String::from(class_name));

                let property = type_table.get_type_property(String::from(properties[0]));

                let mut type_table_store = types.get_type_table(String::from(&property.type_name));

                let nested_property = type_table_store.get_type_property(String::from(properties[1]));

                code.add_method(String::from(format!("({}: {}) -> Result<impl warp::Reply, warp::Rejection>", property.name, property.type_name)));

                code.add_method(String::from(format!(" {{\n    let mut result = HashMap::new();\n
    let r = {}.{}.read();
    for (key,value) in r.iter() {{
        result.insert(key, value);
    }}

    Ok(warp::reply::json(&result))
}}\n", property.name, nested_property.name.to_lowercase())))
            },
            Rule::semicolon => println!("teste2:  {}", elem.as_str()),
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

fn match_assignment_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, property_type: &String, class_name: &str, rust_prop_type: &mut String) -> Type {

    let mut prop_name = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::property_call => {

                if rust_prop_type != "<not_found>" && rust_prop_type != "i32" {
                    code.add_struct(&format!("\nimpl {} {} \n", rust_prop_type, "{"));
                    code.add_method_parameter(String::from(&format!("{}: {}", rust_prop_type.to_lowercase(), rust_prop_type)));
                }

                if property_type.is_empty() {
                    continue;
                }

                prop_name = String::from(elem.as_str());
            },
            Rule::new_instance => {
                code.add_struct(&format!("\tfn new() -> Self {}\n\t\t {} {} \n\t\t\titems: Arc::new(RwLock::new(HashMap::new()))\n\t\t{}\n\t{}\n{}\n\n", "{", property_type, "{", "}", "}", "}"))
            },
            Rule::number => println!("teste2:  {}", elem.as_str()),
            Rule::semicolon => println!("teste2:  {}", elem.as_str()),
            _ => unreachable!()
        }
    }

    return Type { name: prop_name, type_name: String::from(property_type), rule: Rule::identifier, properties: Vec::new() };
}

fn match_property_type_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, test: &mut Vec<String>) -> String {

    let mut propety_type = "";

    for elem in iter.into_inner() {
        test.push(String::from(elem.as_str()));
        match elem.as_rule(){
            Rule::string_key_word => { propety_type = "String" },
            Rule::int_key_word => { propety_type = "i32" },
            Rule::decimal_key_word => { propety_type = "f64" },
            Rule::list_type => { propety_type = "Arc<RwLock<Items>>" },
            Rule::identifier => { propety_type = elem.as_str() },
            _ => { propety_type = "<not_found>" }
        }
    }

    return String::from(propety_type);
}

fn write_all_to_file(code: &mut Code, types: &mut TypeTable) {
    let mut f = File::create("/home/fabrilsson/Documents/repo/CSRust/output.rs")
    .expect("Something went wrong reading the file");

    for elem in &code.usings {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &code.types {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &code.structs {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &code.methods {
        f.write_all(elem.as_bytes()).expect("Something went wrong reading the file");
    }

    for elem in &types.types {
        println!("{:?}", elem);
    }
}