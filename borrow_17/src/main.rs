fn main() {
    // 借用 &变量名
    let study_list = vec!["Go", "Rust", "C++"];
    let study_list2 = study_list;
    show(&study_list2);
    println!("study_list2:{:?}", study_list2);

    let mut study_list3 = vec!["Go", "Rust", "C++"];
    println!("study_list3:{:?}", study_list3);
    show2(&mut study_list3);
    println!("study_list3:{:?}", study_list3);
    // 借用borrow 从一个函数中的变量传递给另外一个函数作为暂时使用
    // 函数离开后将所有权返回给当初传递的变量
    // 可变借用, 定义的时候和使用的时候,都要使用 &mut
}

fn show(v: &Vec<&str>) {
    println!("v = {:?}", v);
}

fn show2(v: &mut Vec<&str>) {
    v[0] = "Study finished";
    println!("v = {:?}", v)
}