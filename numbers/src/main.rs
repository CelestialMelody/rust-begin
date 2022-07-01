use std::ops::{Range, RangeInclusive};

fn main() {
    // 1 integer

    // Remove something to make it work
    //     {
    //         let x: i32 = 5;
    //         let mut y: u32 = 5;
    //
    //         y = x;
    //
    //         let z = 10; // Type of z ?
    //
    //         println!("Success!");
    //     }

    {
        let x: i32 = 5;
        let mut y: i32 = 5;

        y = x;

        let z = 10; // Type of z ?

        println!("Success!");
    }

    //  Fill the blank
    // {
    //     let v: u16 = 38_u8 as __;
    //
    //     println!("Success!");
    // }
    {
        let v: u16 = 38_u8 as u16;

        println!("Success!");
    }

    //  修改 `assert_eq!` 让代码工作
    // {
    //     let x = 5;
    //     assert_eq!("u32".to_string(), type_of(&x));
    // }

    {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));
    }

    // 填空，让代码工作
    // {
    //     assert_eq!(i8::MAX, __);
    //     assert_eq!(u8::MAX, __);
    // }
    {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
    }

    // 解决代码中的错误和 `panic`
    // {
    //     let v1 = 251_u8 + 8;
    //     let v2 = i8::checked_add(251, 8).unwrap();
    //     println!("{},{}", v1, v2);
    // }
    {
        let v1 = 247_u8 + 8;
        let v2 = i8::checked_add(119, 8).unwrap();
        println!("{},{}", v1, v2);
    }

    // 修改 `assert!` 让代码工作
    // {
    //     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    //     assert!(v == 1579);
    // }
    {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024+255+63+255 32bit 0xff 前面补0
        assert!(v == 1597);
        let a = 1024 + 0xff;
        println!("{}", a);
    }

    // 浮点数
    // 将 ? 替换成你的答案
    // {
    //     let x = 1_000.000_1; // ?
    //     let y: f32 = 0.12; // f32
    //     let z = 0.01_f64; // f64
    // }
    {
        let x = 1_000.000_1; // f64
        let y: f32 = 0.12; // f32
        let z = 0.01_f64; // f64
    }

    // {
    //     assert!(0.1 + 0.2 == 0.3);
    // }
    {
        assert!((0.1_f32 + 0.2 - 0.3).abs() < 0.000_1);
        assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    }

    // 3 序列range
    // 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
    // {
    //     let mut sum = 0;
    //     for i in -3..2 {
    //         sum += i
    //     }

    //     assert!(sum == -3);

    //     for c in 'a'..='z' {
    //         println!("{}", c);
    //     }
    // }
    {
        let mut sum = 0;
        for i in -3..2 {
            sum += i
        }

        assert!(sum == -5);

        for c in 'a'..='z' {
            println!("{}", c as u8);
        }
    }

    // 填空
    // {
    //     assert_eq!((1..__), Range { start: 1, end: 5 });
    //     assert_eq!((1..__), RangeInclusive::new(1, 5));
    // }
    {
        assert_eq!((1..5), Range { start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
    }

    // 填空，并解决错误
    //     {
    //         // 整数加法
    //         assert!(1u32 + 2 == __);

    //         // 整数减法
    //         assert!(1i32 - 2 == __);
    //         assert!(1u8 - 2 == -1);

    //         assert!(3 * 50 == __);

    //         assert!(9.6 / 3.2 == 3.0); // error ! 修改它让代码工作

    //         assert!(24 % 5 == __);

    //         // 逻辑与或非操作
    //         assert!(true && false == __);
    //         assert!(true || false == __);
    //         assert!(!true == __);

    //         // 位操作
    //         println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    //         println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    //         println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //         println!("1 << 5 is {}", 1u32 << 5);
    //         println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    //     }

    {
        // 整数加法
        assert!(1u32 + 2 == 3);

        // 整数减法
        assert!(1i32 - 2 == -1);
        assert!(1i8 - 2 == -1);

        assert!(3 * 50 == 150);

        assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 0.001); // error ! 修改它让代码工作

        assert!(24 % 5 == 4);

        // 逻辑与或非操作
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);

        // 位操作
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
