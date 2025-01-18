## 异步函数

```rust
#[tokio::main]
async fn main() {
}
```

### #[tokio::main] 是一个重要的属性宏（attribute macro）

- 它告诉 Rust 编译器，这个函数是异步的，并且需要使用 Tokio 运行时。
- 它还确保在运行时正确初始化 Tokio 的运行时环境。
- 实际上会被展开成类似这样：

```rust
fn main() {
  tokio::runtime::Runtime::new()
      .unwrap()
      .block_on(async {
          // 你的异步代码
      })
}
```
## ::语法
在 Rust 中，::是路径分隔符（path separator），用于：
- 命名空间访问
  用于访问模块、类型、函数等类似其他语言的 .（点）操作符
- 关联函数调用
  Router::new() 中的 new() 是一个关联函数（associated function）类似其他语言的静态方法（static method）

## 异步函数

```rust
async fn hello_world() -> &'static str {
    "Hello, World!"
}
```
- async fn
  1. async 关键字表明这是一个异步函数
  2. 可以在函数内使用 .await 等待其他异步操作
  3. 返回一个 Future（未来值）
- 返回类型 &'static str
  1. str 是字符串切片类型
  2. 'static 是生命周期标注，表示这个字符串在整个程序运行期间都有效
  3. & 表示这是一个引用
- 字符串字面量返回
  1. 在 Rust 中，字符串字面量默认就是 &'static str 类型
  2. 存储在程序的只读内存中
