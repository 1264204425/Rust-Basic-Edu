fn main() {

    // 有符号类型signed可以存储正数或负数，无符号类型 usigned只能存储正数。
    // 按存储控件来说 划分为1字节 2字节 4字节 8字节 16字节 32字节 64字节
    // 1个字节8位 1位表示1个二进制位 1个二进制位表示0或1
    let price = 100;
    let price2:u32 = 200;
    let price3:i32 = -300;
    let price4:isize = 400;
    let price5:usize = 500;

    // 错误的写法
    // let  price6:i32 = 66.66;
    // let price7:i8 = 192;
}