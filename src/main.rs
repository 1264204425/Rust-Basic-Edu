use std::io::Write;

fn main() {
    /*
        std::io::stdin() 返回标准输入流stdin的句柄
        read_line() stdin的句柄的一个方法,从标准输入流中读取输入一行数据,返回一个Result枚举.
        unwrap()是一个帮助的方法,简化恢复错误的处理,返回result中存储实际值
     */
    // let mut in_word = String::new();
    // // 标准库（std::io）进行标准输入（stdin）的读取操作
    // let result = std::io::stdin().read_line(&mut in_word).unwrap();
    // println!("您输入的是:{}", in_word);
    // println!("您读取的字节数为:{}", result);

    let result1 = std::io::stdout().write("learn Rust".as_bytes()).unwrap();
    println!("写入的字节数为:{:?}\n", result1);
    let result2 = std::io::stdout().write("www.google.com".as_bytes()).unwrap();
    println!("写入字节数为:{:?}\n", result2);
    /*
        std::io::stdout()返回标准输出流的句柄
        write()是标准输出流stdout句柄上的一个方法,用于向标准输出流中写入字节流内容,也放回一个Result枚举
        不会输出结束时自动追加换行符\n
     */

    let input_args = std::env::args();
    for arg in input_args {
        println!("命令行参数:{}", arg);
    }
}




