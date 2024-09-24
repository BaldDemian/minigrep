## 功能

在一个给定的文件中搜索指定的字符串并进行染色。支持大小写不敏感的匹配

## 结构

-   `main.rs` 负责解析命令行参数、初始化其他配置，调用 `lib.rs` 中的 `run()`，并对 `run()` 可能返回的错误进行处理
-   `lib.rs` 负责维护具体的业务代码

## 参考

[入门实战：文件搜索工具 - Rust语言圣经(Rust Course)](https://course.rs/basic-practice/intro.html)