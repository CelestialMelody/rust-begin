fn main() {
    // 1.绑定和可变性
    // 修复下面代码的错误并尽可能少的修改
    // {
    //     let x: i32; // 未初始化，但被使用
    //     let y: i32; // 未初始化，也未被使用
    //     println!("x is equal to {}", x);
    // }
    {
        let x: i32 = 0; // 未初始化，但被使用
        let _y: i32; // 未初始化，也未被使用
        println!("x is equal to {}", x);
    }

    // 完形填空，让代码编译
    // {
    //     let __ =  1;
    //     __ += 2;
    //
    //     println!("x = {}", x);
    // }
    {
        let mut x = 1;
        x += 2;

        println!("x = {}", x);
    }

    // 2.变量作用域
    // 修复下面代码的错误并使用尽可能少的改变
    // {
    //     let x: i32 = 10;
    //     {
    //         let y: i32 = 5;
    //         println!("x 的值是 {}, y 的值是 {}", x, y);
    //     }
    //     println!("x 的值是 {}, y 的值是 {}", x, y);
    // }
    {
        let x: i32 = 10;
        {
            let y: i32 = 5;
            println!("x 的值是 {}, y 的值是 {}", x, y);
        }
        println!("x 的值是 {}", x);
    }
    // 修复错误
    // {
    //     println!("{}, world", x);
    // }
    {
        let x = define_x();
        println!("{}, world", x);

        let x = define_x_();
        println!("{:?}, world", x);
    }

    // 3. 变量遮蔽( Shadowing )

    // 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
    // {
    //     let x: i32 = 5;
    //     {
    //         let x = 12;
    //         assert_eq!(x, 5);
    //     }
    //
    //     assert_eq!(x, 12);
    //
    //     let x = 42;
    //     println!("{}", x); // 输出 "42".
    // }
    {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // 输出 "42".
    }

    // 删除一行代码以通过编译
    // {
    //     let mut x: i32 = 1;
    //     x = 7;
    //     // 遮蔽且再次绑定
    //     let x = x;
    //     x += 3;
    //
    //
    //     let y = 4;
    //     // 遮蔽
    //     let y = "I can also be bound to text!";
    // }
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    {
        let mut x: i32 = 1;
        x = 7;
        // 遮蔽且再次绑定
        // let x = x;
        x += 3;

        let y = 4;
        // 遮蔽
        let y = "I can also be bound to text!";
    }

    // 4. 未使用的变量
    // 修复编译器输出的 warning
    // {
    //     let x = 1;
    // }
    {
        #[allow(unused_variables)]
        let x = 1;
    }
    {
        let _x = 1;
    }

    // 5.变量解构
    //修复下面代码的错误并尽可能少的修改
    // {
    //     let (x, y) = (1, 2);
    //     x += 2;
    //
    //     assert_eq!(x, 3);
    //     assert_eq!(y, 2);
    // }
    {
        let (mut x, y) = (1, 2);
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }

    // 6.解构式赋值
    // {
    //     let (x, y);
    //     (x,..) = (3, 4);
    //     [.., y] = [1, 2];
    //     // 填空，让代码工作
    //     assert_eq!([x,y], __);
    // }
    {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];
        // 填空，让代码工作
        assert_eq!([x,y], [3,2]);
    }
}

// fn define_x() {
//     let x = "hello";
// }

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

fn define_x_() -> &'static str {
    let x = "hello";
    x
}
