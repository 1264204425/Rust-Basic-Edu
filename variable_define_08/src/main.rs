fn main() {
    /*
        定义变量的格式
        let 变量名=值；不指定变量类型
        let 变量名：数据类型=值； 指定变量类型
    */

    let study = "";
    println!("study:{}", study);

    // 可变变量
    // mut 变量名：数据类型=值；
    let mut _price = 188;
    _price = 288;
    println!("price:{}", _price);
}
