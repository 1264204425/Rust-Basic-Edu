use std::ffi::CString;

fn main() {
    /*
        enum 枚举名称{
            variant1,
            variant2,
            ...
        }

        使用枚举
        枚举名称::variant

        Option经常用于函数的返回值,可以有返回值,也可以没有返回值
        enum Option<T>{
            Some(T), // 用于返回一个值
            None     // 用于换回null,用None来代替
        }

        match 判断枚举变量的值

        带数据类型的枚举{
        enum 枚举名称{
            variant1(数据类型1),
            variant2(数据类型2),
            ...
        }
        }
     */

    let level3 = RoadMap::Rust;
    println!("level3--{:?}", level3);

    let p = 666;
    let result = get_discount(p);
    println!("{:?}", result);

    print_road_map(RoadMap::Rust);
    print_road_map(RoadMap::GO);
    print_road_map(RoadMap::Python);

    let level3 = StudyRoadMap::Name(String::from("Rust"));
    match level3 {
        StudyRoadMap::Name(val) => {
            println!("{:?}", val);
        }
    }
}

enum StudyRoadMap {
    Name(String),
}

fn print_road_map(r: RoadMap) {
    match r {
        RoadMap::Rust => println!("Rust"),
        RoadMap::GO => println!("GO"),
        RoadMap::Python => println!("Python"),
    }
}

fn get_discount(price: i32) -> Option<bool> {
    if price > 100 {
        Some(true)
    } else {
        None
    }
}

#[derive(Debug)]
enum RoadMap {
    Rust,
    GO,
    Python,
}