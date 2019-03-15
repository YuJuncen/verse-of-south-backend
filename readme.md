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
试试看是不是真的创建了：
```bash
diesel migration redo
```