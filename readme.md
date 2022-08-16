
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