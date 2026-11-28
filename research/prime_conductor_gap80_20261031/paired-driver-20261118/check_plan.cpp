// Compare producer decimal records and consumer numeric/hex records exactly.
// Input shaping is jq only; no field arithmetic is delegated to the shell.
#include <gmpxx.h>
#include <array>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
int main(int argc,char**argv){try{
 if(argc!=3)throw std::runtime_error("expected producer and consumer TSV paths");
 std::ifstream producer(argv[1]),consumer(argv[2]);
 if(!producer||!consumer)throw std::runtime_error("input open failed");
 unsigned long rows=0;
 while(true){
   std::array<std::string,3> a,b;bool pa=bool(producer>>a[0]),pb=bool(consumer>>b[0]);
   if(pa!=pb)throw std::runtime_error("row count mismatch");
   if(!pa)break;
   if(!(producer>>a[1]>>a[2])||!(consumer>>b[1]>>b[2]))throw std::runtime_error("truncated row");
   for(unsigned j=0;j<3;j++)if(mpz_class(a[j],10)!=mpz_class(b[j],0))throw std::runtime_error("scalar/coordinate mismatch");
   rows++;
 }
 if(rows==0)throw std::runtime_error("empty comparison");
 std::cout<<"{\"scope\":\"producer/consumer scalar and coordinate equality only\",\"matching_rows\":"<<rows<<",\"timing_measurements\":null}\n";return 0;
}catch(const std::exception&e){std::cerr<<"check-plan: "<<e.what()<<'\n';return 2;}}
