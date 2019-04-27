# 南方之诗——后端
这是后端部分。主要负责和数据库通信，还有处理来自前端的 HTTP 请求。  
大体上，数据库的表可以在 migrations 文件夹里面找到他们的定义；每一行的内容就和名字一样，没什么特别的。
数据库连接使用 `diesel` 提供的解决方案。  
处理 HTTP 会试着使用 `actix-web` 提供的一套 actor 框架。  

## 特别注意！
这些代码没有任何期望行为，其中的测试样例、模式文档都是虚构的；那些东西与这个仓库里面的代码没有任何关联。  
但是这些代码的功能都确实在我心中。  

## 启动！
你可以使用交互式的配置脚本 EnvGen 来产生 .env 文件，也可以手动设置他们：
``` bash
# 使用 ghci 解释运行……
ghci EnvGen.hs
*Main> main
# ...

# 或者，用 ghc 编译之后运行
ghc -O2 EnvGen.hs
./EnvGen
# ...
```

|  名字  |  作用  |  
| ------- | ------ |
| DATABASE_URL | 数据库 URL；您可以参考 diesel 的文档获得格式。 |  
| DISABLE_RECAPTCHA | 禁用 RECAPTCHA，这个环境变量在今后大概率会被取消。 |  
| ENABLE_CORS | 启用跨站资源引用，这能让前端和后端在不同 URL 的时候（开发环境或者多机计算的时候居多）浏览器前端脚本的 ajax 正常运行。 |  
| RECAPTCHA_SECRET | recaptcha 的服务器密钥。 |  

然后安装 diesel-cli：  
```bash
cargo install diesel_cli
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
然后就可以编译启动了：
```
cargo build --bin vos --release
./target/release/vos run
```
在制作完成管理节点（可能会考虑 `vert.x` 一类的技术栈。）之前，可以先使用这个工具来上传文档：
```
vos post add [info...]
vos tag add [info...]
```

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
- 要去掉这几个方法来让 `repository` 可用吗？  
- *绝对不行，因为这样一来，`Post` 就彻底成为了贫血对象了；之后的复杂度只会越来越大。*  
- 将 `Post` 作为一个 trait 怎么样？对于序列化和传送，可以让其实现 `Into<WebPost>`；其他的几个方法也可以因此 mock 化。  
*这是目前看来最棒的方法；恰如那句话“接受函数而不是数据结构。”。
但是很多代码可能需要重写。
- 另一个问题是，到现在我对 rust 的 trait 使用还是没什么信心……*  
- 或者说，不要 `repository` 了，让 Actor 们直接从 diesel 去获取 `Post`，因为 `Queryable` 在某种程度上本身就带有 `repository` 的特性。  
- *可以，但是问题是：我们并没有去修改 `Queryable` 的能力。`Queryable` 本身也不会提供实现它的类型的信息。
换句话说，无法类型安全地将其注入到工作的 Actor 中去。*  
##### 怎么做？
我决定放弃 `repository` 了；这样的抽象的开销实在太大了——对于 `rust` 来说，我并没有（也不太想）去寻找一套那样的工具，如果自己手动处理这些，很快我可能会陷入重复代码的泥潭中。但是我仍旧保留了数据库的模型和服务器前端模型之间的不同，并且由连接两者 Actor 们作为这两者的中间层，现在暂时还不错：因为某种程度上，Actor 们承担上了内容库们原本的人物：我现在可以 Mock Actor，就像 Mock 内容库那样。
