use std::sync::{Mutex, Arc};
use crate::configuration::definitions::{GenerateRule, VariableType, VariableTypeValue, ConstructRule, remove_parentheses};
use std::error::Error;
use std::fmt::Debug;
use serde::export::Formatter;
use core::fmt;
use std::ops::Deref;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct IncreaseRule {
    pub(crate) start: i64,
    pub(crate) step: i64,
    pub(crate) now: Arc<Mutex<i64>>,
}

impl ConstructRule for IncreaseRule {
    fn construct(s: String) -> Result<Box<Self>, ()> {
        //increase(start,step)
        //increase(INTEGER,INTEGER)
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

#[derive(Debug, Clone)]
pub struct EnumRule {
    enumeration: Vec<String>
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

        for i in args{
            enumeration.push(String::from(i));
        }

        Ok(Box::new(Self {
            enumeration
        }))
    }
}

impl GenerateRule for EnumRule {
    fn generate_into(&self, into_type: VariableType) -> Result<VariableTypeValue, ()> {
        match into_type {
            VariableType::Integer => {
                let rng = rand::thread_rng();

                Err(())
            },
            VariableType::Float => {
                Err(())
            },
            VariableType::String => {
                Err(())
            },
            VariableType::Date => {
                Err(())
            },
        }
    }
}


