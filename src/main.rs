#![deny(warnings)]
use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;
use std::path::Path;
use std::fs;
use std::io::prelude::*;


struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

struct Parse {
    contents: String
}
impl Parse {
    fn carriage_return(&self) -> Self {
        let new_contents = self.contents.replace("\n", "<br/>");
        let p = Parse{contents: new_contents};
        p
    }
    #[allow(dead_code)]
    fn link_to_another_page(&mut self) -> Self{
        let p = Parse{contents: self.contents.clone()};
        println!("value of link_to_another_page {:?}", self.contents.clone());
        p
//        self.contents
//        let returnval = self.contents.to_string();
//        let mut returncondition = 0;
//        for (i, _c) in returnval.chars().enumerate() {
//            if i == 0 || i == 1 || i == self.contents.len()-1 || i == self.content.len()-2{
//                returncondition +=1;
//            }
//        }
//        let mut returnvalchars = returnval.chars();
//        if returncondition == 4{
//            returnvalchars.next();
//            returnvalchars.next();
//            returnvalchars.next_back();
//            returnvalchars.next_back();
//        }
//       returnvalchars.as_str().to_string()
    }
}

fn create_file(entry_path: String, contents: String){
   let new_path = entry_path
       .replace("obsidian_project", "obsidian_js")
       .replace(".md", ".html");
   let mut file = fs::File::create(new_path).unwrap();
   file.write_all(contents.as_bytes()).unwrap();
}

fn parse_file(entry_path: String){
    println!("in parse file and value of entry_path {:?}", entry_path.clone());
    let contents = fs::read_to_string(entry_path.clone())
        .expect("Should have been able to read the file");
    println!("With text: \n{contents}");
    let parsing_contents = Parse{contents: contents};
    let parsed_contents = parsing_contents
        .carriage_return()
        .link_to_another_page();
    println!("the value of parsed_contents is {:?}", parsed_contents.contents);
    //println!("the value of parsing_contents after munging: {:?}", parsing_contents.contents.clone());
    create_file(entry_path.clone(), parsed_contents.contents.clone());
}

fn read_files(){
    let path = Path::new("./src/obsidian_project");
    match fs::remove_dir_all("./src/obsidian_js"){
        Ok(x) => println!("remove_dir_all: {:?}", x), 
        Err(x) => println!("there was an error in remove_dir_all {:?}", x)
    }
    fs::create_dir_all("./src/obsidian_js").unwrap();
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        println!("{}", entry.path().display());
        parse_file(entry.path().display().to_string());
    }
}

#[tokio::main]
async fn main() {
    let template = "
                    <script src='https://unpkg.com/vue@3/dist/vue.global.js'></script>
                    
                    <div id='app'>
                        this is the message : {% message %}
                        {{user}}
                    </div>
                        
                    <script>
                        const { createApp } = Vue

                        createApp({
                            data() {
                                return {
                                    message: 'Hello Vue!'
                                }
                            },
                            delimiters: ['{%', '%}'],
                            mounted(){
                                console.log('the app is mounted');
                                console.log('this is the message: ', this.message);
                                this.$forceUpdate();
                            }
                        }).mount('#app')
                    </script>
                    ";
    read_files();
    let mut hb = Handlebars::new();
    hb.register_template_string("template.html", template)
        .unwrap();    
    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    //let home_page = warp::path::end().map(|| "Hello, World at root!");
    
    //GET /
    let home_page = warp::get()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user" : "Warp"}),
        })
        .map(handlebars);

    let hi = warp::path("hi").map(|| "Hello, World!");

    let readme = warp::path("readme").and(warp::fs::dir("src/obsidian_js/"));
    let routes = warp::get().and(
        home_page
        .or(hi)
        .or(readme)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
