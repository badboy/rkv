extern crate rkv;
extern crate clap;

use std::str;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use rkv::{
    Manager,
    Rkv,
    Store,
    Value,
};

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("rkv")
        .version("1.0")
        .author("Mozilla")
        .about("Interact with a rkv database")
        .arg(Arg::with_name("database")
             .short("d")
             .long("database")
             .value_name("PATH")
             .help("Path to the database")
             .global(true)
             .takes_value(true))
        .arg(Arg::with_name("store")
             .short("s")
             .long("store")
             .value_name("NAME")
             .help("Name of the store to use")
             .global(true)
             .takes_value(true))
        .subcommand(SubCommand::with_name("dump")
                    .about("dump full database"))
        .subcommand(SubCommand::with_name("put")
                    .about("add key/value pair")
                    .arg(Arg::with_name("key")
                         .help("the key of the pair")
                         .index(1)
                         .required(true)
                        )
                    .arg(Arg::with_name("value")
                         .help("the value of the pair")
                         .index(2)
                         .required(true)
                        )
                    )
        .subcommand(SubCommand::with_name("get")
                    .about("get value of key")
                    .arg(Arg::with_name("key")
                         .help("the key of the pair")
                         .index(1)
                         .required(true)
                        )
                   )
        .subcommand(SubCommand::with_name("del")
                    .about("delete a key")
                    .arg(Arg::with_name("key")
                         .help("the key of the pair")
                         .index(1)
                         .required(true)
                        )
                   )
}

fn get_store<'a>(name: Option<&str>, env: &'a Rkv) -> Store<&'a str> {
    match name {
        Some(s) => env.open_or_create(s).unwrap(),
        None => env.open_or_create_default().unwrap()
    }
}

fn main() {
    let matches = app().get_matches();

    let path = matches.value_of("database").unwrap_or(".");
    let store_name = matches.value_of("store");
    let path = Path::new(path);
    let created_arc = Manager::singleton().write().unwrap().get_or_create(path, Rkv::new).unwrap();

    if let Some(_matches) = matches.subcommand_matches("dump") {
        let env = created_arc.read().unwrap();
        let store = get_store(store_name, &env);
        let reader = store.read(&env).unwrap();

        let mut iter = reader.iter_start().unwrap();
        for (country, city) in iter {
            let city = city.unwrap().unwrap();
            println!("{}: {:?}", str::from_utf8(country).unwrap(), city);
        }
    }

    if let Some(matches) = matches.subcommand_matches("put") {
        let env = created_arc.read().unwrap();
        let store = get_store(store_name, &env);
        let mut writer = store.write(&env).unwrap();
        let key = matches.value_of("key").unwrap();
        let value = matches.value_of("value").unwrap();

        writer.put(key, &Value::Str(value)).unwrap();
        writer.commit().unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let env = created_arc.read().unwrap();
        let store = get_store(store_name, &env);
        let reader = store.write(&env).unwrap();
        let key = matches.value_of("key").unwrap();

        let value = reader.get(key).unwrap();
        println!("{}: {:?}", key, value);
    }

    if let Some(matches) = matches.subcommand_matches("del") {
        let env = created_arc.read().unwrap();
        let store = get_store(store_name, &env);
        let mut writer = store.write(&env).unwrap();
        let key = matches.value_of("key").unwrap();

        writer.delete(key).unwrap();
        writer.commit().unwrap();
        println!("{} deleted!", key);
    }
}
