// fn greet_world() {
//     let southern_germany = "Grüß Got&&t!";
//     let chinese = "世界，你好";
//     let e&nglish = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region)
//     }
// }
// fn main() {
//     greet_world()
// }

// https://course.rs/first-try/hello-world.html#rust-%E8%AF%AD%E8%A8%80%E5%88%9D%E5%8D%B0%E8%B1%A1
// fn main() {
//   let penguin_data = "\
//   common name,length (cm)
//   Little penguin,33
//   Yellow-eyed penguin,65
//   Fiordland penguin,60
//   Invalid,data
//   ";

//   let records = penguin_data.lines();

//   for (i, record) in records.enumerate() {
//     if i == 0 || record.trim().len() == 0 {
//       continue;
//     }

//     // 声明一个 fields 变量，类型是 Vec
//     // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//     // <_> 表示 Vec 中的元素类型由编译器自行推断
//     let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//     // 只有在debug模式下生效
//     if cfg!(debug_assertions) {
//       // 输出到标准错误输出
//       eprintln!("debug: {:?} -> {:?}", record, fields)
//     }

//     let name = fields[0];

//     if let Ok(length) = fields[1].parse::<f32>() {
//       // 输出到标准输出
//       println!("{}, {}cm", name, length)
//     }
//   }
// }

// rust 的基本概念
// fn main() {
//   let a = 10;
//   // 主动指定b的类型为i32
//   let b: i32 = 20;

//   // 可以在数值中表示数值是30，类型是i32
//   // 同时可以指定其为 mutable
//   let mut c = 30i32;

//   // 在数值和类型中间添加一个下划线，让可读性更好
//   let d = 30_i32;
//   // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
//   c = add(add(a, b), add(c, d));

//   // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
//   // 该函数将指定的格式化字符串输出到标准输出中(控制台)
//   // {}是占位符，在具体执行过程中，会把e的值代入进来
//   println!("( a + b ) + ( c + d ) = {}", c);
// }

// // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
// fn add(i: i32, j: i32) -> i32 {
//   // 返回相加值，这里可以省略return
//   // i + j 后面不可以添加 ;
//   i + j
// }

// 变量解构
// fn main() {
//     let (a, mut b) = (true, false);
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b)
// }

// 解构式赋值
// struct Struct {
//     e: i32,
// }
// fn main() {
//     let (a, b, c, d, e);

//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };

//     assert_eq!([1, 2, 3, 4, 5], [a, b, c, d, e]);
// }


// 常量
// fn main() {
//     // 插入下划线以提高可读性
//     const MAX_POINTS: u32 = 100_100;
//     println!("{}", MAX_POINTS)
// }

// 变量遮蔽
fn main() {
    // 所有权
    let x = 5;
    // 在 main 函数的作用与内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("花括号内的x值为: {}", x);
    }

    println!("main函数内的x值为: {}", x);
}
// 这个程序首先将数值 5 绑定到 x，然后通过重复使用 let x = 来遮蔽之前的 x，并取原来的值加上 1，所以 x 的值变成了 6。第三个 let 语句同样遮蔽前面的 x，取之前的值并乘上 2，得到的 x 最终值为 12。
