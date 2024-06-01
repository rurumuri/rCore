# rCore

愿此行，终抵星海

基于 https://github.com/rcore-os/rCore-Tutorial-v3

目前进度阶段：MultiprogOS

新增的内容：

- 支持一次性加载多个app到内存不同区域
- 支持多应用任务切换
- 支持 `sys_yeild`、`sys_get_time`系统调用
- 支持时钟中断和分时多任务
- 保存内核的一些配置参数到`os/src/config.rs`

## 备注

### 关于数据布局导致的问题

```Rust
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

尝试下面的代码：

```Rust
use std::mem::{align_of, size_of};

struct RustStruct { ra: usize, sp: usize, s: [usize; 12], }
#[repr(C)]
struct CStruct { ra: usize, sp: usize, s: [usize; 12], }

fn main() {
    let r = RustStruct { ra: 0, sp: 0, s: [0; 12] };
    let c = CStruct { ra: 0, sp: 0, s: [0; 12] };

    let r_addr = &r as *const RustStruct as usize;
    let r_ra_addr = &r.ra as *const usize as usize;
    let r_sp_addr = &r.sp as *const usize as usize;
    let r_s_addr = &r.s as *const [usize; 12] as usize;

    let c_addr = &c as *const CStruct as usize;
    let c_ra_addr = &c.ra as *const usize as usize;
    let c_sp_addr = &c.sp as *const usize as usize;
    let c_s_addr = &c.s as *const [usize; 12] as usize;

    println!("RustStruct r\nr_start: {:#x}\nra: {:#x}\nsp: {:#x}\ns: {:#x}", r_addr, r_ra_addr, r_sp_addr, r_s_addr);
    println!("CStruct c\nc_start: {:#x}\nra: {:#x}\nsp: {:#x}\ns: {:#x}", c_addr, c_ra_addr, c_sp_addr, c_s_addr);
}
```

输出为：

```
RustStruct r
r_start: 0x7ffea9971e18
ra: 0x7ffea9971e78
sp: 0x7ffea9971e80
s: 0x7ffea9971e18
CStruct c
c_start: 0x7ffea9971ee8
ra: 0x7ffea9971ee8
sp: 0x7ffea9971ef0
s: 0x7ffea9971ef8
```

可见C布局下`CStruct`0偏移量处即为ra，而Rust布局下并非如此

简单来说，Rust对`TaskContext`结构体的默认布局方式是`repr(Rust)`，编译器可能进行字段重排、填充等方式进行对齐和优化；而在`extern "C"`的函数`__switch`中发生了跨FFI边界访问，这要求`TaskContext`结构体使用`repr(C)`即C的方式布局。两者布局未达成一致，导致`__switch`中使用偏移量访问`TaskContext`时读到了无效值，从而导致了访问无效内存0x0，使内核卡住等问题。

参考：

- 《Rust 死灵书》（The Rustonomicon） - Rust 中的数据布局：https://nomicon.purewhite.io/data.html
- 《Rust 参考手册》（The Rust Reference） - 类型布局：https://rustwiki.org/zh-CN/reference/type-layout.html
- rust-bindgen：https://rust-lang.github.io/rust-bindgen/introduction.html

### 关于内核boot_stack栈区域大小的尝试

```asm
# os/src/entry.asm line 8-12
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
```

尝试在`os/src/main.rs`的`rust_main`中调用以下函数：

```Rust
fn kernel_stack_test(x: usize) {
    info!("[kernel] stack test {}", x);
    kernel_stack_test(x+1);
}
```

并多次修改`.space`的大小，发现测试函数递归次数确实不同，从而证实了`boot_stack`的作用

然而，在设置`.space`为0后，测试函数仍然能够递归很少的几次（或者能正常执行此前`rust_main`中的内核函数调用），暂时不清楚原本的栈信息存到了哪里，存疑

## 环境配置

使用VSCode开发容器自动配置，也可手动配置环境

参考 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html

## 运行

运行`run.sh`

或者运行`os/run_withgdb.sh`运行qemu，`os/gdb.sh`开启gdb调试
