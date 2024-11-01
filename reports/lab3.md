# Lab3

## 实现功能
实验要求我们先把先前实现的系统调用兼容，然后实现sys_spawn 和 stride调度算法。  

兼容部分：  
主要修改了sys_info的部分实现。由于TaskManager和Processor的解耦，在计算任务的运行开始时间时，需要在Processor中实现。  

sys_spawn:   
在task中实现spawn方法，不同于fork的是，直接为新进程创建MemorySet，分配内核栈。 
在process部分直接通过current_task获取任务的clone，调用即可。  

stride调度算法：  
首先为TCBInner结构中添加priority和stride成员，并给出设置优先级，前进步数等接口，方便外界通过TCB就可调用。  
修改TaskManage中fetch_task的实现，因为Processor从这里获取下一个任务。在这里实现了Stride方法的具体内容。  
补充了set_priority系统调用的实现。

## 问答作业

**理论执行**  
不是， 因为8bit最高就是255，如果不做溢出处理，p2.stride + 10 会溢出，从而比 p1.stride小，根据算法，应该是p2执行。  


**说明原因**  
出现倒置的情况，一般来说是一个低优先级，stride较小的进程在执行后溢出，其stride还是较小（优先级最高）。  
在这种情况下，我们需要保证的是，低优先级的进程不应该频繁被调度，所以要保证对于低优先级的进程，就算溢出了，新的stride也要比老stride大。  
体现在具体计算中，就是BigStride / 2 (低优先级的步进) 需要能够套圈存储Stride的范围（Stride_MAX - Stride_MIN）。

**代码**

```rust
use core::cmp::Ordering;

struct Stride(u64);
const MAX_STRIDE: u64 = 255;

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = (self.0 + MAX_STRIDE / 2) as u8;
        let other_val = (other.0 + MAX_STRIDE / 2) as u8;

        if self_val < other_val {
            Some(Ordering::Less)
        } else if self_val > other_val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        (self.0 % 128) == (other.0 % 128)
    }
}

fn main() {
    assert_eq!(Stride(129) < Stride(255), true);
    assert_eq!(Stride(125) < Stride(255), false);
    println!("Success");
}

```



##
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：  
无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：  
无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。