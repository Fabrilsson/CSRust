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
    return_type: String,
    parameters: Vec<Parameter>
}

#[derive(Debug, Clone, PartialEq)]
struct ClassMethod {
    class_name: String,
    methods: Vec<Method>
}

impl ClassMethod {
    fn get_method(&mut self, name: &str) -> Method {
        self.methods.clone().into_iter().find(|a| a.name == name).expect(&format!("Method {} not found", name))
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

    fn get_class_method(&mut self, class_name: &str) -> ClassMethod {
        self.class_methods.clone().into_iter().find(|a| a.class_name.contains(class_name)).expect(&format!("Class method {} not found", class_name))
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
        self.types.clone().into_iter().find(|a| a.name.contains(&name)).expect(&format!("Type table {} not found", name))
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
        self.properties.clone().into_iter().find(|a| a.name == name).expect(&format!("Property {} not found", name))
    }

    fn get_type_properties(&mut self) -> Vec<Type> {
        self.properties.clone()
    }
}

impl Code {
    fn new() -> Self {
        Code {
            usings: Vec::new(),
            types: Vec::new(),
            structs: Vec::new(),
            methods: Vec::new(),
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
    code.add_using(String::from("use serde::{Serialize, Deserialize};\n"));
    code.add_using(String::from("use parking_lot::RwLock;\n"));
    code.add_using(String::from("use std::sync::Arc;\n"));
    
    parse_models_contents(&mut code, &Step::Models, &mut type_table, &mut class_methods);

    println!("\n\n");

    parse_repository_contents(&mut code, &Step::Repositories, &mut type_table, &mut class_methods);

    println!("\n\n");

    parse_controller_contents(&mut code, &Step::Controllers, &mut type_table, &mut class_methods);
    
    write_all_to_file(&mut code, &mut type_table, &mut class_methods);
}

fn parse_models_contents(code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    let path = "Source/GroceriesAPI/Models/";

    let contents = read_files(path).expect(&format!("Something went wrong reading the file {}", path));

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

    let path = "Source/GroceriesAPI/Repositories/GroceriesRepository.cs";

    let models_contents = fs::read_to_string(path)
    .expect(&format!("Something went wrong reading the file {}", path));

    let text = models_contents.replace("\u{feff}", "");

    let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", successful_parse);

    for pair in successful_parse {
        match_pairs(pair, code, step, types, class_methods);
    }
}

fn parse_controller_contents(code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods) {

    let path = "Source/GroceriesAPI/Controllers/GroceriesController.cs";

    let controller_contents = fs::read_to_string(path)
    .expect(&format!("Something went wrong reading the file {}", path));

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
                    match_repositories_class_code_pairs(elem, code, step, types, class_methods)
                }
                else if *step == Step::Controllers {
                    match_controller_class_code_pairs(elem, code, step, types, class_methods)
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

    let mut is_added_impl = false;

    if *step == Step::Models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => { 
                    if !is_added_impl {
                        code.add_struct("}\n");
                        let split_class_name: Vec<&str> = class_name.split(".").collect();
                        code.add_struct(&format!("\nimpl {} {{", split_class_name[0]));

                        is_added_impl = true;
                    }

                    match_models_constructor_pairs(elem, code, class_name, types, &mut properties, class_methods);
                },
                Rule::action => methods.push(match_models_action_pairs(elem, code, class_name, types, class_methods)),
                Rule::properties => { 
                    let property = match_models_properties_pairs(elem, code, step, class_name);
                    properties.push(property);
                },
                Rule::attribute => {},
                Rule::public_key_word => 
                {
                    code.add_struct("\n#[derive(Debug, Deserialize, Serialize, Clone)]");
                    code.add_struct("\npub ");
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
                Rule::right_bracers => {code.add_struct("}\n\n");},
                _ => unreachable!()
            }
        }

        types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), rule: Rule::identifier, properties: properties });

        class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods })
    }
}

fn match_repositories_class_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods){

    let mut class_name: &str = "";

    let mut properties: Vec<Type> = Vec::new();

    let mut methods: Vec<Method> = Vec::new();

    if *step == Step::Repositories {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::public_key_word => {},
                Rule::class_key_word => {},
                Rule::constructor => { 
                    types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), 
                    rule: Rule::identifier, properties: properties.to_vec() }); 
                },
                Rule::properties => { 
                    let property = match_repositories_properties_pairs(elem, code);
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
                Rule::action => methods.push(match_repositories_action_pairs(elem, code, class_name, types, class_methods)),
                _ => unreachable!()
            }
        }
    }

    class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods })

}

fn match_controller_class_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, types: &mut TypeTable, class_methods: &mut ClassMethods){

    let mut class_name: &str = "";    

    let mut properties: Vec<Type> = Vec::new();

    let mut methods: Vec<Method> = Vec::new();

    let mut constructor_parameters: Vec<Parameter> = Vec::new();

    let mut routes: Vec<String> = Vec::new();

    if *step == Step::Controllers {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::public_key_word => {},
                Rule::class_key_word => {},
                Rule::attribute => {},
                Rule::constructor => constructor_parameters = match_controller_constructor_pairs(elem, class_name, types, &mut properties),
                Rule::properties => { 
                    let property = match_controller_properties_pairs(elem, code);
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
                Rule::action => methods.push(match_controller_action_pairs(elem, code, class_name, types, class_methods)),
                _ => unreachable!()
            }
        }

        class_methods.add_class_method(ClassMethod { class_name: String::from(class_name), methods: methods.clone() });

        code.add_method(String::from("\n#[tokio::main]\n"));
        code.add_method(String::from("async fn main() {"));

        for elem1 in constructor_parameters.clone().into_iter() {
            let mut a = types.get_type_table(String::from(elem1.type_name.trim()));

            for elem2 in a.get_type_properties() {
                if elem2.rule == Rule::identifier {
                    code.add_method(String::from(format!("\n\tlet {} = {}::new0();", elem2.name, elem2.type_name)));
                    code.add_method(String::from(format!("\n\tlet {}_{} = warp::any().map(move || {}.clone());", elem2.name, elem2.type_name.to_lowercase(), elem2.name)));
                }
            }
        }

        for elem in methods.clone().into_iter() {
            if elem.name.to_lowercase().contains("get") {
                routes.push(elem.name.to_lowercase());

                code.add_method(String::from(format!("\n\n\tlet {} = warp::get()", elem.name.to_lowercase())));
                code.add_method(String::from(format!("\n\t.and(warp::path(\"v1\"))")));
                code.add_method(String::from(format!("\n\t.and(warp::path(\"groceries\"))")));
                code.add_method(String::from(format!("\n\t.and(warp::path(\"{}\"))", elem.name.to_lowercase())));
                code.add_method(String::from(format!("\n\t.and(warp::path::end())")));

                for elem1 in constructor_parameters.clone().into_iter() {
                    let mut a = types.get_type_table(String::from(elem1.type_name.trim()));
        
                    for elem2 in a.get_type_properties() {
                        if elem2.rule == Rule::identifier {
                            code.add_method(String::from(format!("\n\t.and({}_{}.clone())", elem2.name, elem2.type_name.to_lowercase())));
                        }
                    }
                }

                code.add_method(String::from(format!("\n\t.and_then({});", elem.name.to_lowercase())));
            }
        }

        let mut joined_routes: Vec<String> = Vec::new();

        for elem in routes.clone().into_iter() {
            if joined_routes.len() == 0 {
                joined_routes.push(elem.to_lowercase());
            }
            else {
                joined_routes.push(format!("or({})", elem.to_lowercase()));
            }
        }

        code.add_method(String::from(format!("\n\n\tlet routes = {};", joined_routes.join("."))));

        code.add_method(String::from(format!("\n\n\twarp::serve(routes)")));
        code.add_method(String::from(format!("\n\t\t.run(([127, 0, 0, 1], 3030))")));
        code.add_method(String::from(format!("\n\t\t.await;")));
        code.add_method(String::from(format!("\n}}")));
    }
}

fn match_controller_properties_pairs(iter: Pair<Rule>, code: &mut Code) -> Type {

    let mut property_type: Vec<Type> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::attribute => {},
            Rule::public_key_word => {},
            Rule::private_key_word => {},
            Rule::static_key_word => {},
            Rule::readonly_key_word => {},
            Rule::assignment => return match_assignment_properties_pairs(elem, &property_type.pop().expect("")),
            Rule::property_type => { match_property_type_code_pairs(elem, code, &mut property_type); },
            Rule::identifier => 
            {
                let prop_type = property_type.pop().expect("");
                return Type { name: String::from(elem.as_str()), type_name: prop_type.type_name, rule: Rule::identifier, properties: Vec::new() };
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

fn match_models_properties_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str) -> Type {

    let mut property_type: Vec<Type> = Vec::new();

    let mut rust_prop_type = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::attribute => {},
            Rule::public_key_word => 
            {
                code.add_struct("   pub ");
            },
            Rule::private_key_word => 
            {
                code.add_struct("   pub ");
            },
            Rule::static_key_word => {},
            Rule::readonly_key_word => {},
            Rule::assignment => return match_assignment_properties_pairs(elem, &property_type.pop().expect("")),
            Rule::property_type => { rust_prop_type = match_property_type_code_pairs(elem, code, &mut property_type) },
            Rule::identifier => 
            {
                let prop_type = property_type.pop().expect("");

                code.add_struct(&format!("{}: {},\n", elem.as_str().to_lowercase(), rust_prop_type));

                return Type { name: String::from(elem.as_str()), type_name: prop_type.type_name, rule: Rule::identifier, properties: Vec::new() };
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

fn match_repositories_properties_pairs(iter: Pair<Rule>, code: &mut Code) -> Type {

    let mut property_type: Vec<Type> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::attribute => {},
            Rule::public_key_word => {},
            Rule::private_key_word => {},
            Rule::static_key_word => {},
            Rule::readonly_key_word => {},
            Rule::assignment => return match_assignment_properties_pairs(elem, &property_type.pop().expect("")),
            Rule::property_type => { match_property_type_code_pairs(elem, code, &mut property_type); },
            Rule::identifier => 
            {
                let prop_type = property_type.pop().expect("");

                return Type { name: String::from(elem.as_str()), type_name: prop_type.type_name, rule: Rule::identifier, properties: Vec::new() };
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

fn match_controller_constructor_pairs(iter: Pair<Rule>, class_name: &str, types: &mut TypeTable, properties: &mut Vec<Type>) -> Vec<Parameter>{

    let mut constructor_parameters: Vec<Parameter> = Vec::new();

    types.add_type(Type { name: String::from(class_name), type_name: String::from(class_name), rule: Rule::identifier, properties: properties.to_vec() });

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor_parameters => { constructor_parameters = match_parameters_pairs(elem); },
            Rule::code => {},
            Rule::public_key_word => {},
            Rule::identifier => { },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }

    constructor_parameters
}

fn match_models_constructor_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, 
    properties: &mut Vec<Type>, class_methods: &mut ClassMethods){

    let mut parameters: Vec<Parameter> = Vec::new();

    types.add_type(Type { name: String::from(class_name).replace(" ", ""), type_name: String::from(class_name).replace(" ", ""), 
        rule: Rule::identifier, properties: properties.to_vec() });

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::constructor_parameters => { parameters = match_parameters_pairs(elem); },
            Rule::code => match_models_constructor_code_pairs(elem, code, class_name, types, class_methods, &mut parameters),
            Rule::public_key_word => {},
            Rule::identifier => {},
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }
}

fn match_controller_action_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods) -> Method{
    let mut method_name = "";

    let mut method_return_type = "";

    let mut parameters: Vec<Parameter> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters => { parameters = match_parameters_pairs(elem); },
            Rule::code => { 
                if method_name.to_lowercase().contains("get") {
                    match_controller_code_pairs(elem, code, class_name, method_return_type, types, class_methods)
                }
            },
            Rule::attribute => if elem.as_str() != "[HttpGet]" {} else {{}},
            Rule::public_key_word => {},
            Rule::action_return_type => method_return_type = elem.as_str(),
            Rule::action_async_return_type => method_return_type = elem.as_str(),
            Rule::method_return_type => method_return_type = elem.as_str(),
            Rule::identifier => {
                method_name = elem.as_str();
                
                if method_name.to_lowercase().contains("get") {
                    code.add_method(String::from(format!("async fn {} ", method_name.to_lowercase())));
                }
            },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }

    Method { name: String::from(method_name), return_type: String::from(method_return_type), parameters: parameters.clone() }

}

fn match_models_action_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods) -> Method{

    let mut method_name = "";

    let mut method_return_type = "";

    let mut parameters: Vec<Parameter> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters => { parameters = match_parameters_pairs(elem); },
            Rule::code => match_models_code_pairs(elem, code, class_name, types, class_methods),
            Rule::attribute => if elem.as_str() != "[HttpGet]" {} else {{}},
            Rule::public_key_word => {},
            Rule::action_return_type => method_return_type = elem.as_str(),
            Rule::action_async_return_type => method_return_type = elem.as_str(),
            Rule::method_return_type => method_return_type = elem.as_str(),
            Rule::identifier => {
                method_name = elem.as_str();
            },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }

    Method { name: String::from(method_name), return_type: String::from(method_return_type), parameters: parameters.clone() }

}

fn match_repositories_action_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods) -> Method{

    let mut method_name = "";

    let mut method_return_type = "";

    let mut parameters: Vec<Parameter> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::action_parameters =>{parameters = match_parameters_pairs(elem); },
            Rule::code => match_repositories_code_pairs(elem, code, class_name, types, class_methods),
            Rule::attribute => if elem.as_str() != "[HttpGet]" {} else {{}},
            Rule::public_key_word => {},
            Rule::action_return_type => method_return_type = elem.as_str(),
            Rule::action_async_return_type => method_return_type = elem.as_str(),
            Rule::method_return_type => method_return_type = elem.as_str(),
            Rule::identifier => {
                method_name = elem.as_str();
            },
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::left_bracers => {},
            Rule::right_bracers => {},
            _ => unreachable!()
        }
    }

    Method { name: String::from(method_name), return_type: String::from(method_return_type), parameters: parameters.clone() }

}

fn match_controller_code_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, method_return_type: &str, 
    types: &mut TypeTable, class_methods: &mut ClassMethods){
    
    let mut is_return_type = false;

    let mut params: Vec<String> = Vec::new();

    let mut param_name = String::from("");

    let mut class = types.get_type_table(String::from(class_name));

    let class_properties = class.get_type_properties();

    let mut method_variables: Vec<Type> = Vec::new();

    for elem1 in class_properties.into_iter() {
        let mut class_property_class = types.get_type_table(elem1.type_name);

        for elem2 in class_property_class.get_type_properties().into_iter() {
            if elem2.rule == Rule::identifier {

                params.push(String::from(format!("{}: {}", elem2.name, elem2.type_name)));

                param_name = elem2.name;
            }
        }
    }

    code.add_method(String::from(format!("({})", params.join(", "))));

    if method_return_type == "IActionResult" {
        code.add_method(String::from(" -> Result<impl warp::Reply, warp::Rejection>"));
    }

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => { match_new_instance_pairs(elem, is_return_type, &param_name, code, &mut method_variables); },
            Rule::method_call => {},
            Rule::async_method_call => {},
            Rule::assignment => { method_variables.push(match_assignment_code_pairs(elem, class_name, types, class_methods)); },
            Rule::return_key_word => {is_return_type = true;},
            Rule::property_call => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn match_repositories_code_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods){
    
    let mut is_return_type = false;

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem, is_return_type, "", code, &mut Vec::new()),
            Rule::method_call => {},
            Rule::async_method_call => {},
            Rule::assignment =>{ match_assignment_code_pairs(elem, class_name, types, class_methods); },
            Rule::return_key_word => {is_return_type = true;},
            Rule::property_call => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn match_models_constructor_code_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, 
    class_methods: &mut ClassMethods, parameters: &mut Vec<Parameter>){
    
    let mut is_return_type = false;

    let split_class_name: Vec<&str> = class_name.split(".").collect();

    code.add_struct(&format!("\n\tfn new{}(", parameters.len()));

    if parameters.len() > 0 {
        for elem in parameters.clone().into_iter() {
            code.add_struct(&format!("{}: {}, ", elem.name, elem.type_name));
        }
    }

    code.add_struct(&format!(") -> Self {{"));

    code.add_struct(&format!("\n\t\t{} {{", split_class_name[0]));

    let mut class_type = types.get_type_table(String::from(class_name));

    let class_type_properties = class_type.get_type_properties();

    for elem in class_type_properties.into_iter() {

        let found_parameter = parameters.clone().into_iter().find(|a| a.name.to_lowercase() == elem.name.to_lowercase());

        match found_parameter {
            Some(x) => {
                code.add_struct(&format!("\n\t\t\t{}: {}, ", x.name, x.name));
            },
            None => {
                code.add_struct(&format!("\n\t\t\t{}: {}, ", elem.name.to_lowercase(), get_rust_type_default_value(elem.type_name)));
            }
        }
    }

    code.add_struct(&format!("\n\t\t}}"));

    code.add_struct(&format!("\n\t}}\n"));

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem, is_return_type, "", code, &mut Vec::new()),
            Rule::method_call => {},
            Rule::async_method_call => {},
            Rule::assignment => { match_assignment_code_pairs(elem, class_name, types, class_methods); },
            Rule::return_key_word => {is_return_type = true;},
            Rule::property_call => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn match_models_code_pairs(iter: Pair<Rule>, code: &mut Code, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods){
    
    let mut is_return_type = false;

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::new_instance => match_new_instance_pairs(elem, is_return_type, "", code, &mut Vec::new()),
            Rule::method_call => {},
            Rule::async_method_call => {},
            Rule::assignment => { match_assignment_code_pairs(elem, class_name, types, class_methods); },
            Rule::return_key_word => {is_return_type = true;},
            Rule::property_call => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn match_parameters_pairs(iter: Pair<Rule>) -> Vec<Parameter>{

    let mut params: Vec<Parameter> = Vec::new();

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameter => {},
            Rule::action_parameter => {
                let split_parameter: Vec<&str> = elem.as_str().split(" ").collect();

                params.push(Parameter { name: String::from(split_parameter[1]), 
                    type_name: String::from(split_parameter[0]) });
            },
            Rule::constructor_parameter => { 
                params.push(match_contructor_parameter_code_pairs(elem));
            },
            _ => unreachable!()
        }
    }

    params
}

fn match_contructor_parameter_code_pairs(iter: Pair<Rule>) -> Parameter {

    let mut param_class_name = "";

    let mut param_name = "";

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::class_name => {
                param_class_name = elem.as_str();
            }
            Rule::identifier => {
                param_name = elem.as_str();
            }
            Rule::string_key_word => { param_class_name = "String" }
            Rule::decimal_key_word => { param_class_name = "f64" }
            Rule::int_key_word => { param_class_name = "i32" }
            _ => unreachable!()
        }
    }

    Parameter { name: String::from(param_name), type_name: String::from(param_class_name) }
}

fn match_new_instance_pairs(iter: Pair<Rule>, is_return_type: bool, param_name: &str, code: &mut Code, method_variables: &mut Vec<Type>){

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::parameters => { 
                let variable = method_variables.into_iter().find(|a| a.name == elem.as_str()).expect("");

                let is_list = is_list_type(&variable.type_name);

                if is_return_type && is_list {
                    code.add_method(String::from(format!(" {{\n\tlet mut result = Vec::new();\n
                        let r = {}.{}.read();
                        for value in r.iter() {{
                        result.push(value);
                        }}\n\n", param_name, variable.name)));
                }

                code.add_method(String::from("\tOk(warp::reply::json(&result))\n}\n\n"));

                match_parameters_pairs(elem);
            },
            Rule::new_key_word => {},
            Rule::identifier => {},
            Rule::left_parenthesis => {},
            Rule::right_parenthesis => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }
}

fn match_assignment_code_pairs(iter: Pair<Rule>, class_name: &str, types: &mut TypeTable, class_methods: &mut ClassMethods) -> Type {

    let mut var_name = "";

    let mut var_type = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::var_key_word => {

            },
            Rule::method_call => {

                let split_methods: Vec<&str> = elem.as_str().split(".").collect();

                let length = split_methods.len();

                let method_call = split_methods[length - 1];
                let property = split_methods[0];

                let mut class_type = types.get_type_table(String::from(class_name));

                let prop_type = class_type.get_type_property(String::from(property));

                let mut class_method = class_methods.get_class_method(&prop_type.type_name);
 
                let method = class_method.get_method(&method_call.replace("();", ""));

                var_type = method.return_type;
            },
            Rule::property_call => {
                var_name = elem.as_str();
            },
            Rule::new_instance => {
                
            },
            Rule::return_key_word => {

            },
            Rule::math_exp => {

            },
            Rule::number => {},
            Rule::semicolon => {},
            _ =>{ unreachable!(); }
        }
    }

    Type { name: String::from(var_name), type_name: String::from(var_type), rule: Rule::identifier, properties: Vec::new() }
}

fn match_assignment_properties_pairs(iter: Pair<Rule>, property_type: &Type) -> Type {

    let mut prop_name = String::from("");

    let type_name = String::from(&property_type.type_name);

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::property_call => {

                if type_name.is_empty() {
                    continue;
                }

                prop_name = String::from(elem.as_str());
            },
            Rule::new_instance => {},
            Rule::number => {},
            Rule::semicolon => {},
            _ => unreachable!()
        }
    }

    Type { name: prop_name, type_name: type_name, rule: property_type.rule, properties: Vec::new() }
}

fn match_property_type_code_pairs(iter: Pair<Rule>, code: &mut Code, test: &mut Vec<Type>) -> String {

    let mut propety_type = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule(){
            Rule::string_key_word => { 
                propety_type = String::from("String");
                test.push(Type {name: String::from(""), type_name: String::from(elem.as_str()), rule: elem.as_rule(), properties: Vec::new()});
            },
            Rule::int_key_word => { 
                propety_type = String::from("i32");
                test.push(Type {name: String::from(""), type_name: String::from(elem.as_str()), rule: elem.as_rule(), properties: Vec::new()});
            },
            Rule::decimal_key_word => { 
                propety_type = String::from("f64");
                test.push(Type {name: String::from(""), type_name: String::from(elem.as_str()), rule: elem.as_rule(), properties: Vec::new()});
            },
            Rule::list_type => { 
                test.push(Type {name: String::from(""), type_name: String::from(elem.as_str()), rule: elem.as_rule(), properties: Vec::new()});
                propety_type = match_list_type(elem);
            },
            Rule::identifier => { 
                propety_type = String::from(elem.as_str());
                test.push(Type {name: String::from(""), type_name: String::from(elem.as_str()), rule: elem.as_rule(), properties: Vec::new()});
            },
            _ => { propety_type = String::from("<not_found>") }
        }
    }

    propety_type
}

fn match_list_type(iter: Pair<Rule>) -> String {

    let mut property_type = String::from("");

    for elem in iter.into_inner() {
        match elem.as_rule() {
            Rule::identifier => {
                property_type = String::from(format!("Arc<RwLock<Vec<{}>>>", elem.as_str()))
            },
            _ => unreachable!()
        }
    }

    property_type
}

fn get_rust_type_default_value(value: String) -> String {

    if value.to_lowercase() == "string" {
        return String::from("String::from(\"\")")
    }
    else if value == "int" {
        return String::from("0")
    }
    else if value == "decimal" {
        return String::from("0.0")
    }
    else if value.to_lowercase().contains("list") {
        return String::from("Arc::new(RwLock::new(Vec::new()))")
    }

    String::from("")
}

fn is_list_type(value: &str) -> bool {
    value.to_lowercase().contains("list") ||
    value.to_lowercase().contains("ienumerable")
}

fn write_all_to_file(code: &mut Code, types: &mut TypeTable, class_methods: &mut ClassMethods) {
    let mut f = File::create("output.rs")
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