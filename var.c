int main() {
    int x = 1;
    int y = x;
    struct test_t {
        int x;
        int y;
    };

    struct test_t t = [1,2];
    struct test_t s = t;

    return 0;
}

#include <stdio.h>

int main(){
    int x = 1;
    int* p = &x;
    int* q = p;
    printf("p-1: %d\n",*p); /* p-1: 1 */
    *q=2;

    /* qとpは同じメモリ領域を指しているので、pが指すメモリ領域に格納されている値も変化*/
    printf("p-2: %d\n", *p); /* p-2: 2 */

    /* x も変化する */
    printf("x:%d\n",x); /*x: 2 */

    return 0;
}
