#include <chrono>
#include <iostream>
#include <thread>

#include "libcrust/include/libcrust.generated.h"

int main(int argc, char **argv)
{
    Backend *be = be_make();
    be_start(be);
    be_poke_later(be, 1000, 4711);
    for (int i = 0; i < 100; i++) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        be_poke_now(be, i);
    }
    std::this_thread::sleep_for(std::chrono::seconds(1));
    be_del(be);
}
