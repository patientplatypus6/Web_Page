# Using Obsidian as a Static Site Generator

Here's a way to use Obsidian to convert your files into a static site generator. It's in the first stages, so all it does is take the markdown files, converts them to html and then searches for any page links using the delimiters `[[` and `]]` and then converts them into anchor links using Regex. The next steps would be to add a WSIWYG html/css editor that would incorporate Vue.js, find a way to organize the files, and add the ability to add pictures, and links to sections. 

If I add the ability to use Vue.js what it would be is a static site uploader that then is modified using Javascript into a fully functional website.
So the first step would be to have some collection of linked Obsidian files someone would upload to a server, and then a designer would use the WSIWYG to modify the files in order to make them dynamic using Vue.js.

I still don't know how I'm going to deal with allowing users to upload content to a server or handle a database. I could see it being useful to have these features, but the more I add on in terms of functionality i
n a GUI the more complicated it would be to use and the harder it would be to deal with edge cases. This will take some thought.

There are three options to do this
- I could leave all the database posting and manipulation in code
- I could have a GUI to deal with posting and pulling user data
- I could require there to be flags in an Obsidian document to deal with user data (like, say, having a `[[this is a user textbox]]` field in Obsidian.

I don't know if going to all that trouble is worth it, rather than just having a way for people to spin up javascript only non-user-interactive sites.


# A Simple Web Page with Rust and Vue.js

Here's a simple web page with Rust and Vue.js. It uses a the Warp crate for routing and handlebars for templating. Straitforward way to start off building a page with templating and a reactive js framework. Next steps would be to add a database, saving of data in the front end (Rust -> Vue) and then manipulating that data, and posting data (Vue -> Rust) in order to save it to the database. I may not do the last step, because online users are terrible and allowing people to save information on the internet is sadness. Then I'd have to find hosting somewhere, but might just use a static hosting service such as github. Who knows? Putting this here in case someone is searching for an easy way to start a web project.
