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

    let s = Arc::new(config.items[2].borrow());

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

#[cfg(test)]
mod test {
    use std::io;
    use std::fs::File;
    use serde_json::{Value, Number};
    use serde_json::json;
    use data_faker_rust::configuration::FakerConfiguration;
    use std::borrow::Borrow;
    use data_faker_rust::configuration::definitions::{OutPutType, OutPutTypeValue};

    #[test]
    pub fn csv_test() {
        let mut wtr = csv::Writer::from_writer(File::create("test.csv").unwrap());

        // When writing records without Serde, the header record is written just
        // like any other record.
        wtr.write_record(&["ci,ty", "region", "country", "population"]);
        wtr.write_record(&["Southborough", "MA", "United States", "9686"]);
        wtr.write_record(&["Northbridge", "MA", "United States", "14061"]);
        wtr.flush();
    }

    #[test]
    pub fn json_test() {
        let mut vec = Vec::<Value>::new();
        vec.push(Value::String("123123".parse().unwrap()));
        vec.push(Value::Bool(true));
        vec.push(json!(123));

        let v = Value::Array(vec);
        println!("{}", v.to_string());

        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
               "+44 1234567",
                "+44 2345678"
            ]
        });

        println!("{}", john.to_string());
    }

    #[test]
    pub fn test_for_generate_json() {
        let config = FakerConfiguration::read_from_file(String::from("test.txt").borrow()).unwrap();
        let mut vec = vec![];
        for _ in 0..100 {
            let js = config.generate(OutPutType::Json).unwrap();
            if let OutPutTypeValue::Json(u) = js {
                vec.push(u);
            }
        }
        let out_json = Value::Array(vec);
        println!("{}", out_json.to_string());
    }

    #[test]
    pub fn test_for_generate_csv() {
        let config = FakerConfiguration::read_from_file(String::from("test.txt").borrow()).unwrap();
        let mut wtr = csv::Writer::from_writer(File::create("test.csv").unwrap());
        // todo 写column名
        for _ in 0..100 {
            let js = config.generate(OutPutType::Csv).unwrap();
            if let OutPutTypeValue::Csv(u) = js {
                wtr.write_record(u);
            }
        }
    }
}
