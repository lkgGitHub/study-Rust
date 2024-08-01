use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Rng 是一个 trait
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //  loop /luːp/ 环形 loop 关键字创建了一个无限循环。
    loop {
        println!("Please input your guess.");
        // :: 语法表明 new 是 String 类型的一个关联函数（associated function）
        // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为静态方法（static method）。
        let mut guess = String::new();

        // & 表示这个参数是一个引用（reference）
        // read_line 返回 -> io::Result<usize>  Result 是一种枚举类型，通常也写作 enum。
        // Result 的成员是 Ok 和 Err。Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。
        io::stdin().read_line(&mut guess).
            expect("Failed to read line");

        // Rust 允许用一个新值来隐藏 （Shadowing） guess 之前的值
        // guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // _ 是一个通配符值
        };

        // {} 是预留在特定位置的占位符
        println!("You guessed:{}", guess);

        // cmp 方法用来比较两个值并可以在任何可比较的值上调用
        match guess.cmp(&secret_number) {
            // Ordering 也是一个枚举。成员是 Less、Greater 和 Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }

}
