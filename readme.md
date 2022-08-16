
# 初识rust
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

# rust 的基本概念
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
