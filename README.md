# rCore

愿此行，终抵星海

基于 https://github.com/rcore-os/rCore-Tutorial-v3

目前进度阶段：MultiprogOS

新增的内容：

- 支持一次性加载多个app到内存不同区域
- 支持多应用任务切换
- 支持sys_yeild系统调用
- 保存内核的一些配置参数到`os/src/config.rs`

## 备注：关于数据布局


```
// os/src/task/context.rs line 3-12

// 经过多次debug，最终发现是缺了这里。
// 如果不加，经过gdb检查发现__switch处加载TaskContext（在a1寄存器）后ra为0，导致访问无效内存0x0从而使内核卡死，
// 而检查发现TaskContext发现内部的ra有效，于是考虑内存布局问题，从而解决。
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

```

chatGPT的解释：


    在Rust中，`#[repr(C)]`是一个属性，它改变了数据结构的内存布局以使其与C语言的内存布局兼容。`repr`是"representation"的缩写，`C`表示这种布局应与C语言的布局兼容。

    默认情况下，Rust编译器可以自由地调整数据结构的内存布局以优化性能。例如，它可能会重新排序结构体的字段或添加填充以改善内存访问的速度。然而，这种自由的内存布局可能会与其他语言（如C语言）的内存布局不兼容。

    当你需要与C语言（或任何假设C兼容内存布局的语言）的代码交互时，你可能需要使用`#[repr(C)]`来确保你的数据结构在内存中的布局与C语言的布局相同。例如，如果你正在编写一个Rust函数，该函数将被C代码调用并接收一个指向C结构体的指针，那么你需要使用`#[repr(C)]`来定义一个与该C结构体内存布局相同的Rust结构体。

    例如：

    ```rust
    #[repr(C)]
    struct MyStruct {
        a: u32,
        b: f32,
    }
    ```

    在这个例子中，`MyStruct`的内存布局将与C语言中具有相同字段类型的结构体的布局相同。

    请注意，使用`#[repr(C)]`可能会影响性能，因为它限制了编译器优化内存布局的能力。因此，你应该只在需要与C语言代码交互时使用它。


## 环境配置

使用VSCode开发容器自动配置，也可手动配置环境

参考 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html

## 运行

运行`run.sh`

或者运行`os/run_withgdb.sh`运行qemu，`os/gdb.sh`开启gdb调试
