#!/bin/bash

test_name=$(basename "$0" .sh) # $0 means the script name without path, .sh means delete the extension
t=out/tests/$test_name

mkdir -p "$t"

cat <<EOF | $CC -o "$t"/a.o -c -xc -
#include <stdio.h>

int main(void) {
    printf("Hello, World\n");
    return 0;
}
EOF

$CC -B. -static "$t"/a.o -o "$t"/out