# 南方之诗——后端
这是后端部分。主要负责和数据库通信，还有处理来自前端的 HTTP 请求。  
大体上，数据库的表可以在 migrations 文件夹里面找到他们的定义；每一行的内容就和名字一样，没什么特别的。
数据库连接使用 `diesel` 提供的解决方案。  
处理 HTTP 会试着使用 `actix-web` 提供的一套 actor 框架。  

## 启动！
将数据库（因为是依靠 SQL 来初始化的，所以暂时只能使用 `postgresql` 来作为 DataSource。）的 URL
放到 .env 文件中的 `DATABASE_URL` 环境变量中去；然后安装 diesel-cli：  
```bash
cargo install diesel-cli
```
接着就可以创建模式了：
```bash
diesel migration run
```
试试看能否真的回滚：
```bash
diesel migration redo
```
使用 `diesel setup` 来生成 schema 模块。 

# 最后要什么？
**读者**在首页阅读**文章**的简介。
首页一次性发送数篇新**文章**，**读者**可以**请求**更老的文章，也可以**搜索**满足条件的文章。
进入具体的**文章**页面后，**读者**可以**评论文章**。


# 一些无关紧要的笔记
### 依赖倒置？贫血对象？
##### 问题
中间……有一个非常严重的问题。  
我希望使用 `repository` 来阻止 Actor 系统和数据库实现产生耦合：不过随后发现很难办，
因为我为 `Post` 加上了几个方法；但是这几个方法全部和 diesel 有着强耦合：
就是说，无论如何，`Post` 对象都会需要借助于 diesel 的力量来完成操作。  
要去掉这几个方法来让 `repository` 可用吗？  
*绝对不行，因为这样一来，`Post` 就彻底成为了贫血对象了；之后的复杂度只会越来越大。*  
将 `Post` 作为一个 trait 怎么样？对于序列化和传送，可以让其实现 `Into<WebPost>`；其他的几个方法也可以因此 mock 化。  
*这是目前看来最棒的方法；恰如那句话“接受函数而不是数据结构。”。
但是很多代码可能需要重写。
另一个问题是，到现在我对 rust 的 trait 使用还是没什么信心……*  
或者说，不要 `repository` 了，让 Actor 们直接从 diesel 去获取 `Post`，因为 `Queryable` 在某种程度上本身就带有 `repository` 的特性。  
*可以，但是问题是：我们并没有去修改 `Queryable` 的能力。`Queryable` 本身也不会提供实现它的类型的信息。
换句话说，无法类型安全地将其注入到工作的 Actor 中去。*  
##### 怎么做？
让我好好想想吧……