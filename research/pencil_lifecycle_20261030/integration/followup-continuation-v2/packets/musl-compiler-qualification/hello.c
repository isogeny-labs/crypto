#include <stdio.h>
int main(void) {
    if (puts("pencil-musl-smoke-ok") < 0) return 1;
    return 0;
}
