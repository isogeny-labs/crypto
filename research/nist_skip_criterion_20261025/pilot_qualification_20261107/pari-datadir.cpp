// Build metadata only: no curve construction or point counting.
#include <pari/pari.h>
#include <iostream>

int main() {
    std::cout << paricfg_datadir << '\n';
}
