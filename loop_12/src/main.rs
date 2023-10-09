fn main() {
    /*
     for 临时变量 in 左区间..右区间
     */
    for num in 1..5 {
        println!("num is {}", num);
    }

    for num in 1..=5 {
        println!("num is {}", num);
    }

    let study_list = vec![
        "Python",
        "C",
        "Rust",
    ];
    // iter()这个方法每次迭代是借用集合中的一个元素,元素本身不会改变,循环之后还可以使用
    for name in study_list.iter() {
        match name {
            &"Rust" => println!("Rust is a systems programming language"),
            _ => println!("I don't know about {}", name)
        }
    }
    let study_list2 = vec![
        "Python",
        "C",
        "Rust",
    ];
    // into_iter 会消耗集合,每次迭代;集合中的数据本身被提供,一旦集合被消耗完了
    // 之后无法被使用 因为他已经在循环中被move了
    for name in study_list2.into_iter() {
        match name {
            "Rust" => println!("Rust is a systems programming language"),
            _ => println!("I don't know about {}", name)
        }
    }
    let mut study_list3 = vec![
        "Python",
        "C",
        "Rust",
    ];
    // iter_mut 可变地借用集合中的每个元素, 从而允许集合被就地修改
    // 之后无法被使用 因为他已经在循环中被move了
    for name in study_list3.iter_mut() {
        *name = match name {
            &mut "Rust" => { "Rust is a systems programming language" }
            _ => *name,
        }
    }
    println!("studyList3:{:?}", study_list3);

    let mut num = 1;
    while num < 20 {
        println!("num is {}", num);
        num = num * 2
    }

    let mut num = 1;
    loop {
        if num > 20 {
            println!("stop");
            break;
        }
        println!("num is {}", num);
        num = num * 3;
    }
}
