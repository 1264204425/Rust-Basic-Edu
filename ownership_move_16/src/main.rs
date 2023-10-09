fn main() {
    let _name = "Joe";

    // move
    // let a=b
    // foo(a)
    /*
        栈 后进先出 类型大小是固定的 如i32
        堆 编译时大小未知或不确定大小 用户自己管理 增加内存溢出风险
     */
    let a = 88;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // vec! 是一个宏（macro），用于创建一个新的 Vec（向量）对象。
    // Vec 是Rust标准库中的一种动态数组类型，可以存储多个相同类型的元素。
    // 这么写是不对的需要使用let v2 = v1.clone();
    // let v1 = vec!["hi", "hello", "world"];
    // let v2 = v1;
    // println!("{:?}", v1);

    let study_list = vec!["Rust", "Go", "Python"];
    let study_list2 = study_list;
    show(study_list2);
    // println!("study_list2 {:?}", study_list2)

    let study_list3 = vec!["Rust", "Go", "Python"];
    let study_list4 = study_list3;
    let result = show2(study_list4);
    println!("{:?}", result);
}

fn show2(v: Vec<&str>) -> Vec<&str> {
    println!("v:{:?}", v);
    return v;
}

fn show(v: Vec<&str>) {
    println!("v:{:?}", v)
}