use std::sync::{Mutex, Arc};
use crate::configuration::definitions::{GenerateRule, VariableType, VariableTypeValue, ConstructRule, remove_parentheses};
use std::error::Error;
use std::fmt::Debug;
use serde::export::Formatter;
use core::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::num::ParseIntError;
use std::fs::read_to_string;

/// increase(start,step)
/// increase(INTEGER,INTEGER)
#[derive(Debug)]
pub struct IncreaseRule {
    pub(crate) start: i64,
    pub(crate) step: i64,
    pub(crate) now: Arc<Mutex<i64>>,
}

impl ConstructRule for IncreaseRule {
    fn construct(s: String) -> Result<Box<Self>, ()> {
        let args = remove_parentheses(s.borrow())?;

        //check number of parameter
        if args.len() != 2 {
            return Err(());
        }

        let start: i64 = match args[0].parse::<i64>() {
            Ok(o) => { o }
            Err(_) => { return Err(()); }
        };

        let step: i64 = match args[1].parse::<i64>() {
            Ok(o) => { o }
            Err(_) => { return Err(()); }
        };

        return Ok(Box::new(IncreaseRule {
            start,
            step,
            now: Arc::new(Mutex::new(start)),
        }));
    }
}

impl GenerateRule for IncreaseRule {
    fn generate_into(&self, into_type: VariableType) -> Result<VariableTypeValue, ()> {
        match into_type {
            VariableType::Integer => {
                let s = self.now.lock().unwrap().deref().clone();
                *self.now.lock().unwrap() += self.step;
                return Ok(VariableTypeValue::Integer(s));
            }
            VariableType::Float => {
                return Err(());
            }
            VariableType::String => {
                let s = self.now.lock().unwrap().deref().clone();
                *self.now.lock().unwrap() += self.step;
                return Ok(VariableTypeValue::String(format!("{}", s)));
            }
            VariableType::Date => {
                return Err(());
            }
        };
    }
}

impl Clone for IncreaseRule {
    fn clone(&self) -> Self {
        return Self {
            start: self.start,
            step: self.step,
            now: Arc::from(Mutex::new(self.now.lock().unwrap().deref().clone())),
        };
    }
}

// ------------ end of the IncreaseRule ---------

/// 枚举类型
/// 对应不同的数据类型
/// var1||int||enum(1,2,3,4,5)
/// var2||float||enum(1.0,2.0,3.0,4.0,5.0)
/// var3||string||enum(abc,def,dds,ffq,eer,wwt)
#[derive(Debug, Clone)]
pub struct EnumRule {
    enumeration: Vec<String>,
}

impl ConstructRule for EnumRule {
    fn construct(s: String) -> Result<Box<Self>, ()> {
        //enum(var1,var2,...)
        //enum(STRING|INTEGER|FLOAT|DATE,...)
        let args = remove_parentheses(s.borrow())?;

        //check number of parameter
        if args.len() == 0 {
            return Err(());
        }

        let mut enumeration = vec![];

        for i in args {
            enumeration.push(String::from(i));
        }

        Ok(Box::new(Self {
            enumeration,
        }))
    }
}

impl GenerateRule for EnumRule {
    fn generate_into(&self, into_type: VariableType) -> Result<VariableTypeValue, ()> {
        match into_type {
            VariableType::Integer => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return match en.parse::<i64>() {
                    Ok(e) => {
                        Ok(VariableTypeValue::Integer(e))
                    }
                    Err(_) => {
                        Err(())
                    }
                };
            }
            VariableType::Float => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return match en.parse::<f64>() {
                    Ok(e) => {
                        Ok(VariableTypeValue::Float(e))
                    }
                    Err(_) => {
                        Err(())
                    }
                };
            }
            VariableType::String => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return Ok(VariableTypeValue::String(en.clone()));
            }
            VariableType::Date => {
                Err(())
            }
        }
    }
}

// ------------ end of the EnumRule ---------

#[derive(Debug, Clone)]
pub struct EnumFileRule {
    enumeration: Vec<String>,
}

impl GenerateRule for EnumFileRule {
    fn generate_into(&self, into_type: VariableType) -> Result<VariableTypeValue, ()> {
        match into_type {
            VariableType::Integer => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return match en.parse::<i64>() {
                    Ok(e) => {
                        Ok(VariableTypeValue::Integer(e))
                    }
                    Err(_) => {
                        Err(())
                    }
                };
            }
            VariableType::Float => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return match en.parse::<f64>() {
                    Ok(e) => {
                        Ok(VariableTypeValue::Float(e))
                    }
                    Err(_) => {
                        Err(())
                    }
                };
            }
            VariableType::String => {
                let mut rng = rand::thread_rng();
                let len = self.enumeration.len();
                let ran = rng.gen_range(0, len);
                let en = self.enumeration.get(ran).unwrap();
                return Ok(VariableTypeValue::String(en.clone()));
            }
            VariableType::Date => {
                Err(())
            }
        }
    }
}

impl ConstructRule for EnumFileRule {
    fn construct(s: String) -> Result<Box<Self>, ()> {
        let args = remove_parentheses(s.borrow())?;
        if args.len() != 1 {
            return Err(());
        }

        let path = args[0];
        let file = match read_to_string(path) {
            Ok(e) => { e }
            Err(_) => { return Err(()); }
        };
        let v: Vec<_> = file.lines().filter(|ss| {
            ss.ne(&"")
        }).map(|ss| { String::from(ss) }).collect();

        Ok(Box::new(EnumFileRule {
            enumeration: v
        }))
    }
}
