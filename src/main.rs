#![deny(warnings)]
use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use regex::Regex;

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

fn is_an_image(name: String) -> bool {
    println!("inside is_an_image and value of name: {:?}", name);
    let image_formats = vec![".jpg", ".jpeg", ".gif", ".pdf", ".svg", ".tiff", ".webp", ".png"];
    let mut is_an_image = false;
    for ext in image_formats{
        is_an_image = name.contains(ext);
        if is_an_image {
            break;
        }
    }
    is_an_image
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
    fn add_picture_to_page(&mut self) -> Self{
        let p = Parse{contents: self.contents.clone()};
        let re = Regex::new(r"!\[\[([^\[\[]+)\]\]").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text){
            let uncleaned_capture = &capture[0].to_string();
            let cleaned_capture = &capture[0].to_string().replace("[", "").replace("]", "").replace("!", "");
            let imagestring = [
                "<img src='/img/", 
                cleaned_capture, 
                "'/>"
            ].join("");
            cleantext = cleantext.replace(uncleaned_capture, &imagestring);
        }
        let p = Parse{contents: cleantext};
        p
    }

    #[allow(dead_code)]
    fn link_to_another_page(&mut self) -> Self{
        let p = Parse{contents: self.contents.clone()};
        let re = Regex::new(r"\[\[([^\[\[]+)\]\]").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text){
            println!("&&&&&&&&&&&&&&&&&&&&&&&&&");
            println!("the value of the capture is {:?}", capture);
            println!("&&&&&&&&&&&&&&&&&&&&&&&&&");
            let uncleaned_capture = &capture[0].to_string();
            let cleaned_capture = &capture[0].to_string().replace("[", "").replace("]", "");
            if cleaned_capture.find("#").is_none() && !is_an_image(cleaned_capture.to_string()) {
                let (anchor_visible_name, anchor_href) = match cleaned_capture.match_indices("|").find_map(|(i, _val)| Some(i)) {
                    Some(cleaned_text_index) => {
                        println!("the value of res: {:?}", cleaned_capture);
                        let anchor_visible_name = cleaned_capture.get(cleaned_text_index+1..cleaned_capture.len()).unwrap();
                        let anchor_href = cleaned_capture.get(0..cleaned_text_index).unwrap();
                        (anchor_visible_name.to_string(), anchor_href.to_string())
                        
                    }, 
                    None => {
                        println!("character | was not found");
                        (cleaned_capture.to_string(), cleaned_capture.to_string())
                    }
                };
                println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                println!("The value of anchor_visible_name, anchor_href; {:?} {:?}", anchor_visible_name, anchor_href);
                println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                let anchor_string = [
                    "<a href='/html/", 
                    &anchor_href,
                    ".html'/>", 
                    &anchor_visible_name,
                    "</a>"
                ].join("");
                cleantext = cleantext.replace(uncleaned_capture, &anchor_string);
            }else{
                //todo
            }
        }
        let p = Parse{contents: cleantext};
        p
    }
}

fn create_page(entry_path: String, contents: String){
   let new_path = entry_path
       .replace("obsidian_project", "obsidian_html")
       .replace(".md", ".html");
   let mut file = fs::File::create(new_path).unwrap();
   file.write_all(contents.as_bytes()).unwrap();
}

fn store_images(entry_path:String){
    let new_path = entry_path
        .replace("obsidian_project", "obsidian_img");
    let _result = fs::copy(entry_path, new_path);
}

fn parse_file(entry_path: String){
    println!("in parse file and value of entry_path {:?}", entry_path.clone());
    if !is_an_image(entry_path.clone().to_string()){
        let contents = fs::read_to_string(entry_path.clone())
            .expect("Should have been able to read the file");
        println!("With text: \n{contents}");
        let parsing_contents = Parse{contents: contents};
        let parsed_contents = parsing_contents
            .carriage_return()
            .add_picture_to_page()
            .link_to_another_page();
        println!("the value of parsed_contents is {:?}", parsed_contents.contents);
        //println!("the value of parsing_contents after munging: {:?}", parsing_contents.contents.clone());
        create_page(entry_path.clone(), parsed_contents.contents.clone());
    }else{
        store_images(entry_path.clone());
    }
}

fn read_files(){
    let path = Path::new("./src/obsidian_project");
    match fs::remove_dir_all("./src/obsidian_html"){
        Ok(x) => println!("remove_dir_all: {:?}", x), 
        Err(x) => println!("there was an error in remove_dir_all {:?}", x)
    }
    match fs::remove_dir_all("./src/obsidian_img"){
        Ok(x) => println!("remove dir_all: {:?}", x),
        Err(x) => println!("there was an error in remove_dir_all {:?}", x)
    }
    fs::create_dir_all("./src/obsidian_html").unwrap();
    fs::create_dir_all("./src/obsidian_img").unwrap();
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

    let html = warp::path("html").and(warp::fs::dir("src/obsidian_html/"));
    let img = warp::path("img").and(warp::fs::dir("src/obsidian_img/"));
    let routes = warp::get().and(
        home_page
        .or(hi)
        .or(html)
        .or(img)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
