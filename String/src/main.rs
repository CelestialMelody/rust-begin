fn main() {
    // 修复错误，不要新增代码行
    // 正常情&况下我们无法使用 str 类型，但是可以使用 &str 来替代
    {
        let s: &str = "hello, world";
    }
    //  如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
    // 使用至少两种方法来修复错误
    {
        // let s: Box<str> = "hello, world".into();
        // greetings(&s)

        let s: Box<&str> = "hello, world".into();
        // greetings(&s) // ? 也可以
        greetings(*s);
    }

    // 填空
    {
        // let mut s = String::from("");
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');

        assert_eq!(s, "hello, world!");
    }
    // 填空
    {
        let s = String::from("I like dogs");
        // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
        let s1 = s.replace("dogs", "cats");

        // println!("{s}", s); //  wrong 所有权转移
        assert_eq!(s1, "I like cats")
    }

    // 修复所有错误，不要删除任何一行代码
    {
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2; // 防止s1所有权转移 clone; &s2 自动解引用为&str
        assert_eq!(s3, "hello,world!");
        println!("{}", s1);
    }

    // 使用至少两种方法来修复错误
    {
        fn greetings(s: String) {
            println!("{}", s)
        }

        let s = "hello, world".to_string();
        greetings(s);

        // let s = "hello, world".to_string();
        // greetings(&s); // 借用 所有权不转移
    }
    {
        let mut s = String::from("hello");

        {
            // 可变引用
            let r1 = &mut s;
            // println!("{}", s) // 若r1 后面不再使用 所有权归还 s仍然有所有权
            // println!("{}, {}", r1, s); // r1使用时 s将所有权借给r1 s 没有所有权 报错；
            // 另一方面也说明 可变引用（或者说可变的）同时只能存在一个
            println!("{}", r1); // 此后r1不再使用 引用作用域结束 s具有所有权
            println!("{}", s); //

            // 不可变引用
            let r2 = &s;
            // 这部分不太会解释, 有点不理解
            // 我好像只能用：两个都没有修改，只是读取，即没有-可变与不可变同时存在-来解释
            println!("{}, {}", s, r2); // r2 为不可变引用 允许使用值 s 与 r2; r2 应该也是借用了s的所有权 但是s也可以使用
            println!("{}, {}", &s, &r2); // 与上面的区别是&Strinng &&String（不知道与 &str &str 一个意思）, 上面是&String 与 String(不知道是不是与&str String一个意思) 但结果相同

            // s = "rust".to_string(); // 修改s
            // println!("{}, {}", s, r2); // 报错-可变与不可变同时存在

            let mut ss = String::from("hello, ");
            ss.push_str(&s); // 仍然可用
            ss.push_str(&r2);

            let sss = r2; // 不可变引用可拷贝

            // s.push_str(&r2); // 报错， 存在可变借用与不可变借用 `pub fn push_str(&mut self, string: &str)`类似于例子中的clear
            // s.push_str(", world"); // 报错

            let r3 = s; // s 所有权转移

            // println!("{}", s); // s 没有所有权 r3 不是借用 r3变量作用域仍然在
        } //r3作用域结束
    }

    // 使用两种方法来解决错误，不要新增代码行
    {
        let s = "hello, world".to_string();
        let s1: &str = &s;
    }

    {
        // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
        // 填空以输出 "I'm writing Rust"
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // 也可以使用 Unicode 形式的转义字符
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        );

        // 还能使用 \ 来连接多行字符串
        let long_string = "String literals \
                        can span multiple lines. \
                        The linebreak and indentation here \
                         can be escaped too!";
        println!("{}", long_string);
    }
    /* 填空并修复所有错误 */
    {
        let raw_str = "Escapes don't work here: \x3F \u{211D}";
        assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

        // 如果你希望在字符串中使用双引号，可以使用以下形式
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果希望在字符串中使用 # 号，可以如下使用：
        let delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", delimiter);

        // 填空
        let long_delimiter = r###"Hello, "##""###;
        assert_eq!(long_delimiter, "Hello, \"##\"")
    }
    // 你无法通过索引的方式去访问字符串中的某个字符，
    //但是可以使用切片的方式 &s1[start..end] ，但是start 和 end 必须准确落在字符的边界处.

    {
        let s1 = String::from("hi,中国");
        let h = &s1[..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
        assert_eq!(h, "h");

        let h1 = &s1[3..=5]; // 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
        assert_eq!(h1, "中");
    }
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        println!("{:?}, {:?}", a, slice);
    }

    {
        // 填空，打印出 "你好，世界" 中的每一个字符
        for c in "你好，世界".chars() {
            println!("{}", c)
        }
    }
}
fn greetings(s: &str) {
    println!("{}", s)
}
