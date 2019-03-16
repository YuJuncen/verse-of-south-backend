# 南方之诗——后端
这是后端部分。主要负责和数据库通信，还有处理来自前端的 HTTP 请求。  
大体上，数据库有三张表；可以在 migrations 文件夹里面找到他们的定义；每一行的内容就和名字一样，没什么特别的。
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

# 结构
在与数据库通信的部分，使用 `diesel` 来连接到 `postgresql`。  
而后，`wrapper` 部分会提供一个“视图”的抽象（即 `repository`），这里会把数据库中的原始实体包装成更加合适的结构。
注意：`wrapper` 模块中的 `models` 与 `database` 中的是不同的。  
此处会建立一套 Actor 系统，前端的连接请求会和这套 Actor 系统通信。  

# 最后要什么？
**读者**在首页阅读**文章**的简介。
首页一次性发送数篇新**文章**，**读者**可以**请求**更老的文章，也可以**搜索**满足条件的文章。
进入具体的**文章**页面后，**读者**可以**评论****文章**。