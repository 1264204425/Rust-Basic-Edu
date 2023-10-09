fn main() {
    let total: f32 = 666.00;
    /*
        if ...{
        }
     */
    if total > 500.00 {
        println!("打8折，总价为：{}", total * 0.8);
    }
    /*
        if ...{
        }else{
        }
     */
    let total: f32 = 166.00;
    if total > 500.00 {
        println!("打8折，总价为：{}", total * 0.8);
    } else {
        println!("不打折，总价为：{}", total);
    }
    /*
        if...else if...else
        if 条件表达式1{
        }
        else if 条件表达式2{
        }
        else{
        }
     */
    let total: f32 = 366.00;
    if total > 200.00 && total < 500.00 {
        println!("打9折,{}", total * 0.9);
    } else if total > 500.00 {
        println!("打8折,{}", total * 0.8);
    } else {
        print!("无优惠折扣{}", total);
    }

    let code = "10010";
    let choose = match code {
        "10010" => "联通",
        "10086" => "移动",
        _ => "Unknown",
    };
    println!("选择{}", choose);
}
