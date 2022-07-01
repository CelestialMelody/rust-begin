fn main() {
    // Make it work with two ways
    {
        let v = {
            let mut x = 1;
            x += 2
        };

        assert_eq!(v, ());

        println!("Success!");
    }
    {
        let v = {
            let mut x = 1;
            // x + 2
            x += 2;
            x
        };

        assert_eq!(v, 3);

        println!("Success!");
    }

    {
        // let v = (let x = 3); // 这样写有错 目前版本认为let x = 3是一个语句，而不是表达式，不返回值;;let 一定要';' -> 语句
        let v = {
            let x = 3;
            x
        };
        println!("{}", v);
        assert!(v == 3);
    }
    {
        // let v = (let x = 3);
        let v = {
            let x = 3;
        };
        assert!(v == ());
    }

    {
        let s = sum(1, 2);
        assert_eq!(s, 3);
    }
}
fn sum(x: i32, y: i32) -> i32 {
    // return x + y;
    x + y
}
let v = {
    let x = 3;
};
assert!(v == ());