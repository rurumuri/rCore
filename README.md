# rCore

愿此行，终抵星海

基于 https://github.com/rcore-os/rCore-Tutorial-v3，仓库内的`os`目录包含跟随教程逐步实现的rCore OS

目前进度：LibOS，能输出字符串和关机

## 运行

使用VSCode开发容器全自动配置，也可手动配置环境

运行`os/run.sh`直接运行qemu

运行`os/run_withgdb.sh`运行qemu，在另一个终端运行`os/gdb.sh`开启gdb调试
