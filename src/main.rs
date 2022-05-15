extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::{self, File};
use std::io::Write;
use pest::iterators::Pair;

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
struct Parameter {
    name: String,
    type_name: String
}

#[derive(Debug, Clone, PartialEq)]
struct Method {
    name: String,
    parameters: Vec<Parameter>
}

impl Method {
    fn new() -> Self {
        Method {
            name: String::from(""),
            parameters: Vec::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ClassMethod {
    class_name: String,
    methods: Vec<Method>
}

impl ClassMethod {
    fn new() -> Self {
        ClassMethod {
            class_name: String::from(""),
            methods: Vec::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ClassMethods {
    class_methods: Vec<ClassMethod>
}

impl ClassMethods {
    fn new() -> Self {
        ClassMethods {
            class_methods: Vec::new()
        }
    }

    fn add_class_method(&mut self, method: ClassMethod) {
        self.class_methods.push(method);
    }
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
    Models,
    Repositories,
    Controllers
}

fn main() {

    let mut code = Code::new();

    let mut type_table = TypeTable::new();

    let mut class_methods = ClassMethods::new();

    code.add_using(String::from("use warp::{http, Filter};\n"));
    code.add_using(String::from("use parking_lot::RwLock;\n"));
    code.add_using(String::from("use std::collections::HashMap;\n"));
    code.add_using(String::from("use std::sync::Arc;\n"));
    code.add_using(String::from("use serde::{Serialize, Deserialize};\n\n"));

    parse_models_contents(&mut code, &Step::Models, &mut type_table, &mut class_methods);

    println!("\n\n");

    parse_repository_contents(&mut code, &Step::Repositories, &mut type_table, &mut class_methods);

    println!("\n\n");

    parse_controller_contents(&mut code, &Step::Controllers, &mut type_table, &mut class_methods);
    
    write_all_to_file(&mut code, &mut type_table, &mut class_methods);
}

fn parse_models_contents(code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    let contents = read_files("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Models/").expect("Error");

    for models_contents in contents {

        let text = models_contents.replace("\u{feff}", "");

        let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

        println!("{:?}", successful_parse);

        for pair in successful_parse {
            match_pairs(pair, code, step, types, class_methods);
        }
    }
}

fn parse_repository_contents(code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    let models_contents = fs::read_to_string("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Repositories/GroceriesRepository.cs")
    .expect("Something went wrong reading the file");

    let text = models_contents.replace("\u{feff}", "");

    let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        match_pairs(pair, code, step, types, class_methods);
    }
}

fn parse_controller_contents(code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    let controller_contents = fs::read_to_string("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Controllers/GroceriesController.cs")
    .expect("Something went wrong reading the file");

    let text = controller_contents.replace("\u{feff}", "");

    let successful_parse = CSParser::parse(Rule::parse_controller_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        match_pairs(pair, code, step, types, class_methods);
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

fn match_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::namespace_code_block => match_namespace_code_block(elem, code, step, types, class_methods),
            Rule::using_code_block => match_using_code_block(elem),
            _ => unreachable!()
        };
    }
}

fn match_namespace_code_block(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::class_code => 
            {
                if *step == Step::Models {
                    match_class_models_code_pairs(elem, code, step, types, class_methods)
                }
                else if *step == Step::Repositories {
                    match_repositories_code_pairs(elem, code, step, types, class_methods)
                }
                else if *step == Step::Controllers {
                    match_controller_code_pairs(elem, code, step, types, class_methods)
                }
            },
            Rule::namespace_key_word => {},
            Rule::identifier => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        };
    }
}

fn match_using_code_block(iter: Pair<Rule>) {

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::using_key_word => {},
            Rule::identifier => {},
            Rule::semicolon => {},
            _ => unreachable!()
        };
    }
}

fn match_class_models_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    let mut methods: Vec<Method> = Vec::new();

    if *step == Step::Models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => match_constructor_pairs(elem, code, step, class_name, types),
                Rule::action => methods.push(match_action_pairs(elem, code, step, class_name, types, &mut properties)),
                Rule::properties => { 
                    let property = match_properties_pairs(elem, code, step, class_name);
                    properties.push(property);
                },
                Rule::attribute => {},
                Rule::public_key_word => 
                {
                    code.add_struct("\n#[derive(Debug, Deserialize, Serialize, Clone)] \n");
                    code.add_struct("pub ");
                },
                Rule::class_key_word => 
                {
                    code.add_struct("struct ");
                },
                Rule::class_name =>
                {
                    code.add_struct(elem.as_str().trim());
                    class_name = elem.as_str().trim();
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

        types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), rule: Rule::identifier, properties: properties });

        class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods })
    }
}

fn match_repositories_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    let mut methods: Vec<Method> = Vec::new();

    if *step == Step::Repositories {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::public_key_word => {},
                Rule::class_key_word => {},
                Rule::properties => { 
                    let property = match_properties_pairs(elem, code, step, class_name);
                    properties.push(property);
                },
                Rule::class_name =>
                {
                    if class_name.is_empty() {
                        class_name = elem.as_str();
                    }
                },
                Rule::left_bracers => {},
                Rule::right_bracers => {},
                Rule::action => methods.push(match_action_pairs(elem, code, step, class_name, types, &mut properties)),
                _ => unreachable!()
            }
        }
    }

    class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods })

}

fn match_controller_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    let mut methods: Vec<Method> = Vec::new();

    if *step == Step::Controllers {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::public_key_word => {},
                Rule::class_key_word => {},
                Rule::attribute => if elem.as_str() != "[HttpGet]" {return} else {{}},
                Rule::properties => { 
                    let property = match_properties_pairs(elem, code, step, class_name);
                    properties.push(property);
                },
                Rule::class_name =>
                {
                    if class_name.is_empty() {
                        class_name = elem.as_str();
                    }
                },
                Rule::left_bracers => {},
                Rule::right_bracers => {},
                Rule::action => methods.push(match_action_pairs(elem, code, step, class_name, types, &mut properties)),
                _ => { println!("\n\n{}", elem.as_str()); unreachable!(); }
            }
        }

        class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods })
    }
}

fn match_properties_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str) -> Type {

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
                if *step == Step::Models{
                    code.add_struct("   pub ");
                }
            },
            Rule::private_key_word => 
            {
                if *step == Step::Models{
                    code.add_struct("   pub ");
                }
            },
            Rule::static_key_word => {},
            Rule::readonly_key_word => {},
            Rule::assignment => return match_assignment_code_pairs(elem, code, step, &property_type.pop().expect(""), class_name, &mut rust_prop_type),
            Rule::property_type => { rust_prop_type = match_property_type_code_pairs(elem, code, step, &mut property_type) },
            Rule::identifier => 
            {
                let prop_type = property_type.pop().expect("");

                if create_hashmap {
                    code.add_type(&format!("type {}s = Vec<{}>;\n", class_name, class_name));
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

    let mut parameters: Vec<Parameter> = Vec::new();

    let mut method_name = "";

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor_parameters => match_parameters_pairs(elem, method_name,  &mut parameters),
            Rule::code => match_code_pairs(elem, code, step, class_name, types),
            Rule::public_key_word => {},
            Rule::identifier => { method_name = elem.as_str() },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }
}

fn match_action_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable, properties: &mut Vec<Type>, ) -> Method{

    let class_names: Vec<&str> = class_name.split(":").collect();

    let mut method_name = "";

    for elem in class_names.clone().into_iter() {
        types.add_type(Type { name: String::from(elem).replace(" ", ""), type_name: String::from(elem).replace(" ", ""), rule: Rule::identifier, properties: properties.to_vec() });
    }

    let mut parameters: Vec<Parameter> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters => match_parameters_pairs(elem, method_name, &mut parameters),
            Rule::code => match_code_pairs(elem, code, step, class_names[0].trim(), types),
            Rule::attribute => if elem.as_str() != "[HttpGet]" { return Method::new() } else {{}},
            Rule::public_key_word => {},
            Rule::action_return_type => {},
            Rule::action_async_return_type => {},
            Rule::method_return_type => {},
            Rule::identifier => {
                if *step == Step::Repositories {

                    if !elem.as_str().contains("Get"){
                        return Method::new();
                    }

                    method_name = elem.as_str();
                    
                    code.add_method(String::from(format!("async fn {} ", elem.as_str().to_lowercase())));
                }
            },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }

    Method { name: String::from(method_name), parameters: parameters.clone() }

}

fn match_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str, types: &mut TypeTable){
    
    let mut is_return_type = false;
    
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem),
            Rule::method_call => {},
            Rule::async_method_call => {},
            Rule::assignment => {},
            Rule::return_key_word => {is_return_type = true;},
            Rule::property_call => {

                let properties: Vec<&str> = elem.as_str().split(".").collect();

                let mut type_table = types.get_type_table(String::from(class_name));

                let property = type_table.get_type_property(String::from(properties[0]));

                let mut type_table_store = types.get_type_table(String::from(&property.type_name));

                let nested_property = type_table_store.get_type_property(String::from(properties[1]));

                code.add_method(String::from(format!("({}: {}) -> Result<impl warp::Reply, warp::Rejection>", property.name, property.type_name)));

                let nested_property_is_list = is_list_type(nested_property.type_name.to_lowercase());

                if nested_property_is_list && is_return_type {
                code.add_method(String::from(format!(" {{\n    let mut result = Vec::new();\n
    let r = {}.{}.read();
    for value in r.iter() {{
        result.push(value);
    }}\n\n", property.name, nested_property.name.to_lowercase())));
                }

                if is_return_type {
                    code.add_method(String::from("\tOk(warp::reply::json(&result))\n}\n"))
                }
            },
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn is_list_type(value: String) -> bool {
    value.contains("list") ||
    value.contains("ienumerable")
}

fn match_parameters_pairs(iter: Pair<Rule>, method_name: &str, parameters: &mut Vec<Parameter>){
    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameter => {},
            Rule::action_parameter => {
                let split_parameter: Vec<&str> = elem.as_str().split(" ").collect();

                parameters.push(Parameter { name: String::from(split_parameter[1]), 
                    type_name: String::from(split_parameter[0]) });
            },
            Rule::constructor_parameter => {},
            _ => unreachable!()
        }
    }
}

fn match_new_instance_pairs(iter: Pair<Rule>){

    let mut parameters: Vec<Parameter> = Vec::new();

    let mut method_name = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameters => match_parameters_pairs(elem, &method_name, &mut parameters),
            Rule::new_key_word => {},
            Rule::identifier => { method_name = String::from(elem.as_str()) },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::semicolon => {},
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
            Rule::number => {},
            Rule::semicolon => {},
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

fn write_all_to_file(code: &mut Code, types: &mut TypeTable, class_methods: &mut ClassMethods) {
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

    println!("\n\n");

    for elem in &types.types {
        println!("{:?}\n", elem);
    }

    println!("\n\n");

    for elem in &class_methods.class_methods {
        println!("{:?}\n", elem);
    }
}