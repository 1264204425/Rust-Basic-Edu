fn main() {
    // const 常量名称：数据类型 = 值
    // 常量名称全部大写
    // 常量可以在任何地方定义,常量只是一个符号,编译时会被替换成对应的值
    // static 具有静态生命周期的变量 ‘static 可以是可变的变量
    const PI: f64 = 3.1415926;
    println!("PI:{}", PI);

    let name = "rust";
    let name = "rust-lang";
    println!("name:{}", name);

    // 隐藏变量并且改变数据类型
    let price = 188;
    let price = "288";
    println!("price:{}", price);

    // 常量不能被隐藏，也不能被重复定义
    // const DISCOUNT: f64 = 0.8;
    const DISCOUNT: f64 = 0.9;

    static BOOK: &'static str = "Rust 编程";
    println!("BOOK:{}", BOOK);
}
