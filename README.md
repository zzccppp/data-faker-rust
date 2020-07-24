# Data Faker
一个用Rust编写的制造测试数据的小工具
灵感来源于https://github.com/gangly/datafaker

## 配置文件基本语法
变量名||输出类型||生成规则

## 目前正在编写的生成规则

### 1.increase
用于生成一串数列

用法: increase(start,step)

start:整数，step：整数

可选输出类型：int,string

### 2.enum
用于随机从枚举的量中抽取数据

用法：enum(e1,e2,e3,e4)

可选输出类型：int,float,string

### 3.enum_file
用于从文件中抽取数据随机枚举

用法: enum(file_path)

文件中的数据换行隔开，忽略空行

可选输出类型：int,float,string

还在编写中.....
