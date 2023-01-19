#![deny(warnings)]
use std::sync::Arc;
use serde::{Serialize};
use serde_json::json;
use handlebars::Handlebars;
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


// Code I won't use in the template but am keeping for notes
// this is the message {% message %}
// {{user}}

#[tokio::main]
async fn main() {

    let document_list = "
        <script src='https://unpkg.com/vue@3/dist/vue.global.js'></script>
        <div id='document_list'>TESTING: {% message %}</div>
        <script>
            const { createApp } = Vue

            export default createApp({
                data() {
                    return {
                        message: 'Hello from document_list'
                    }
                }, 
                delimiters: ['{%', '%}'],
            }).mount('#document_list')
        </script>
    ";

    let template = "
                    <script src='https://unpkg.com/vue@3/dist/vue.global.js'></script>


                    <div id='app'>
                        <h1>
                            Welcome to Web Page
                        </h1>
                        <h3>
                            This is a static site generator porting Obsidian to the Web! 
                        </h3>
                    
                        <p>
                            This project allows you to take an <a href='https//www.obsidian.md'>Obsidian.md</a> project, upload it and then edit the project using Vue.js in a WYSIWYG (what you see is what you get).
                            For the moment the project is primarily built around get requests, although post requests may be in the works (not sure).
                        </p>

                        <p>
                            Here are the major project guidelines - 
                        </p>
                        
                        <ul>
                            <li>
                                Statically upload Obsidian files - done!
                            </li>
                            <li>
                                WYSIWYG modification of files using Vue.js - TODO
                            </li>
                            <li>
                                Login/Auth for multiple users - todo
                            </li>
                            <li>
                                Socketing real time collaboration on file editing - TODO (needs login/auth as well).
                            </li>
                            <li>
                                General css work on UI/UX for the dashboard interface - TODO
                            </li>
                        </ul>
                       
                        <div>
                            testing a message: {% message %}
                        </div>
                        
                        <div>
                            {{html_files.0}}
                        </div>

                        <div>
                            there should be a document list here
                            <Document_List />
                        </div>

                        <ul>
                            <li v-for='value in {{html_files}}'>
                                {{ value }}
                            </li>
                        </ul>

                        <div>
                            [[html_files]]
                        </div>
                    
                    </div>
                        
                    <script>
                        const { createApp } = Vue
                        import Document_List from './document_list.html'
                        let html_files = {{html_files}};

                        console.log('the value of html_files:', {{html_files}});

                        createApp({
                            data() {
                                return {
                                    message: 'Hello Vue!'
                                }
                            },
                            delimiters: ['{%', '%}'],
                            mounted(){
                                console.log('inside the mounted function');
                                this.$forceUpdate()
                            }
                        }).mount('#app')

                    </script>
                    ";
    read_files();
    
    let mut hb = Handlebars::new();
    hb.register_template_string("template.html", template)
        .unwrap();    
    hb.register_template_string("document_list.html", document_list)
        .unwrap();
    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    
    let paths_html = fs::read_dir("./src/obsidian_html").unwrap();
    let paths_img = fs::read_dir("./src/obsidian_img").unwrap();

    let mut html_vec: Vec<String> = vec![];
    let mut img_vec:  Vec<String> = vec![];

    //let mut i_html:u32 = 0;
    for path in paths_html {
        html_vec.append(&mut vec![path.unwrap().path().display().to_string()]);
    }

    //let mut i_img:u32 = 0;
    for path in paths_img {
        img_vec.append(&mut vec![path.unwrap().path().display().to_string()]);
    }

    println!("value of html_vec {:?}", html_vec.clone());
    println!("value of img_vec  {:?}", img_vec.clone());
    
    //let uploaded_files : UploadedFiles = serde_json::from_str(data).unwrap();

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");

    let home_page = warp::get()
        .and(warp::path::end())
        .map(move || WithTemplate {
            name: "template.html",
            value: json!({
                "user": "Warp", 
                "test": {
                    "0": "something"
                },
                "html_files": html_vec, 
                "img_files" : img_vec
            }),
        })
        .map(handlebars.clone());

    let document_list = warp::path("document_list")
        .map(move || WithTemplate {
            name: "document_list.html",
            value: json!({})
        })
        .map(handlebars.clone());

    let hi = warp::path("hi").map(|| "Hello, World!");
    let html = warp::path("html").and(warp::fs::dir("src/obsidian_html/"));
    let img = warp::path("img").and(warp::fs::dir("src/obsidian/html/"));
    let routes = warp::get().and(
        home_page
        .or(hi)
        .or(html)
        .or(img)
        .or(document_list)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
