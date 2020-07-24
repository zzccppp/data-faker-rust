use crate::configuration::FakerConfiguration;
use std::borrow::Borrow;
use std::io::{Error, ErrorKind};
use crate::configuration::definitions::{OutPutType, OutPutTypeValue};
use std::fs::File;
use std::sync::mpsc;
use std::thread::Thread;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FakerEngine {
    thread_count: i32,
    config: FakerConfiguration,
}

impl FakerEngine {
    pub fn new(path: &String, thread_count: i32) -> Result<Self, Error> {
        if thread_count <= 0 {
            return Err(Error::new(ErrorKind::Other, "thread_count should > 0"));
        }
        let config = FakerConfiguration::read_from_file(path)?;

        Ok(FakerEngine {
            thread_count,
            config,
        })
    }

    pub fn manufacturing_to_file(&self, tt: OutPutType, new_file_path: String, item_count: u64) -> Result<(), Error> {
        match tt {
            OutPutType::Json => {
                let items_per_sub_thread = item_count / self.thread_count as u64;
                let items_main_thread = item_count % self.thread_count as u64;

                let file = File::create(new_file_path)?;


                for _ in 0..self.thread_count {
                    std::thread::spawn(|| {
                        let mut vec = vec![];
                        for _ in 0..items_per_sub_thread {
                            let js = self.config.generate(OutPutType::Json).unwrap();
                            if let OutPutTypeValue::Json(u) = js {
                                vec.push(u);
                            }
                        }
                    });
                }


                Ok(())
            }
            OutPutType::Csv => {
                Ok(())
            }
        }
    }
}