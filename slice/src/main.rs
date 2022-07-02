fn main() {
    // 修复代码中的错误，不要新增代码行!
    {
        let arr = [1, 2, 3];
        // let s1: &[i32] = &arr[0..2];
        let s1 = &arr[0..2];

        let s2: &str = "hello, world";
    }

    {
        let arr: [char; 3] = ['中', '国', '人'];

        let slice = &arr[..2]; // 引用前两个元素， 每个元素大小是一个字 64位 一字8字节

        // 修改数字 `6` 让代码工作
        // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： 因为'中'和'国'是 UTF-8 字符，它们每个占用 3 个字节，2 个字符就是 6 个字节
        assert!(std::mem::size_of_val(&slice) == 16);

        println!("{}", std::mem::size_of_val(&'a'));
        println!("{}", std::mem::size_of_val(&arr));
        println!("{}", std::mem::size_of_val(&slice));
    }

    {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        // 填空让代码工作起来
        let slice: &[i32] = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }

    {
        let s = String::from("hello");

        let slice1 = &s[0..2];
        // 填空，不要再使用 0..2
        let slice2 = &s[..2];

        assert_eq!(slice1, slice2);
    }

    {
        let s = "你好，世界";
        // 修改以下代码行，让代码工作起来
        let slice = &s[0..3];

        assert!(slice == "你");
    }

    // 修复所有错误
    {
        let mut s = String::from("hello world");

        // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
        // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
        let word = first_word(&s);

        // s.clear(); // error!

        println!("the first word is: {}", word);
        s.clear(); // 放在不可变引用作用域后就行
    }
    fn first_word(s: &str) -> &str {
        &s[..1]
    }
}
