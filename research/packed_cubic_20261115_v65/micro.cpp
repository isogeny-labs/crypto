#include <algorithm>
#define main scanner_main
#include "scan.cpp"
#undef main
static uint64_t rng=65001,cases=0,brute=0;
static uint64_t next64(){rng^=rng<<13;rng^=rng>>7;rng^=rng<<17;return rng;}
static void check(const std::vector<uint64_t>& a,uint64_t b,int rows,bool exhaustive){
 int low=rows<48?rows:48,high=rows-low,n=int(a.size());std::array<Row,83> cols{};
 E lowmask=(E(1)<<low)-1;for(int i=0;i<n;++i)cols[i]={E(a[i])&lowmask,E(a[i])>>low};Row rhs{E(b)&lowmask,E(b)>>low};Family got=packed_solve(cols,rhs,n,low,high),ref=solve(cols,rhs,n);
 if(got.consistent!=ref.consistent||got.rank!=ref.rank||got.kernel!=ref.kernel||(got.consistent&&got.particular!=ref.particular))throw std::runtime_error("micro family mismatch");
 if(exhaustive){std::vector<uint64_t> want,actual;for(uint64_t x=0;x<(uint64_t(1)<<n);++x){uint64_t v=0;for(int i=0;i<n;++i)if((x>>i)&1)v^=a[i];if(v==b)want.push_back(x);++brute;}if(got.consistent)for(uint64_t x=0;x<(uint64_t(1)<<got.kernel.size());++x){E v=got.particular;for(size_t i=0;i<got.kernel.size();++i)if((x>>i)&1)v^=got.kernel[i];actual.push_back(uint64_t(v));}std::sort(actual.begin(),actual.end());if(actual!=want)throw std::runtime_error("independent brute solution mismatch");}
 ++cases;
}
int main(){try{
 for(int m=0;m<=3;++m)for(int n=0;n<=3;++n)for(uint64_t encoding=0;encoding<(uint64_t(1)<<(m*n));++encoding){std::vector<uint64_t>a(n);for(int i=0;i<n;++i)a[i]=(encoding>>(m*i))&((uint64_t(1)<<m)-1);for(uint64_t b=0;b<(uint64_t(1)<<m);++b)check(a,b,m,true);}
 for(int width: {52,64})for(int defect=0;defect<=4;++defect)for(int trial=0;trial<16;++trial){std::vector<uint64_t>a(width);int rank=width-defect;for(int i=0;i<rank;++i)a[i]=uint64_t(1)<<i;uint64_t b=(defect&&trial%2)?uint64_t(1)<<rank:next64()&((rank==64)?UINT64_MAX:((uint64_t(1)<<rank)-1));
  for(int step=0;step<128;++step){int i=next64()%width,j=next64()%width;if(i==j)continue;for(auto& c:a)if((c>>j)&1)c^=uint64_t(1)<<i;if((b>>j)&1)b^=uint64_t(1)<<i;}
  for(int step=0;step<128;++step){int i=next64()%width,j=next64()%width;if(i!=j)a[i]^=a[j];}check(a,b,width,false);
 }
 std::cout<<"{\"cases\":"<<cases<<",\"brute_assignments\":"<<brute<<",\"widths\":[52,64],\"rank_defects\":[0,1,2,3,4]}\n";return 0;
}catch(const std::exception& e){std::cerr<<e.what()<<"\n";return 2;}}
