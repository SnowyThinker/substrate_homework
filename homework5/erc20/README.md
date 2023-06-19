# contract

* 创建合约

~~~shell
cargo contract new erc20
~~~

* 编译合约

~~~shell
cargo contract build
~~~

* 启动节点

~~~shell
target/release/substrate-contracts-node --dev --tmp
~~~

* 编写单元测试

* 安装 cargo-expand

~~~shell
cargo install cargo-expand
~~~

* 生成 out.rs文件

~~~shell
cargo expand --lib > out.rs
~~~
