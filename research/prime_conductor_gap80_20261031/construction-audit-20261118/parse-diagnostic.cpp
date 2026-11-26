#include <pari/pari.h>
#include <iostream>
int main(){pari_init(1000000,10000);GEN x=strtoi("-140");char*s=GENtostr(x);std::cout<<s<<'\n';pari_free(s);pari_close();}
