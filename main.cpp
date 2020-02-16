#include <chrono>
#include <iostream>
#include <thread>
#include <stdlib.h>
#include <time.h>

#include "libcrust/include/libcrust.generated.h"

int main(int argc, char **argv)
{
    srand(time(NULL));
    Backend *b1 = be_make("b1");
    be_start(b1);
    Backend *b2 = be_make("b2");
    be_start(b2);
    be_poke_later(b1, 1000, 4711);
    for (int i = 0; i < 100; i++)
    {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        if (rand() % 10 < 7) {
            be_poke_now(b1, i);
        } else {
            be_poke_now(b2, i);
        }
    }
    std::this_thread::sleep_for(std::chrono::seconds(1));
    be_del(b1);
    be_del(b2);
}
