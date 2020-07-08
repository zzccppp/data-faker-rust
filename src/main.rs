use data_faker_rust::rules::IncreaseRule;
use std::any::{Any, TypeId};
use data_faker_rust::configuration::{FakerConfiguration, ConfigurationItem};
use std::borrow::Borrow;
use data_faker_rust::configuration::definitions::VariableType;
use std::sync::Arc;
use std::cell::RefCell;
use std::ops::Deref;
use crossbeam::thread;

fn main() {
    let config = FakerConfiguration::read_from_file(String::from("test.txt").borrow()).unwrap();

    let s = Arc::new(config.items[0].borrow());

    thread::scope(|a| {
        let mut vec = vec![];
        for z in 0..3 {
            let s_clone = s.clone();
            let x = a.spawn(move |_| {
                for _ in 1..100 {
                    let re = s_clone.rule.generate_into(VariableType::String).unwrap();
                    println!("{}:{:?}", z, re);
                }
            });
            vec.push(x);
        }
        for i in vec {
            i.join();
        }
    }).unwrap();
}
