use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use crate::configuration::definitions::{VariableType, GenerateRule, construct_from_str, OutPutType, OutPutTypeValue, VariableTypeValue};
use crate::rules::IncreaseRule;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;
use crate::configuration::definitions::OutPutTypeValue::Json;
use std::collections::HashMap;
use serde_json::json;

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

    pub fn generate(&self, tt: OutPutType) -> Result<OutPutTypeValue, ()> {
        return match tt {
            OutPutType::Json => {
                let mut map = serde_json::Map::<String, serde_json::Value>::new();

                for n in &self.items {
                    match n.var_type {
                        VariableType::Integer => {
                            let v = n.rule.generate_into(VariableType::Integer)?;
                            if let VariableTypeValue::Integer(u) = v {
                                map.insert(n.var_name.clone(), json!(u));
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::Float => {
                            let v = n.rule.generate_into(VariableType::Float)?;
                            if let VariableTypeValue::Float(u) = v {
                                map.insert(n.var_name.clone(), json!(u));
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::String => {
                            let v = n.rule.generate_into(VariableType::String)?;
                            if let VariableTypeValue::String(u) = v {
                                map.insert(n.var_name.clone(), json!(u));
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::Date => {
                            let v = n.rule.generate_into(VariableType::Date)?;
                            if let VariableTypeValue::Date(u) = v {
                                let str = u.format("%Y-%m-%d %H:%M:%S").to_string();
                                map.insert(n.var_name.clone(), json!(str));
                            } else {
                                return Err(());
                            }
                        }
                    }
                }

                Ok(OutPutTypeValue::Json(serde_json::Value::Object(map)))
            }
            OutPutType::Csv => {
                let mut vec = Vec::<String>::new();

                for n in &self.items {
                    match n.var_type {
                        VariableType::Integer => {
                            let v = n.rule.generate_into(VariableType::Integer)?;
                            if let VariableTypeValue::Integer(u) = v {
                                vec.push(u.to_string());
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::Float => {
                            let v = n.rule.generate_into(VariableType::Float)?;
                            if let VariableTypeValue::Float(u) = v {
                                vec.push(u.to_string());
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::String => {
                            let v = n.rule.generate_into(VariableType::String)?;
                            if let VariableTypeValue::String(u) = v {
                                vec.push(u);
                            } else {
                                return Err(());
                            }
                        }
                        VariableType::Date => {
                            let v = n.rule.generate_into(VariableType::Date)?;
                            if let VariableTypeValue::Date(u) = v {
                                let str = u.format("%Y-%m-%d %H:%M:%S").to_string();
                                vec.push(str);
                            } else {
                                return Err(());
                            }
                        }
                    }
                }
                Ok(OutPutTypeValue::Csv(vec))
            }
        };
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
        m.insert("string",VariableType::String);

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