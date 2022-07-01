fn main() {
    {
        // 不要修改下面两行代码!
        let (x, y) = (1, 2);
        let s = sum(x, y);

        assert_eq!(s, 3);
    }

    {
        print();
    }
    // 用两种方法求解
    {
        // never_return();
    }

    {
        // 填空
        let b = false;
        // let b = true;

        let v = match b {
            true => 1,
            // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
            false => {
                println!("Success!");
                panic!("we have no value for `false`, but we can panic")
            }
        };

        println!("{}", v);

        println!("Excercise Failed if printing out this line!");
    }
}

fn sum(x: i32, y: i32) -> i32 {
    // x y 类型必须说明
    x + y
}
// 使用另一个类型来替代 i32
// fn print() -> i32 {
fn print() {
    println!("hello,world");
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    // loop {}
    // panic!("I return nothing!")
    unimplemented!()
}

// use std::thread;
// use std::time;

// fn never_return() -> ! {
//     // implement this function, don't modify fn signatures
//     loop {
//         println!("I return nothing");
//         // sleeping for 1 second to avoid exhausting the cpu resoucre
//         thread::sleep(time::Duration::from_secs(1))
//     }
// }

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    // 1
    unimplemented!()
    // // 2
    // panic!()
    // // 3
    // loop {
    //     std::thread::sleep(std::time::Duration::from_secs(1))
    // }
}
