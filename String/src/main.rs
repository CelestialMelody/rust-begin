fn main() {
    // 修复错误，不要新增代码行
    // 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
    {
        // let s: str = "hello, world";
        let s: &str = "hello, world";
    }
    //  如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
    // 使用至少两种方法来修复错误
    {
        // let s: Box<str> = "hello, world".into();
        // // greetings(s)
        // greetings(&s)

        let s: Box<&str> = "hello, world".into();
        // greetings(&s) // ? 也可以
        greetings(*s);
    }

    // 填空
    {
        let mut s = String::from("");
        s.push_str("hello, world");
        s.push('!');

        assert_eq!(s, "hello, world!");
    }
}
fn greetings(s: &str) {
    println!("{}", s)
}
