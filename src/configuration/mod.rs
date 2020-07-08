use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use crate::configuration::definitions::{VariableType, GenerateRule, construct_from_str};
use crate::rules::IncreaseRule;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub mod definitions;


/// 配置文件格式定义：
/// 变量名||变量类型||数据制造规则
/// '#'后的不会解析
///
#[derive(Debug, Clone)]
pub struct FakerConfiguration {
    pub items: Vec<ConfigurationItem>
}

#[derive(Debug, Clone)]
pub struct ConfigurationItem {
    pub var_name: String,
    pub var_type: VariableType,
    pub rule: Box<dyn GenerateRule>,
}

impl FakerConfiguration {
    pub fn read_from_file(path: &String) -> Result<FakerConfiguration, Error> {
        let re = read_to_string(path)?;
        let mut config = FakerConfiguration { items: vec![] };
        for line in re.lines() {
            if line.trim().starts_with("#") {
                continue;
            }
            let split: Vec<&str> = line.split("||").collect();

            if split.len() != 3 {
                return Err(Error::new(ErrorKind::InvalidData, "输入格式解析错误"));
            }

            let tt = match variable_type_from_str(split[1]) {
                Ok(o) => {
                    o
                }
                Err(_) => {
                    return Err(Error::new(ErrorKind::InvalidData, "数据类型解析错误"));
                }
            };

            let rule = match construct_from_str(split[2]) {
                Ok(o) => {
                    o
                }
                Err(_) => {
                    return Err(Error::new(ErrorKind::InvalidData, "构造规则解析错误"));
                }
            };

            let it = ConfigurationItem {
                var_name: split[0].to_string(),
                var_type: tt,
                rule,
            };
            config.items.push(it);
        }
        Ok(config)
    }
}

lazy_static! {
    static ref VariableTypeMap : HashMap<&'static str,VariableType> = {
        let mut m = HashMap::new();
        m.insert("int",VariableType::Integer);

        m.insert("date",VariableType::Date);

        m.insert("char",VariableType::String);
        m.insert("varchar",VariableType::String);
        m.insert("text",VariableType::String);
        m.insert("enum",VariableType::String);

        m.insert("float",VariableType::Float);
        m
    };
}

pub fn variable_type_from_str(s: &str) -> Result<VariableType, ()> {
    let op = VariableTypeMap.get(s);
    return match op {
        None => {
            Err(())
        }
        Some(e) => {
            Ok(e.clone())
        }
    };
}