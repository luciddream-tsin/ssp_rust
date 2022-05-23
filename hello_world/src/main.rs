
fn main_001() {
    println!("{}, this is ", 3);
    println!("{1} - {3} - {0} - {2}", "a", "b", "jj", "i");
    println!("{a} is {b}", b="a", a="b");
    println!("{} of {:b}", 1, 2);// 1 / 2(binary)
    println!("{number:>width$}", number=2, width=9);
    println!("{number:>0width$}", number=2, width=9);

    //std::fmt 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：
    //fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
    //fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。
    //上例使用了 fmt::Display，因为标准库提供了那些类型的实现。若要打印自定义类型的文本，需要更多的步骤。
}




//调试（Debug）
//所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。
// 自动的实现只为一些类型提供，比如 std 库中的类型。所有其他类型都必须手动实现。
//fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推导（derive，即自动创建）fmt::Debug 的实现。
// 但是 fmt::Display 需要手动实现。


// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);
// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
//所有 std 库类型都天生可以使用 {:?} 来打印：


// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Struct(i32);
// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Struct);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main_002(){
    println!("{:?}", 12);
    println!("{:?}", Deep(Struct(3)));
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `3` 怎么办？


    //所以 fmt::Debug 确实使这些内容可以打印，但是牺牲了一些美感。Rust 也通过 {:#?} 提供了 “美化打印” 的功能：
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    //美化大印
    println!("{:?}", peter);
    println!("{:#?}", peter);
    //你可以通过手动实现 fmt::Display 来控制显示效果。
}









//显示（Display）
//fmt::Debug 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。这需要通过 手动实现 fmt::Display 来做到。
//fmt::Display 采用 {} 标记。实现方式看 起来像这样：

//// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;
use std::fmt::Formatter;

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    //    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
//fmt::Display 的效果可能比 fmt::Debug 简洁，但对于 std 库来说，这就有一个问 题。模棱两可的类型该如何显示呢？举个例子，
// 假设标准库对所有的 Vec<T> 都实现了同 一种输出样式，那么它应该是哪种样式？下面两种中的一种吗？
//Vec<path>：/:/etc:/home/username:/bin（使用 : 分割）
//Vec<number>：1,2,3（使用 , 分割）
//这并不是一个问题，因为对于任何非泛型的容器类型， fmt::Display 都能够实 现。…

// 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main_003() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}


//测试实例：List
//对一个结构体实现 fmt::Display，其中的元素需要一个接一个地处理到，这可能会很麻 烦。问题在于每个 write! 都要生成一个
//fmt::Result。正确的实现需要 处理所有的 Result。Rust 专门为解决这个问题提供了 ? 操作符。

// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
        write!(f, "[")?;//<------------------------------------------------------------

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。

            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main_004() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}








use std::fmt::{Display};
struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
        // 一个缓冲区（即第一个参数f）中。
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}
