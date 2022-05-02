extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::{self, File};
use std::io::Write;
use pest::iterators::Pair;
use std::io::Read;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CSParser;

struct Code {
    usings: Vec<String>,
    types: Vec<String>,
    structs: Vec<String>,
    methods: Vec<String>
}

impl Code {
    fn new() -> Self {
        Code {
            usings: Vec::new(),
            types: Vec::new(),
            structs: Vec::new(),
            methods: Vec::new()
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

#[derive(PartialEq)]
pub enum Step {
    models,
    repositories,
    controllers
}

fn main() {

    let mut code = Code::new();

    parse_models_contents(&mut code, &Step::models);

    //parse_repository_contents(&mut code, &Step::repositories);

    write_all_to_file(&mut code);

    //parse_controller_contents();

}

fn parse_models_contents(code: &mut Code, step: &Step) {

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

            match_pairs(pair, code, step);
        }
    }
}

fn parse_repository_contents(code: &mut Code, step: &Step) {

    let contents = read_files("/home/fabrilsson/Documents/repo/CSharpSandbox/GroceriesApi/Repositories/").expect("Error");

    for models_contents in contents {

        let text = models_contents.replace("\u{feff}", "");

        println!("With text:\n{}", models_contents);

        let successful_parse = CSParser::parse(Rule::parse_models_contents, &text).unwrap_or_else(|e| panic!("{}", e));

        println!("{:?}", successful_parse);

        for pair in successful_parse {
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());

            match_pairs(pair, code, step);
        }
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
                else if *step == Step::repositories {
                    match_repositories_code_pairs(elem, code, step)
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

    let mut class_name: &str = "";

    if *step == Step::models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => match_constructor_pairs(elem),
                Rule::action => match_action_pairs(elem),
                Rule::properties => match_properties_pairs(elem, code, step, class_name),
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
    }
}

fn match_repositories_code_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step){

    let mut class_name: &str = "";

    if *step == Step::models {
        for elem in iter.into_inner() {
            match elem.as_rule(){
                Rule::constructor => match_constructor_pairs(elem),
                Rule::action => match_action_pairs(elem),
                Rule::properties => match_properties_pairs(elem, code, step, class_name),
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
    }
}

fn match_properties_pairs(iter: Pair<Rule>, code: &mut Code, step: &Step, class_name: &str) {

    let mut property_type = vec!();

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
                code.add_struct("   pub ");
            },
            Rule::private_key_word => 
            {
                code.add_struct("   pub ");
            },
            Rule::property_type => property_type.push(elem.as_str()),
            Rule::identifier => 
            {
                let prop_type = property_type.pop().unwrap_or("<not_found>");

                let processed_prop_type = get_equivalent_rust_type(prop_type);

                if create_hashmap{
                    code.add_type(&format!("type {}s = HaspMap<{}, {}>;\n", class_name, processed_prop_type, class_name));
                }

                code.add_struct(&format!("{}: {},\n", elem.as_str().to_lowercase(), processed_prop_type));

                return;
            },
            _ => unreachable!()
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
            Rule::action_return_type => println!("teste2:  {}", elem.as_str()),
            Rule::action_async_return_type => println!("teste2:  {}", elem.as_str()),
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

fn get_equivalent_rust_type(value: &str) -> &str {
    
    if value == "int"{
        return "i32";
    }

    if value == "string"{
        return "String";
    }

    if value == "decimal"{
        return "f64";
    }

    if value == "List<Item>"{
        return "Arc<RwLock<Items>>";
    }

    return "<not_found>";
}

fn write_all_to_file(code: &mut Code) {
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
}