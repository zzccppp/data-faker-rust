use std::io::ErrorKind;
use std::fmt::Debug;
use std::borrow::Borrow;
use std::num::ParseIntError;
use crate::rules::IncreaseRule;
use std::sync::{Arc, Mutex};
use chrono::Utc;

/// Integer -> 对应 int、integer、smallint和numeric 等等
/// Float -> float、real和double, precision
/// String -> char、varchar、binary、varbinary、blob、text、enum和set
/// Date -> datetime、date、timestamp、time和year
#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Integer,
    Float,
    String,
    Date,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableTypeValue {
    Integer(i64),
    Float(f64),
    String(String),
    Date(Utc),
}

pub trait ConstructRule {
    fn construct(s: String) -> Result<Box<Self>, ()>;
}

///代表数据生成规则的trait
pub trait GenerateRule: Debug + GenerateRuleClone + Sync + Send {
    ///生成对应数据类型的数据，如果不能生成则返回Err（基本Err是 address 无法作为 Integer 输出）
    fn generate_into(&self, into_type: VariableType) -> Result<VariableTypeValue, ()>;
}

pub trait GenerateRuleClone {
    fn clone_box(&self) -> Box<dyn GenerateRule>;
}

impl<T> GenerateRuleClone for T
    where T: Clone + GenerateRule + 'static
{
    fn clone_box(&self) -> Box<dyn GenerateRule> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<dyn GenerateRule> {
    fn clone(&self) -> Box<dyn GenerateRule> {
        self.clone_box()
    }
}

/////识别Configuration的第三列数据，构造规则，若输入不符合要求返回Err(())
pub fn construct_from_str(config_str: &str) -> Result<Box<dyn GenerateRule>, ()> {
    let s = config_str.trim().to_lowercase();
    if s.starts_with("increase") {
        return match IncreaseRule::construct(s) {
            Ok(e) => {
                Ok(e as Box<dyn GenerateRule>)
            }
            Err(_) => {
                Err(())
            }
        };
    }

    Err(())
}

/// 将 foo(var1,var2,var3) 简单变成 [var1,var2,var3] 的vec
pub fn remove_parentheses(s: &String) -> Result<Vec<&str>, ()> {
    let left_brackets_idx = match s.find("(") {
        None => {
            return Err(());
        }
        Some(e) => {
            e
        }
    };
    let right_brackets_idx = match s.find(")") {
        None => {
            return Err(());
        }
        Some(e) => {
            e
        }
    };
    let sub_str = &s[left_brackets_idx + 1..right_brackets_idx];
    let args: Vec<&str> = sub_str.split(",").collect();
    return Ok(args);
}