fn main() {
    /*
        let tuple变量名称:(数据类型1，数据类型2,.....) = (数据1，数据2,...)
        let tuple变量名称 = (数据1，数据2,.....)
     */
    let t = ("one", "two");
    println!("{:?}", t);
    // 元祖变量,索引数字
    println!("t.0 = {}", t.0);
    show_tuple(t);

    let (first, second) = t;
    println!("{}", first);
    println!("{}", second);
}

fn show_tuple(t: (&str, &str)) {
    println!("{:?}", t);
}