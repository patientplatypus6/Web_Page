#![deny(warnings)]
use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

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

    let routes = warp::get().and(
        home_page
        .or(hi)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
