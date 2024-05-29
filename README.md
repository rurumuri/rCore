# rCore

愿此行，终抵星海

基于 https://github.com/rcore-os/rCore-Tutorial-v3

目前进度阶段：MultiprogOS

新增的内容：

- 支持一次性加载多个app到内存不同区域
- 保存内核的一些配置参数到`os/src/config.rs`

## 环境配置

使用VSCode开发容器自动配置，也可手动配置环境

参考 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html

## 运行

运行`run.sh`

或者运行`os/run_withgdb.sh`运行qemu，`os/gdb.sh`开启gdb调试
