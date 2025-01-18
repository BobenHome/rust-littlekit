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

## 闭包
### 语法
```rust
// 基本语法
|参数| 返回值

// 例子
|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())

.map_err(|e| ...)  // e 是原始错误
// 相当于这样的函数
fn convert_error(e: SqlxError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
```
### 其他闭包例子
```rust
// 无参数闭包
|| println!("hello")

// 多参数闭包
|x, y| x + y

// 带类型标注的闭包
|x: i32| x * 2
```
### 闭包是一种匿名函数，|e| 中的 e 是参数名，类似函数的参数列表。这里用于将数据库错误转换为 HTTP 错误响应。

## SQLx 宏
- `query_as!` 是编译时宏
- `!` 表示这是声明宏（declaration macro）
- 提供 SQL 语句的编译时检查
- 确保查询结果与 Rust 类型匹配

## 引用操作符 &
- `&db` 创建数据库连接池的引用
- 允许多个查询共享同一个连接池
- 允许共享访问而不转移所有权

## Axum 参数提取
- 通过参数提取器自动注入值
- `State(db)`: 提取应用状态（如数据库连接）
- `Query(query)`: 提取 URL 查询参数
- 不需要手动传参，Axum 框架自动处理

## 运行说明
### 开发环境
```bash
# 运行服务器
cargo run
```

### 生产环境
```bash
# 编译优化版本
cargo build --release

# 运行服务器
./target/release/rust-http
```

### 测试 API
```bash
# 测试用户查询
curl http://localhost:3000/users
curl http://localhost:3000/users?name=张
```

