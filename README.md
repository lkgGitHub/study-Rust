# study-Rust
study Rust note


第一天运行，提示项目名用"蛇形命名法"，所以暂时这个项目名和项目里其他项目名不一致。 尽量按照本语言规范来。


```shell
rustup update
rustup.exe update
rustc --version

# 卸载
rustup self uninstall
```

# 问题一： error: linker `link.exe` not found

依赖于微软的 msvc linker.exe。
一般让去下载 VS，不过可以直接用 rust 命令解决。
```shell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```