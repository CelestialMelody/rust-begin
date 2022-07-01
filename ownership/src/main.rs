fn main() {
    // Use as many approaches as you can to make it work
    {
        // 所有权转移
        let x = String::from("hello, world");
        let y = x;
        println!("{}", y);
    }
    {
        // 深拷贝
        let x = String::from("hello, world");
        let y = x.clone();
        println!("{},{}", x, y);
    }
    {
        // 基本类型 栈分配
        let x = "hello, world";
        let y = x;
        println!("{},{}", x, y);
    }
    {
        // 不可变引用&
        let x = &String::from("hello, world");
        let y = x;
        println!("{},{}", x, y);
    }
    // 不要修改下面的代码
    {
        let s1 = String::from("hello, world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }

    {
        let s = give_ownership();
        println!("{}", s);
    }
    // 修复错误，不要删除任何代码行
    {
        let s = String::from("hello, world");

        // print_str(s);
        print_str(s.clone());

        println!("{}", s);
    }
    // 不要使用 clone，使用 copy 的方式替代
    {
        // let x = (1, 2, (), "hello".to_string());
        // let y = x.clone();
        let x = (1, 2, (), "hello");
        let y = x;
        println!("{:?}, {:?}", x, y);
    }

    // 可变性 当所有权转移时，可变性也可以随之改变。
    {
        let s = String::from("hello, ");

        // 只修改下面这行代码 !
        let mut s1 = s;

        s1.push_str("world")
    }

    {
        let x = Box::new(5);

        // let ...      // 完成该行代码，不要修改其它行！
        let mut y = Box::new(5);
        *y = 4;

        assert_eq!(*x, 5);
    }

    {
        let t = (String::from("hello"), String::from("world"));

        let _s = t.0;

        // 仅修改下面这行代码，且不要使用 `_s`
        println!("{:?}", t.1);
    }

    {
        let t = (String::from("hello"), String::from("world"));

        // 填空，不要修改其它代码
        // let (__, __) = __;
        let (s1, s2) = t.clone();

        println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    }
}

fn print_str(s: String) {
    println!("{}", s);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型

    // String 本质是堆上面的 Vec，Vec 的切片引用就是 &[]
    // 所以 &str 和 String，等同于 &[u8] 和 Vec
    // 那么 as_bytes 就是拿到字符串的切片引用 &[u8]，s 本身的所有权没有发生变化
    // 而 into_bytes() 是拿到字符串的 Vec，然后发生了 move 动作，所有权转移了

    // let _s = s.into_bytes(); // move 所有权转移
    let _s = s.as_bytes(); // 借用
    s
}
