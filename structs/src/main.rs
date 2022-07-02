use std::fmt::Debug;

fn main() {
    // fix the error
    {
        struct Person {
            name: String,
            age: u8,
            hobby: String,
        }
        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("programming"),
        };
    }
    {
        struct Unit;
        trait SomeTrait {
            // ...定义一些行为
        }

        // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
        // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
        impl SomeTrait for Unit {}
        let u = Unit;
        do_something_with_unit(u);

        // 填空，让代码工作
        fn do_something_with_unit(u: Unit) {}
    }
    {
        // 填空并修复错误
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let v = Color(0, 127, 255);
        check_color(v);

        fn check_color(p: Color) {
            let Color(x, _, _) = p;
            assert_eq!(x, 0);
            assert_eq!(p.1, 127);
            assert_eq!(p.2, 255);
        }
    }
    {
        // 填空并修复错误，不要增加或移除代码行
        struct Person {
            name: String,
            age: u8,
        }
        {
            let age = 18;
            let mut p = Person {
                name: String::from("sunface"),
                age,
            };

            // how can you believe sunface is only 18?
            p.age = 30;

            // 填空
            p.name = String::from("sunfei");
        }
    }
    {
        // 填空
        struct Person {
            name: String,
            age: u8,
        }

        fn build_person(name: String, age: u8) -> Person {
            Person { age, name }
        }
    }
    {
        // 填空，让代码工作
        #[derive(Debug)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64, // 有逗号
        }
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);

        let u3 = User {
            email: String::from("melody@example.com"),
            username: u2.email.clone(), // u2 email 所有权并未转移
            ..u2
        };
        fn set_email(u: User) -> User {
            // u1所有权给u
            User {
                email: String::from("contact@im.dev"),
                // username: u.username.clone(),// 没必要clone，u的作用域在函数结束时结束, 所有权没有返回给u1
                ..u // 没有，
            }
        }
        // println!("{:#?}", u1); // 使用set_email时所有权转移给u, 即使是bool 和 u64，也会转移 由于作用域与所有权转移问题 无法访问u1
        println!("{:#?}", u2);
        println!("{:#?}", u3);

        let u4 = User { ..u3 };
        println!("{:#?}", u4);
        // println!("{:#?}", u3); // String所有权转移\
        println!("{}, {}", u3.active, u3.sign_in_count);
    }
    {
        // 填空，让代码工作
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
            height: 50,
        };

        dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

        println!("{:#?}", rect1); // 打印 debug 信息到标准输出 stdout
    }
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
        // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
        let Person { name, ref age } = person; // 这里age  &Box<u8>; 注意要写Person

        println!("The person's age is {}", age);

        println!("The person's name is {}", name);

        // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
        //println!("The person struct is {:?}", person);

        // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
        println!("The person's age from person struct is {}", person.age);
    }
    {
        // 修复错误
        #[derive(Debug)]
        struct File {
            name: String,
            data: String,
        }
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string(),
        };

        // 或者
        // let _name = f.name.clone();
        let _name = f.name;

        // 只能修改这一行
        println!("{}", f.data);
    }
}
