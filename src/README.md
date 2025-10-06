# Rust 学习资料集合

这个目录包含了一系列Rust编程语言的教学文件，按照数字序号组织，涵盖了Rust的各种语法和功能。每个文件都包含了详细的注释和示例代码，帮助你学习和理解Rust的核心概念。

## 文件列表

### 基础概念
1. **01_variables_and_data_types.rs** - 变量和数据类型
   - 不可变和可变变量
   - 基本数据类型（整数、浮点数、布尔值、字符）
   - 复合数据类型（元组、数组）
   - 类型注解和转换
   - 常量

2. **02_functions.rs** - 函数
   - 函数定义和调用
   - 参数和返回值
   - 多个返回值（元组）
   - 递归函数
   - 闭包函数

3. **03_control_flow.rs** - 流程控制
   - if表达式
   - loop循环
   - while循环
   - for循环
   - match表达式（模式匹配）
   - if let和while let语法糖

### 所有权和类型系统
4. **04_ownership.rs** - 所有权
   - 所有权规则
   - 移动（Move）语义
   - 复制（Copy）语义
   - 克隆（Clone）操作
   - 借用（引用）
   - 可变引用和不可变引用
   - 引用规则

5. **05_structs.rs** - 结构体
   - 结构体定义和实例化
   - 可变结构体
   - 结构体更新语法
   - 元组结构体
   - 单元结构体
   - 结构体方法
   - 关联函数

6. **06_enums_and_pattern_matching.rs** - 枚举和模式匹配
   - 枚举定义和实例化
   - 带关联数据的枚举
   - 枚举方法
   - Option枚举
   - 高级模式匹配技巧

### 集合和模块系统
7. **07_collections.rs** - 常见集合及操作
   - Vector（动态数组）
   - String（UTF-8字符串）
   - HashMap（键值对集合）
   - 集合的基本操作（创建、添加、访问、遍历等）

8. **08_packages_and_modules.rs** - 包和模块
   - 模块定义
   - 公共和私有访问控制
   - 路径访问
   - use语句
   - 导入和重导出

### 错误处理和泛型
9. **09_error_handling.rs** - 错误处理
   - panic!宏（不可恢复错误）
   - Result枚举（可恢复错误）
   - unwrap和expect方法
   - 错误传播
   - ?运算符
   - 自定义错误类型

10. **10_generics.rs** - 泛型
    - 泛型函数
    - 泛型结构体
    - 泛型枚举
    - 泛型约束
    - where子句

11. **11_traits.rs** - Trait
    - Trait定义
    - 默认实现
    - 为类型实现Trait
    - Trait作为参数和返回类型
    - Trait约束
    - 关联类型
    - Trait继承

12. **12_lifetimes.rs** - 生命周期
    - 生命周期注解
    - 结构体中的生命周期
    - 方法中的生命周期
    - 多个生命周期参数
    - 生命周期省略规则
    - 静态生命周期

13. **13_advanced_features.rs** - 高级特性
    - 闭包
    - 迭代器
    - 线程
    - 智能指针（Box、Rc、RefCell）
    - 宏
    - 内部可变性
    - 不安全Rust

## 如何使用

1. 使用`cargo run --bin <文件名>`命令运行单个示例：
   ```
   cargo run --bin 01_variables_and_data_types
   ```

2. 或者直接使用`rustc`编译并运行：
   ```
   rustc 01_variables_and_data_types.rs
   ./01_variables_and_data_types
   ```

3. 阅读代码和注释，理解每个概念的用法和原理。

4. 修改和扩展示例代码，加深理解。

## 学习建议

- 按照序号顺序学习，从基础到高级逐步掌握
- 每个概念都编写自己的练习代码
- 阅读代码时注意观察Rust的所有权系统和类型系统
- 尝试将示例代码组合起来，构建简单的应用程序

祝你学习Rust愉快！