# 实验概述
## 接口介绍
此实验实现了syscall sys_task_info, 通过在任务控制块中维护任务的执行系统调用的次数(syscall_num) 还有最开始的运行时间的信息(start_time)。

## 问答题
1. 使用的RustSBI version: 0.3.0-alpha.2
   内核报错信息为：
   [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400414, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   
2. 
	(1)a0保存了调用__restore时的第一个参数。 它是TrapContext上下文的地址。作用于从从特权态中恢复。
	(2)这几行汇编代码特殊处理了sstatus, sepc, sscratch三个csr寄存器， 这些状态寄存器对于用户态的意义是完成特权级的切换与恢复。
		sstatus表示CPU所处的特权级
		sepc表示Trap结束后的下一条指令地址
		sscratch指向当前应用的内核栈栈顶
	(3)户栈指针x2指向内核栈，而用户栈指针已经被保存到sscratch。 而x4一般不使用
	(4)sp是用户栈指针， sscratch是内核栈指针
	(5)__restore发生状态切换在sret指令， 执行sert指令后会返回到用户态的程序内容中继续执行
	(6)执行后sp是内核栈指针， 而sscratch是用户栈指针
	(7)从 U 态进入 S 态是ecall指令发出, 调用后进入stvec寄存器的Trap地址开始执行

## 荣誉准测
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

 rcore-tutorial-v3

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。