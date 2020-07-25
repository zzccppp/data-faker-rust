use crate::configuration::FakerConfiguration;
use std::borrow::Borrow;
use std::io::{Error, ErrorKind, Write};
use crate::configuration::definitions::{OutPutType, OutPutTypeValue};
use std::fs::File;
use std::sync::{mpsc, Arc, Mutex, Barrier};
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
                let items_per_sub_thread = item_count / (self.thread_count as u64 - 1);
                let items_main_thread = item_count % (self.thread_count as u64 - 1);

                let mut file = File::create(new_file_path)?;

                let mut v = Vec::<Value>::new();
                let v_pt = Arc::new(Mutex::new(v));
                //let barrier = Arc::new(Barrier::new());

                crossbeam::thread::scope(|a| {
                    for _ in 0..(self.thread_count - 1) {
                        a.spawn(|_| {
                            //let mut vec = vec![];
                            for _ in 0..items_per_sub_thread {
                                let js = self.config.generate(OutPutType::Json).unwrap();
                                if let OutPutTypeValue::Json(u) = js {
                                    let mut vec = v_pt.lock().unwrap();
                                    vec.push(u);
                                }
                            }
                        });
                    }
                    for _ in 0..items_main_thread {
                        let js = self.config.generate(OutPutType::Json).unwrap();
                        if let OutPutTypeValue::Json(u) = js {
                            let mut vec = v_pt.lock().unwrap();
                            vec.push(u);
                        }
                    }
                });

                let value = Value::Array(v_pt.lock().unwrap().clone());

                file.write(value.to_string().as_bytes());

                Ok(())
            }
            OutPutType::Csv => {
                let items_per_sub_thread = item_count / (self.thread_count as u64 - 1);
                let items_main_thread = item_count % (self.thread_count as u64 - 1);

                let mut file = File::create(new_file_path)?;
                let mut wtr = csv::Writer::from_writer(file);
                let w_pt = Arc::new(Mutex::new(wtr));

                crossbeam::thread::scope(|a| {
                    for _ in 0..(self.thread_count - 1){
                        a.spawn(|_| {
                            for _ in 0..items_per_sub_thread {
                                let js = self.config.generate(OutPutType::Csv).unwrap();
                                if let OutPutTypeValue::Csv(u) = js {
                                    w_pt.lock().unwrap().write_record(u);
                                }
                            }
                        });
                    }
                    for _ in 0..items_main_thread {
                        let js = self.config.generate(OutPutType::Csv).unwrap();
                        if let OutPutTypeValue::Csv(u) = js {
                            w_pt.lock().unwrap().write_record(u);
                        }
                    }
                });

                Ok(())
            }
        }
    }
}