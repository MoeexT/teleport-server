#include <stdio.h>

#define link(a, b) a ## b

int main() {
    int i = 0;
    int link(a, this(i));  // athis(i)

}

int this(int i) {
    return i;
}
