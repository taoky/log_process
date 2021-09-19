# log_process

入门 Rust 的练习：分析 mirrors 的 nginx/rsyncd 日志，并统计一些信息。

模块：

- nginx: 计算日志的请求数与访问的 IP 数，此外额外计算了浏览器（Mozilla）访问主页的请求次数和单独 IP 数。
- rsyncd: 计算文件的请求数与访问的 IP 数，日志额外设置了时间范围，虽然现在是 hard code 进代码的……
- nginx_json: 计算 ngx_json 格式的日志，按照 IP 段（IPv4 /24, IPv6 /48）和 UA 归类，给出流量大于 1GB 的所有的 IP 段和 UA 信息。

用 Rust 写的第一个实用脚本。感觉写小脚本虽然不如 Python 写起来快，但是基本上编译过了就不会出问题，而且性能很好。

## 还可以做的改进

- 并行化
- 自定义 rsyncd 日志起始和结束时间
