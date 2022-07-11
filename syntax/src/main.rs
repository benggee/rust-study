use std::mem;
use std::thread;

// ============= struct ================
// 元组结构
struct Pair(i32, f32);

// 标准C结构
#[derive(Debug)]   // 编译语法
struct Person {
    name: String,
    age: u32,
}

// 单元结构（无字段），一般用于泛型
struct Unit;

// ============== enum =================
enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
}

// if-let
enum Symbol {
    Char(char),
    Number,
}

// while-let
#[derive(Debug)]
enum Alphabet {
    A,
    B,
    C,
}

// function
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    // 构造方法
    fn new(x: u64, y: u64) -> Point {
        Point { x , y }
    }

    fn get_x(&self) -> u64 {
        self.x 
    }

    fn set_x(&mut self, x: u64) {
        self.x = x;
    }
}

// 高阶函数
type Method = fn(u32, u32) -> u32;
//fn calc(method: fn(u32, u32)->u32, a: u32, b: u32) -> u32 {
fn calc(method: Method, a: u32, b: u32) -> u32 {
    method(a, b)
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

// 发散函数
fn foo() ->! {
    panic!("This call never returns.");
}

fn main() {
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    let (sum, is_overflow) = a.overflowing_add(b);

    println!("sum = {:?}, is_overflow = {}", sum, is_overflow);

    // 元组
    let ta: i32 = 10;
    let tb: char = 'A';

    let mytuple: (i32, char) = (ta, tb);

    let (tc, td) = mytuple;
    println!("tc={}, td={}", tc, td);

    println!("mytuple.0 = {}", mytuple.0);
    println!("mytuple.1 = {}", mytuple.1);

    println!("====================array==================");

    // 数组
    let myarray: [u32; 5] = [1,2,3,4,5];
    println!("myarray=[1] = {}", myarray[1]);
    
    // 数组越界
    // let index = "5".parse::<usize>().unwrap();
    // println!("myarray[5] = {}", myarray[index]);

    let mut mybuffer: [u32; 32 * 1024] = [0; 32 * 1024];
    println!("mybuffer[1024]={}", mybuffer[1024]);

    mybuffer[1024] = 13;

    println!("mybuffer[1024]={}", mybuffer[1024]);

    // 切片
    println!("============================slice================================");

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // 创建切片

    println!("slice[0]={}, len={}", slice[0], slice.len());

    let slice2 = &arr[3..5];
    println!("slice2[0]={}, slice2[1]={}", slice2[0], slice2[1]);
    println!("slice2_len={}", slice2.len());

    let mut slice3 = &mut arr[..];
    slice3[0] = 6;
    println!("arr[0]={}", arr[0]);

    // 结构体
    println!("=============================struct==============================");
    // 元组结构
    let pair = Pair(10, 4.2);
    println!("{}", pair.0);

    // 标准C结构
    let jack = Person {
        name: String::from("jack"),
        age: 35,
    };
    println!("name={}, age={}", jack.name, jack.age);

    // 单元结构
    let unit = Unit;

    // 打印结构体所有成员
    println!("all struct = {:?}", jack);

    // 枚举
    println!("==============================enum==============================");
    let localhost: IPAddr = IPAddr::IPv4(127, 0, 0, 1);
    
    match localhost {
        IPAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        } 
        _ => {}
    }

    // 类型转换
    println!("==============================类型转换===========================");
    let ca: i8 = -10;
    let cb = a as u8;
    println!("a={} b={}", ca, cb);

    // transmute 
    // 只能在unsafe块中
    unsafe {
        let ta = [0u8, 1u8, 0u8, 0u8];
        let tb: u32 = mem::transmute(ta);
        println!("transmute result = {}", tb)
    }

    // if表达式
    println!("=============================if表达式============================");
    // 作为 语句
    let ifn = 0;
    if ifn > 0 {
        println!("{} is positive", ifn);
    } else if ifn < 0 {
        println!("{} is nagative", ifn);
    } else {
        println!("{} is zero", ifn);
    }

    // 作为表达式
    let ifm = if ifn < 0 {
        2.0
    } else {
        3.0
    };

    println!("ifm = {}", ifm);

    // loop
    println!("=============================loop================================");
    let mut loopsum = 0;
    let mut loopn = 1;
    loop {
        loopsum += loopn;
        loopn += 1;
        if loopn > 100 {
            break;
        }
    }
    println!("1+2+... + 100 = {}", loopsum);

    // 作为表达式
    let mut loopcounter = 0;

    let loopresult = loop {
        loopcounter += 1;
        if loopcounter == 10 {
            break loopcounter * 2;
        }
    };

    println!("loopresult={}", loopresult);

    // while
    println!("==================================while=============================");
    let mut whilen = 1;
    while whilen < 101 {
        if whilen % 15 == 0 {
            println!("fizzbuzz");
        } else if whilen % 3 == 0 {
            println!("fizz");
        } else if whilen % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", whilen);
        }
        whilen += 1;
    }

    // for-range
    println!("==================================for-range=========================");
    // [)
    for i in 0..5 {
        println!("i = {}", i);
    }

    println!("---------------");
    // []
    for i in 0..=5 {
        println!("i = {}", i);
    }

    println!("---------------");

    // 遍历数组
    let for_range_arr = ["a", "b", "c"];
    for i in for_range_arr.iter() {
        println!("i = {}", i);
    }

    println!("----------------");

    // 可变
    let mut for_range_arr2 = [1, 2, 3];
    for i in for_range_arr2.iter_mut() {
        *i *= 2;
    }

    for i in for_range_arr2.iter() {
        println!("i = {}", i);
    }

    // match
    println!("==============================match==================================");
    let match_opcode: u8 = 42;
    match match_opcode {
        42 => {
            println!("bingo!");
        } 
        // 其它情况
        _ => {
            println!("{}", match_opcode)
        }
    }

    // if-let语法糖
    println!("===============================if let=================================");
    let letter = Symbol::Char('A');
    if let Symbol::Char(x) = letter {
        println!("x = {}", x);
    } 
    
    // while-let语法糖
    println!("===============================while let==============================");
    let mut while_letter = Alphabet::A;
    while let Alphabet::A = while_letter {
        println!("{:?}", while_letter);
        while_letter = Alphabet::B;
    }

    // function
    println!("===============================function================================");
    let mut p = Point::new(10, 20);
    println!("p = {:?}", p);
    println!("p.x = {:?}", p.get_x());

    p.set_x(30);
    println!("p.x = {:?}", p.get_x());

    // 闭包
    let times3 = |n: u32| -> u32 {n * 3};
    println!("time3={}", times3(10));

    // move: 将环境中的值移动到闭包内部
    // 使用场景：从主线程移动值到子线程
    let hello_message = "Hello World!";
    thread::spawn(move || {
        println!("thread hello = {}", hello_message);
    }).join();

    println!("calc = {}", calc(add, 10, 20));
    println!("calc = {}", calc(sub, 30, 10));

    // 发散函数
    let a = if true {
        10 
    } else {
        foo()
    };
    println!("{}", a);
}  