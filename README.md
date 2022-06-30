# locutus
BORG Backup web 

I am learning Rust, while at the same time learning about BORG Backup. It's a great commandline tool. To make it more useful for other people in my company, i am building a webgui as a project to learn Rust. It will consist of a client (Locutus) which will first only handle some logging, but eventually will manage the whole process of backing things up using an .ini file for its settings. And a server (Unimatrix) that stores the logging into a Postgresql database. Eventually Unimatrix will be a central web interface for multiple clients to receive their settings, save their loggings, etc etc. 
