use std::mem;
use std::thread;
mod omod1;
mod omod2;


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

// pub mod
mod mod1 {
    pub const MESSAGE: &str = "Hello World";
    
    // 下面两种是一样的效果，表示在当前这个mod下可见
    //pub(self) const NUMBER: u32 = 42;
    const NUMBER: u32 = 42;

    // 在crete下可见
    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";

        pub fn say42() {
            println!("{}", super::NUMBER);
        }
    }
}

// 结构体的可见性
mod p1 {
    pub struct Person {
        pub name: String,
        nickname: String,
    }

    impl Person {
        pub fn new(name: &str)-> Self {
            Person {
                name: String::from(name),
                nickname: String::new(),
            }
        }

        pub fn set_nickname(&mut self, nickname: &str) {
            self.nickname = String::from(nickname);
        }

        pub fn say_nickname(&self) {
            println!("nickname: {}", self.nickname);
        }
    }
}

// super self
fn function() {
    println!("super function.")
}

mod mod3 {
    pub fn function() {
        super::function();
    }

    pub mod mod4 {
        fn function() {
            println!("mod3:mod4:function");
        }

        pub fn call() {
            super::function();
            self::function();
        }
    }
}

// 泛型
// std::cmp::PartialOrd修饰泛型的特征，表示可以比较
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// 泛型结构
#[derive(Debug, PartialEq, Default)]
struct Point1<T> {
    x: T,
    y: T,
}
struct Line<T> {
    x: Point1<T>,
    y: Point1<T>,
}

// 实现结构
impl<T: Clone + std::cmp::PartialOrd> Point1<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
}

// 可以只实现某种类型
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2))
    }
}

// traits
impl<T: std::fmt::Display> std::fmt::Display for Point1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 另一种方法
fn show(a: impl std::fmt::Display) {
    println!("show: {}", a);
}

#[derive(Debug, Default)]
struct Point2 {
    x: i32,
    y: i32,
    z: f64,
}

fn bigger<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1 > str2 {
        str1
    } else {
        str2
    }
}

#[derive(Debug)]
struct Person2<'a> {
    name: &'a str,
}

// 错误处理
fn eadd(a: u32, b: u32) -> u32 {
    // 未实现的代码
    unimplemented!()
}
fn divide_by_three(x: u32) -> u32 {
    for i in 0.. {
        if 3 * i < i {
            panic!("u32 overflow");
        }
        if x < 3 * i {
            return i - 1;
        }
    }
    
    // 一定执行不到的代码
    unreachable!()
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

    // mod pub
    println!("==============================pub mod==========================");
    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::CrateEnum::Item as u32);
    mod1::mod2::say42();


    // 结构体可见性
    let mut sp1 = p1::Person::new("jack");
    sp1.set_nickname("baby");
    sp1.say_nickname();

    // self super
    mod3::mod4::call();

    // 分文件 
    println!("==============================分文件=============================");
    println!("mod1-message:{}", omod1::MESSAGE);
    println!("mod2-message:{}", omod2::MESSAGE);
    println!("mod2_a-message:{}", omod2::omod2_a::MESSAGE);
    println!("mod2_b-message:{}", omod2::omod2_b::MESSAGE);

    // 泛型
    println!("==============================泛型================================");
    println!("{}", largest(10, 20));
    println!("{}", largest(10.0, 20.0));
    println!("{}", largest('a', 'b'));

    let fa:f64 = 3.14;
    let fb:f64 = 2.17;
    println!("{}", largest(fa, fb));

    // 结构泛型
    let point1: Point1<u32> = Point1 { x: 0, y: 0 };
    let point2: Point1<u32> = Point1 { x: 2, y: 2 };
    let line = Line {x: point1, y: point2};

    println!("{} {} {} {}", line.x.x, line.x.y, line.y.x, line.y.y);
    
    // 实现结构
    let point3 = Point1{x: 10, y:20};
    println!("{:?}", point3.largest());

    let point3 = Point1 {x: 10.0, y: 20.0};
    println!("{}", point3.largest());
    println!("{}", point3.distance_from_origin());

    // traits
    let point5 = Point1{x: 10, y: 20};
    println!("{}", point5);

    show(point5);
    
    // 自动派生
    // #[derive(Debug)]
    let point6 = Point1{x: 50, y: 100};
    println!("{:?}", point6);

    // #[derive(PartialEq)]
    let point7 = Point1{x: 50, y: 100};
    println!("p6 == p7 : {}", point6 == point7);

    // #[derive(Default)]
    // 填充默认值
    let point8 = Point2::default();
    println!("default-point:{:?}", point8);

    // 所有权
    println!("============================所有权==========================");
    let s2: String;
    {
        let s1 = String::from("Hello World!");
        s2 = s1;
        // println!(s1); // 会报错，所有权已经转移到s2了
    }
    println!("{}", s2);

    // 生命周期
    println!("==============================生命周期=============================");
    let p5 = Person2{name: "jack"};
    println!("{:?}", p5);

    println!("{}", bigger("a", "b"));

    // 错误处理
    println!("==============================错误处理==============================");
    // 不可恢复错误
    // panic!("error");
    // assert!(1 == 2);
    // assert_eq!(1, 2);

    // println!("{}", eadd(10, 20));
    // divide_by_three(100);

    // 可恢复错误
    let r = std::fs::read("/tmp/foo");
    match r {
        Ok(data) => println!("{:?}", std::str::from_utf8(&data).unwrap()),
        Err(err) => println!("{:?}", err),
    }
}  