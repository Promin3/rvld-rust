# rvld
rvld 是一个针对 RV64GC 架构的最小链接器实现

rvld 可以静态链接一个简单的 C 程序（例如下面示例中的 Hello world），并生成可运行的二进制。

```bash
cat <<EOF | $CC -o a.o -c -xc -static -
#include <stdio.h>
int main() {
  printf("Hello, World.\n");
  return 0;
}
EOF

$CC -B. -s -static a.o -o out
qemu-riscv64 out

# Hello, World.
```
