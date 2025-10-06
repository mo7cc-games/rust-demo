# rust-demo

rust 的学习 Demo

## 常用命令

````bash

## 安装依赖
cargo install cargo-watch
cargo install cargo-edit

## 运行
cargo run

## 编译调试
cargo build

## 编译发布
cargo build --release

# 当代码文件变化时，自动重新运行程序。
# rust-demo

本仓库为 Rust 入门示例集合，每个示例侧重讲解 Rust 的一个核心概念，方便阅读、实验与练习。

结构说明
- 示例文件均位于 `src/bin/`，以数字前缀分组（例如 `01_variables_and_types.rs`）。每个文件是一个独立的 binary，可以单独运行。

快速前提
- 已安装 Rust 工具链（rustup、cargo）。
- Windows PowerShell 下示例命令已给出。其他 shell（bash）也适用，但注意命令格式。

如何运行（PowerShell）

运行单个示例（推荐，用于学习每个概念）：

```powershell
cargo run --bin 01_variables_and_types
````

构建所有示例（仅构建，不运行）：

```powershell
cargo build
```

构建 release 版本：

```powershell
cargo build --release
```

如果想持续监视文件变更并自动运行（需安装 `cargo-watch`）：

```powershell
cargo install cargo-watch
cargo watch -x run
```

示例文件清单与说明

- `src/bin/01_variables_and_types.rs` — 变量（可变/不可变）、基本标量类型（整型、浮点、布尔、字符）、元组与数组示例。重点：变量默认不可变，类型推断与显式注解。
- `src/bin/02_functions.rs` — 函数定义、参数与返回值、表达式 vs 语句、闭包示例。重点：表达式会返回值。
- `src/bin/03_control_flow.rs` — 流程控制：`if/else`、`loop`、`while`、`for`。重点：`for` 常配合区间和迭代器使用。
- `src/bin/04_ownership.rs` — 所有权/移动/拷贝、借用（不可变/可变借用）、引用规则。重点：借用规则防止悬垂引用与数据竞争。
- `src/bin/05_structs.rs` — 结构体定义、元组结构体（示例中为命名结构体）、`impl` 块中方法与关联函数（如构造函数）。
- `src/bin/06_enums_pattern_matching.rs` — 枚举与 `match`、`if let`。重点：枚举搭配模式匹配写法非常强大，用于表示有多种可能的数据形态。
- `src/bin/07_collections.rs` — 常见集合：`Vec`、`String`、`HashMap`，包括插入、遍历示例。重点：集合的所有权和借用语义。
- `src/bin/08_packages_and_modules.rs` — 模块（`mod`）、可见性（`pub`）、`use` 导入示例。说明如何组织 crate 内代码。
- `src/bin/09_error_handling.rs` — 错误处理：`Result<T, E>`、`?` 运算符、`panic!`（示例中注释掉）。示例通过读取 `Cargo.toml` 展示文件 IO 与错误传播。
- `src/bin/10_generics.rs` — 泛型函数与约束（trait bounds），示例展示对不同类型复用算法。
- `src/bin/11_traits.rs` — Trait 定义与实现、默认方法与 trait 作为函数参数的用法。相当于接口抽象。
- `src/bin/12_lifetimes.rs` — 生命周期标注示例，说明引用生命周期如何标注以避免返回悬垂引用。

教学建议（如何高效学习）

- 先逐个运行每个文件并阅读注释；修改并重新运行以观察变化。
- 面对所有权与借用章节，尝试构造借用冲突的例子，看编译器错误提示，理解错误信息。
- 把小示例拆开写单元测试（在 `src/lib.rs` 或 `tests/` 中），能帮助理解行为和边界情况。

常见问题（FAQ）

- 如果 `cargo run --bin <name>` 报错提示找不到 binary：确保文件名位于 `src/bin/`，并且 `--bin` 的名字等于文件名（不带 `.rs`）。
- 如果 `09_error_handling` 读取文件失败：检查仓库根目录是否存在 `Cargo.toml`，或修改示例中要读取的路径。

参考资料

- The Rust Programming Language（The Rust Book）：https://doc.rust-lang.org/book/
- Rust by Example：https://doc.rust-lang.org/rust-by-example/

其它脚本（项目内辅助命令）

- 项目中包含一些脚本：`script/` 目录下有 `clear_dir.ts`、`git_sync.ts` 等，可通过 `npm run` 脚本调用（需要 Node 环境）。

示例：在 PowerShell 中一次运行某个示例并查看输出

```powershell
# 运行第 3 个示例（流程控制）
cargo run --bin 03_control_flow
```

如果你希望我为每个示例补充更详尽的注释、练习题、或把样例转换成教学 markdown 文件，告诉我优先级与风格（例如短教程或带习题的章节点）。

---

最后更新：添加了一系列按序号组织的 Rust 学习示例，构建已在本地验证通过。如需继续，我可以：

- 为每个示例添加更详细的中文注释和练习题；
- 为部分示例添加单元测试并配置 CI（例如 GitHub Actions）；
- 将示例转成教学文档（`docs/` 或 GitHub Pages）。
