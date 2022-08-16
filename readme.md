
# 一、初识rust
## 1. 创建项目
```zsh
cargo new world_hello
cd world_hello
```

## 2. 运行项目
运行方式一：  
```zsh
cargo run
```
运行方式二：
手动编译并运行项目  
编译  
```zsh
cargo build
```
运行  
```zsh
./target/debug/world_hello
```
这是是debug模式，编译速度非常快，但运行速度就慢了，想要高性能的代码需要添加`--release`进行编译  
```zsh
cargo run --release
cargo build --release
./target/release/world_hello
```

## cargo check
项目庞大以后上面两种都会变慢，可以使用`cargo check`来检查项目代码能否通过编译。
```zsh
cargo check
```

# 二、Rust 基础入门
## rust 的基本概念
```rust
fn main() {
	let a = 10;
	// 主动指定b的类型为i32
	let b: i32 = 20;

	// 可以在数值中表示数值是30，类型是i32
	// 同时可以指定其为 mutable
	let mut c = 30i32;

	// 在数值和类型中间添加一个下划线，让可读性更好
	let d = 30_i32;
	// 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
	c = add(add(a, b), add(c, d));

	// println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
	// 该函数将指定的格式化字符串输出到标准输出中(控制台)
	// {}是占位符，在具体执行过程中，会把e的值代入进来
	println!("( a + b ) + ( c + d ) = {}", c);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
	// 返回相加值，这里可以省略return
	i + j
}
```
1. 在上面的`add`函数中，不可以为表达式`i + j`添加`;`，因为表达式不能包含分号，一旦加上分号就变成了语句，导致不会返回值。  
2. 字符串需要使用双引号而不是单引号，Rust 中单引号是留给单个字符类型(`char`)使用的  
3. Rust 使用`{}`来作为格式化输出占位符，`println!`会自动推导出具体的类型，无需手动指定  

## 变量绑定与解构
### 变量绑定
在 Rust 中声明一个变量：`let a = "hello world"`，这个过程叫*变量绑定*。

### 变量可变性
Rust 的变量在默认情况下是*不可变的*。可以通过`mut`关键字让变量变为*可变的*。

### 使用下划线开发忽略未使用的变量
如果希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头：
```rust
fn main() {
	let _x = 5;
	let y = 10;
	println!("{}", y)
}
```

### 变量解构
```rust
fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

```

#### 解构式赋值
```rust
struct Struct {
    e: i32,
}
fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 3, 4, 5], [a, b, c, d, e]);
}
```

### 常量
常量使用`const`关键字声明，并且值的类型必须标注。  
常量不允许使用`mut`。常量自始至终不可变，在编译完成后，已经确定了它的值。  
```rust
// 插入下划线以提高可读性
const MAX_POINTS: u32 = 100_100;
```

### 变量遮蔽
```rust
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
```
这个程序首先将数值 5 绑定到 x，然后通过重复使用 let x = 来遮蔽之前的 x，并取原来的值加上 1，所以 x 的值变成了 6。第三个 let 语句同样遮蔽前面的 x，取之前的值并乘上 2，得到的 x 最终值为 12。

这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。