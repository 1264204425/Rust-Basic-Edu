use std::collections::{HashMap, HashSet};
use std::os::unix::fs::symlink;

fn main() {
    /*
        向量vector
        特点
        1 相同类型的元素集合
        2 长度可变,运行时增加减少都可以
        3 使用索引查找
        4 添加元素到队尾
        5 向量内存在堆上 长度可以动态变化

        创建向量
        Vec::new();
        vec![val1,val2,...]
     */

    let mut v = Vec::new();
    v.push("Rust学习");
    v.push("Golang学习");
    v.push("Python学习");
    println!("{:?}", v);
    println!("len:{}", v.len());

    let mut v2 = vec!["Rust学习", "Golang学习", "Python学习"];
    println!("{:?}", v2);

    let x = v2.remove(0);
    println!("x:{}", x);
    println!("{:?}", v2);

    if v.contains(&"Golang学习") {
        println!("包含");
    } else {
        println!("不包含");
    }

    let y = v[0];
    for item in v {
        println!("now is {}", item);
    }

    /*
        Hashmap是键值对的集合,键是不能重复的,值可以重复.显示导入std::collections
        let mut 变量名称=HashMap::new();
     */

    let mut process = HashMap::new();
    process.insert("Chrome", 1);
    process.insert("Firefox", 2);
    process.insert("Edge", 3);

    println!("{:?}", process);
    println!("len {}", process.len());

    match process.get(&"Chrome") {
        Some(v) => {
            println!("HashMap v:{}", v);
        }
        None => {
            println!("None");
        }
    }

    for (k, v) in process.iter() {
        println!("k;{} vL{}", k, v);
    }

    if process.contains_key(&"Chrome") {
        println!("find it");
    }

    let x = process.remove(&"remove");
    println!("{:?}", x);
    println!("{:?}", process);

    /*
        HashSet 相同数据类型的集合,没有重复的值.如果存在相同的值,插入或失败.
        let mut 变量名称=HashSet::new();
     */

    let mut studySet = HashSet::new();
    studySet.insert("Chrome");
    studySet.insert("Firefox");
    studySet.insert("Edge");

    println!("{:?}", studySet);
    studySet.insert("Edge");
    println!("{:?}", studySet);
    println!("len:{:?}", studySet.len());

    for item in studySet.iter() {
        println!("item:{}", item);
    }

    match studySet.get(&"Chrome") {
        None => {
            println!("not found")
        }
        Some(data) => {
            println!("data:{}", data);
        }
    }

    if studySet.contains("Edge") {
        println!("存在");
    }

    studySet.remove("Firefox");
    println!("{:?}", studySet);
}