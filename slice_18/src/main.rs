fn main() {
    /*
        let 切片变量=&变量[起始位置..结束位置] 左闭右开
     */
    let mut v = Vec::new();
    v.push("Rust");
    v.push("Go");
    v.push("Python");
    println!("len:{}", v.len());
    let s1 = &v[..2];
    // [..]获取全部元素
    // [起始位置..] 获取起始位置 到 整个容器的元素
    // [..结束位置] 获取0到结束位置的元素
    println!("s1:{:?}", s1);
    show_slice(s1);
    println!("s1:{:?}", s1);

    let mut v2 = Vec::new();
    v2.push("Rust");
    v2.push("Go");
    v2.push("Python");
    println!("modify_slice 之前的 v2:{:?}", v2);
    modify_slice(&mut v2[1..3]);
    println!("modify_slice 之后的 v2:{:?}", v2);
}

fn show_slice(s: &[&str]) {
    println!("show_slice函数内:{:?}", s);
}

fn modify_slice(s: &mut [&str]) {
    s[0] = "Study End";
    println!("modify_slice:{:?}", s);
}