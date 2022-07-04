fn main() {
    {
        // 修复错误
        enum Number {
            Zero,
            One,
            Two,
        }

        enum Number1 {
            Zero = 0,
            One,
            Two,
        }

        // C语言风格的枚举定义
        enum Number2 {
            Zero = 0,
            One = 1,
            Two = 2,
        }

        println!(
            "{}, {}, {}",
            Number::Zero as i32,
            Number::One as i32,
            Number::Two as i32
        );
        println!(
            "{}, {}, {}",
            Number1::Zero as i32,
            Number1::One as i32,
            Number1::Two as i32
        );
        println!(
            "{}, {}, {}",
            Number2::Zero as i32,
            Number2::One as i32,
            Number2::Two as i32
        );

        // 通过 `as` 可以将枚举值强转为整数类型
        assert_eq!(Number::One as i32, Number1::One as i32);
        assert_eq!(Number1::One as u32, Number2::One as u32);

        test_type(Number::One);
        println!("{}", Number::One as i32); // 整个枚举类型可以被强转为整数类型
    }
    {
        // 填空
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
        let msg2 = Message::Write("hello world!".to_string()); // 使用 "hello, world!" 来初始化

        test_type(msg1);
        test_type(Message::Quit);

        // test_type(Message::Quit as i32); // wrong

        // test_type(Message::Move);  // wrong

        test_type(Message::Move { x: 1, y: 2 });

        // test_type(Message::Move { x: i32, y: i32 }); // wrong 不是实例

        test_type(Message::Write);
        test_type(Message::Write("hello world!".to_string()));

        // println!("{}", Message::Write("hello world!".to_string())); // wrong

        // println!("{}", Message::Quit as i32);
        // 区别 上面number::one; 问题是non-primitive cast: `Message` as `i32`
        // Message 整个类型不可以转化为某种类型

        test_type(Message::ChangeColor); // 枚举成员本身枚举类型---Message
        test_type(Message::ChangeColor(1, 2, 3)); // 实例化后是一种类型 -- Message::ChangeColor::{{constructor}}(i32, i32, i32)
    }
    {
        enum Char {
            // c1 = 'a' as i8, // 只能是isize类型
            // c1 = 'a' as i64, //
            c1 = 'a' as isize,
            c2,
            c3,
        }
        let c1 = Char::c1;
        let c2 = Char::c2;
        let c3 = Char::c3;
        println!("{}", c1 as i8); // 这里可以转换为i8

        // println!("{}", c2); wrong

        // test_type(c1);// c1 不是基本类型 所有权转移

        test_type(c2);
    }

    {
        let x = String::from("hello");
        let ref y = x;
        let ref z = &x;
        let yy = &x;
        let zz = &z;

        println!("{}, {}, {} , {}, {}", x, y, z, yy, zz); // 都是hello

        // test_type(x); // x已经借用；test_type参数非不可变引用
        test_type(y);
        test_type(z);
        test_type(yy);
        test_type(zz);
    }

    {
        // 仅填空并修复错误
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::Move { x: 1, y: 1 };

        if let Message::Move { x: a, y: b } = msg {
            assert_eq!(a, b);
        } else {
            panic!("不要让这行代码运行！");
        }
    }
}

use std::any::type_name;

fn test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}
