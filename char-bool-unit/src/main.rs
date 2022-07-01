use std::mem::size_of_val;
fn main() {
    // char
    //  修改2处 `assert_eq!` 让代码工作
    {
        let c1: char = 'a';
        assert_eq!(size_of_val(&c1), 4); // all 4 byte

        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);

        println!("Success!")
    }
    //  修改一行让代码正常打印
    {
        // let c1 = "中"; // &str
        let c1 = '中';
        print_char(c1);
    }

    // 使成功打印
    {
        let _f: bool = false;

        let t = false;
        if !t {
            println!("Success!")
        }
    }

    {
        let f = true;
        let t = true && false;
        let t = true || false;
        assert_eq!(t, f);

        println!("Success!")
    }

    // 让代码工作，但不要修改 `implicitly_ret_unit` !
    {
        let _v: () = ();

        // let v = (2, 3);
        let v = ();
        assert_eq!(v, implicitly_ret_unit());

        println!("Success!")
    }

    // 让代码工作：修改 `assert!` 中的 `4`
    // 可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。
    // 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。
    {
        let unit: () = ();
        // assert!(size_of_val(&unit) == 4);
        assert!(size_of_val(&unit) == 0);

        println!("Success!")
    }
}

fn print_char(c: char) {
    println!("{}", c);
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
