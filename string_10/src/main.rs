fn main() {
    // &str 是在模块 std:str, 字符串切片
    let lesson = "Introduction";

    /*
        字符串对象
        Sting::new() 创建空字符串,他是静态的
        String::from("Hello") 从字符串字面量创建字符串对象
     */
    let s1 = String::new();
    println!("s1 = {}, s1-len = {}", s1, s1.len());

    let s2 = String::from("Hello");
    println!("s2 = {}, s2-len = {}", s2, s2.len());

    let mut s3 = String::new();
    s3.push_str("Hello");
    println!("s3 = {}, s3-len = {}", s3, s3.len());

    // push追加字符
    s3.push('1');
    s3.push('2');

    // 字符串切片
    let s4 = String::from("Hello");
    let result = s4.replace("l", "x");
    println!("result = {}", result);

    let s5 = String::from("学习Rust");
    println!("len = {}", s5.len());

    let s6 = "学习Rust".to_string();

    let s7 = String::from("Rust");
    show_name(s7.as_str());

    // 去掉头尾空白符， 制表符\t 换行符\r\n 空格 回车\r
    let s8 = "\tRust\tGo\r\nPython\r\n           ";
    println!("len = {}", s8.len());
    println!("len = {}", s8.trim().len());
    println!("{}", s8);

    let s9 = "在线翻译,桌面端,免费下载,有道速读,arXiv论文,翻译作文";
    for item in s9.split(',') {
        println!("item = {}", item);
    }

    // let s10 = "在线翻译,桌面端,免费下载,有道速读,arXiv论文,翻译作文";
    // for c in s10.chars() {
    //     println!("c = {}", c);
    // }

    let s11 = "123".to_string();
    let s12 = "456".to_string();
    let result = s11 + &s12;
    println!("result = {}", result);
}

fn show_name(name: &str) {
    println!("name = {}", name);
}

