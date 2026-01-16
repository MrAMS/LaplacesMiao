# LaplacesMiao

<table>
  <tr>
    <td valign="middle" width="35%" style="border: none;">
      <img src="doc/asserts/LaplacesMiao.webp" width="100%" />
    </td>
    <td valign="middle" style="border: none; padding-left: 20px;">
      <h3>A highly extensible tiny compiler written in Rust.</h3>
    </td>
  </tr>
</table>

## Language

### LasMiao

A High-level Dataflow DSL：
- Pure Dataflow, Purely Functional
- Declarative Ops, Native Tensor Semantics
- Heterogeneous-Aware, Explicit Orchestration

#### Examples

```scala
a:f32 = 1.+sin(pi)
f = (x => sin(x)+cos(x))
c = b.map(x => f(x)+1.)

list_on_cpu = [[1:i32,2],[3,4],[5,6]]@cpu
xpuN#1024 // meta define for custom compiler pass
tensor_on_xpu:tensor(i32, 3, 2) = list_on_cpu@xpu

buffer_on_cpu = $(1024, input_tensor)@cpu
buffer_on_xpu = $(1024, sram)@xpu

m3:my_type = _xpu_acc(m1,m2)
```

#### Vision

Algorithms should first be described in **pure** computation and data flow using the **LasMiao** DSL. Users then implement custom operators or DSA-specific optimizations by writing passes within the **LaplacesMiao** compiler. Finally, the compiler generates the DSA-executable code.

Philosophy: We believe **compilers, not users, should write the kernels**. Users should focus on the **meta-design** (writing the passes for compiler to generates the kernel), rather than hand-crafting the kernel itself.
