#include<stdio.h>
#include "tyro.h"

main() {
    Tyro tyro = new_tyro();

    hello(add100(tyro, 55));

    // CALL DESTRUCTORS (drop)
    drop_tyro(tyro);
    return 0;
}