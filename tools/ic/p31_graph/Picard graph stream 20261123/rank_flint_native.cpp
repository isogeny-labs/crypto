// Exact FLINT rank/determinant of a projected p31 factor-base matrix.
#include <flint/flint.h>
#include <flint/nmod_mat.h>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>

int main(int argc,char**argv){
 if(argc!=3)throw std::runtime_error("usage: rank_flint_native matrix.bin n");
 const uint64_t ell=11832483282198303559ULL;
 uint32_t n=std::stoul(argv[2]);if(n<1||n>25000)throw std::runtime_error("n out of range");
 std::ifstream in(argv[1],std::ios::binary|std::ios::ate);
 if(!in||in.tellg()!=std::streamoff(uint64_t(n)*n*8))throw std::runtime_error("matrix size mismatch");
 in.seekg(0);
 nmod_mat_t A;nmod_mat_init(A,n,n,ell);
 for(uint32_t i=0;i<n;++i){
  for(uint32_t j=0;j<n;++j){
   uint64_t value;
   if(!in.read(reinterpret_cast<char*>(&value),8)||value>=ell)throw std::runtime_error("bad matrix entry");
   nmod_mat_entry(A,i,j)=value;
  }
  if((i+1)%1000==0)std::cerr<<"loaded rows "<<i+1<<" / "<<n<<'\n';
 }
 ulong determinant=nmod_mat_det(A);
 std::cout<<"{\"rows\":"<<n<<",\"ell\":"<<ell<<",\"determinant_mod_ell\":"<<determinant
          <<",\"full_row_rank_certified\":"<<(determinant?"true":"false")<<"}\n";
 nmod_mat_clear(A);
}
