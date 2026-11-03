// Development diagnostic on known public P-192 arithmetic, not a cohort retry.
#include "../../native/common.hpp"
#include <iostream>
int main() {
    pari_init(128000000, 2000000);
    GEN p=integer("0xfffffffffffffffffffffffffffffffeffffffffffffffff");
    GEN b=integer("0x64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1");
    GEN n=integer("0xFFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831");
    std::cerr << "before p primality\n" << std::flush;
    require((isprimeAPRCL(p)==1), "p");
    std::cerr << "before n primality\n" << std::flush;
    require((isprimeAPRCL(n)==1), "n");
    std::cerr << "before ellinit\n" << std::flush;
    GEN e=ellinit(mkvec2(stoi(-3),b),p,DEFAULTPREC);
    std::cerr << "before sqrt\n" << std::flush;
    GEN y=Fp_sqrt(b,p),q=mkvec2(gen_0,y);
    std::cerr << "before n multiplication\n" << std::flush;
    GEN z=liftall(ellmul(e,q,n));
    require(lg(z)==2&&gequal0(gel(z,1)),"nQ");
    std::cerr << "before b multiplication\n" << std::flush;
    (void)liftall(ellmul(e,q,b));
    std::cerr << "before bit and character functions\n" << std::flush;
    (void)popcount(ibitxor(y,b));
    (void)kronecker(modii(subii(y,b),p),p);
    std::cerr << "all primitive diagnostics completed\n" << std::flush;
    pari_close();
}
