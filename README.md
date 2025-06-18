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

缓慢更新中，get fucked by rust...

**ELF (Executable_and_Linkable_Format) 文件格式：**
![elf](https://upload.wikimedia.org/wikipedia/commons/thumb/e/e4/ELF_Executable_and_Linkable_Format_diagram_by_Ange_Albertini.png/2880px-ELF_Executable_and_Linkable_Format_diagram_by_Ange_Albertini.png)



**ar (archive) 文件格式：**
![ar](https://upload.wikimedia.org/wikipedia/commons/thumb/6/67/Deb_File_Structure.svg/1024px-Deb_File_Structure.svg.png)
