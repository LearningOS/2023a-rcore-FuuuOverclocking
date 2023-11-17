## 简答题
### 1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 [Rust 三个 bad 测例 (ch2b_bad_\*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2023A/tree/master/src/bin) ， 注意在编译时至少需要指定 `LOG=ERROR` 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

使用的 SBI 版本：RustSBI-QEMU Version 0.2.0-alpha.2

三个 bad 测例输出：
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

分别对应 `ch2b_bad_address`, `ch2b_bad_instructions`, `ch2b_bad_register`。

### 2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2023A/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

#### 2.1 L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

代表 `TrapContext`。

情况 1：非切换进程，例如系统调用

此时 `trap_handler(..)` 完成调用正常返回，恢复用户态上下文。

情况 2：切换进程，例如第一个任务运行、任务切换

在 `__restore` 之前完成了内核栈的换栈，切换成了新任务的内核栈。然后从该栈中恢复新任务的用户态上下文。

#### 2.2 L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

特殊处理了
- `sstatus`: `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息。应为 U。
- `sepc`: 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址。应为用户态程序挂起的位置的下一条指令。
- `sscratch`: 用户栈的 `sp`。

#### 2.3 L50-L56：为何跳过了 `x2` 和 `x4`？

x2（用户栈）已经存到 t2 了，它是栈顶指针，修改它会妨碍从栈里恢复上下文，在最后修改。
x4 是线程指针，未使用。

#### 2.4 L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

该指令之后，sp 指向用户栈，sscratch 指向内核栈

#### 2.5 `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

sret，因为 `sstatus` 指出了 Trap 之前 CPU 处于 U 态。

#### 2.6 L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

之后，`sp` 指向内核栈，`sscratch` 指向用户栈。

#### 2.7 从 U 态进入 S 态是哪一条指令发生的？

ecall 或者各种错误。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    > 无
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
    > risc-v 手册
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。