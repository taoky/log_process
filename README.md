# log_process

入门 Rust 的练习：统计 nginx/rsyncd 日志文件中的请求次数和单独 IP 数。

- Nginx 额外计算了浏览器（Mozilla）访问主页的请求次数和单独 IP 数。
- Rsyncd 日志额外设置了时间范围，虽然现在是 hard code 进代码的……

用 Rust 写的第一个实用脚本。感觉写小脚本虽然不如 Python 写起来快，但是基本上编译过了就不会出问题，而且性能很好。

## 还可以做的改进

- 并行化
- 自定义 rsyncd 日志起始和结束时间
