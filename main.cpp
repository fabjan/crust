#include <iostream>

#include "libcrust/include/libcrust.generated.h"

int main(int argc, char **argv)
{
    Foo *f = make(4711);
    std::cout << "init: " << get(f) << std::endl;

    bump(f);
    std::cout << "bumped: " << get(f) << std::endl;

    bump(f);
    std::cout << "again: " << get(f) << std::endl;

    Baz b;
    
    b = baz(f);
    std::cout << "baz qux: " << b.qux << std::endl;

    fix(f);
    b = baz(f);
    std::cout << "baz qux (fixed): " << b.qux << std::endl;

    del(f);
}
