# 问答题

stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

- 实际情况是轮到 p1 执行吗？为什么？

> 不是，因为整数溢出（在 rust 的 debug 模式下将 panic），p2.stride = 4，轮到 p2 执行。

我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

- 为什么？尝试简单说明（不要求严格证明）。

> 假设 STRIDE_MAX – STRIDE_MIN > BigStride / 2，那么 STRIDE_MAX 的那个任务上次运行时，stride = STRIDE_MAX - BigStride / prio，但 prio 最小是 2。可知，当时该任务的 stride 比 STRIDE_MIN 大，不应该被挑出来运行，矛盾。

- 已知以上结论，**考虑溢出的情况下**，可以为 Stride 设计特别的比较器，让 BinaryHeap\<Stride\> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 `partial_cmp` 函数，假设两个 Stride 永远不会相等。

```rust
use core::cmp::{Ordering, Reverse};

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if u64::abs_diff(self.0, other.0) > BIG_STRIDE / 2 {
            Reverse(self.0).partial_cmp(&Reverse(other.0))
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

TIPS: 使用 8 bits 存储 stride, BigStride = 255, 则: `(125 < 255) == false`, `(129 < 255) == true`.
