fn main() {
    /*
        fn function缩写
        fn 函数名称([参数:参数类型]) -> 返回值{
            代码执行逻辑
        }

        无明确返回的时候,就会返回一个单元类型()
    */
    hello();
    println!("{}", get_name());
    println!("{}", get_name2());

    let price = 99;
    double_price(price);
    println!("外部price:{}", price);

    let mut price = 88;
    double_price2(&mut price);
    println!("外部price:{}", price);

    let name = String::from("Rust");
    show_name(name);
    // println!("调用show_name函数后{}", name)
}

fn hello() {
    println!("Hello Rust!")
}

fn get_name() -> String {
    return String::from("Rust");
}

// 没有return关键字 函数会默认使用最后一条的语句的返回值,并且数据类型必须一致
fn get_name2() -> String {
    String::from("GO")
}

// 值传递, 函数内部和外部各保存了相同的值,互不影响,因此内外结果不一致
fn double_price(mut price: i32) {
    price = price * 2;
    println!("内部price:{}", price);
}

// &符号用于创建引用（Reference）,引用允许你在不转移所有权的情况下访问值
// 引用传递, 把当前变量的内存地址传递给函数,函数内部和外部共享相同的值,因此内外结果一致
// *解引用符,用于获取访问变量price指向内存地址上存储变量的值
fn double_price2(price: &mut i32) {
    *price = *price * 2;
    println!("内部price:{}", price);
}

fn show_name(name: String) {
    println!("language:{}", name);
}