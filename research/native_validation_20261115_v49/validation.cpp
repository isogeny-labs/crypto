#include <vector>
#include "../native_quintic_support_20261115_v48/support.cpp"
int bitop(E v){return(v>>64)?127-__builtin_clzll((uint64_t)(v>>64)):63-__builtin_clzll((uint64_t)v);}
E portable_eval(const E*P,uint64_t x){E v=0;for(int i=5;i>=0;i--)v=gf83::mul_portable(v,E(x))^P[i];return v;}
extern "C" int verify83(const uint64_t*matrix,std::size_t m,const uint64_t*checks,std::size_t count,const uint64_t*bases,std::size_t basis_count){
 if(m>83||count>2048)return -1;std::vector<E>polys(6*(m+1));for(std::size_t j=0;j<polys.size();j++){polys[j]=E(matrix[2*j])|(E(matrix[2*j+1])<<64);if(!gf83::canonical(polys[j]))return -2;}
 E mask=(E(1)<<m)-1;
 for(std::size_t k=0;k<count;k++){
  const uint64_t*C=checks+8*k;uint64_t x=C[0],expected=C[1],consistent=C[2],number=C[3],offset=C[6];if(x>=(1u<<18)||expected>m||consistent>1||offset>basis_count||number>basis_count-offset)return -3;
  std::vector<E>values(m);std::array<E,83>rows{},piv{};std::array<unsigned,83>rhsPivot{};E rhs=portable_eval(polys.data(),x);
  for(std::size_t j=0;j<m;j++){E v=portable_eval(polys.data()+6*(j+1),x);values[j]=v;while(v){int p=bitop(v);rows[p]|=E(1)<<j;v^=E(1)<<p;}}
  unsigned rank=0;bool bad=false;
  for(unsigned i=0;i<83;i++){E row=rows[i];unsigned r=(rhs>>i)&1;while(row){int p=bitop(row);if(piv[p]){row^=piv[p];r^=rhsPivot[p];}else{piv[p]=row;rhsPivot[p]=r;rank++;break;}}if(!row&&r)bad=true;}
  if(rank!=expected||(!bad)!=bool(consistent)||number!=m-rank)return int(100*k+10);
  auto image=[&values](E bits){E v=0;while(bits){int p=bitop(bits);v^=values[p];bits^=E(1)<<p;}return v;};
  E part=E(C[4])|(E(C[5])<<64);if(part&~mask)return int(100*k+11);if(consistent&&image(part)!=rhs)return int(100*k+12);
  std::array<E,83>kpiv{};unsigned kr=0;
  for(uint64_t i=0;i<number;i++){E v=E(bases[2*(offset+i)])|(E(bases[2*(offset+i)+1])<<64);if((v&~mask)||image(v))return int(100*k+13);while(v){int p=bitop(v);if(kpiv[p])v^=kpiv[p];else{kpiv[p]=v;kr++;break;}}}
  if(kr!=number)return int(100*k+14);
 }
 return 0;
}
