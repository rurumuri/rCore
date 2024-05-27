# rCore

愿此行，终抵星海

基于 https://github.com/rcore-os/rCore-Tutorial-v3 ，仓库内的`os`目录包含跟随教程逐步实现的rCore OS

目前进度阶段：BatchOS

- 支持用户态和内核态的切换
- 支持加载单个预先编译的app并在用户态执行
- 为用户态app支持系统调用

## 环境配置

使用VSCode开发容器自动配置，也可手动配置环境

参考 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html

## 运行

运行`run.sh`

或者运行`os/run_withgdb.sh`运行qemu，`os/gdb.sh`开启gdb调试
