#include <iostream>
#include <vector>
#include <string>
#include <chrono>
#include <cassert>
#include <sys/resource.h>
#include "../k0_zero_cubic_20261115_v43/lab_variant_02/native_gf83.h"
using E=gf83::Element;
E inv(E x){assert(x);E t=x;for(int i=1;i<82;i++)t=gf83::mul(gf83::square(t),x);t=gf83::square(t);assert(gf83::mul(x,t)==1);return t;}
E trace(E x){E t=x;for(int i=1;i<83;i++){x=gf83::square(x);t^=x;}assert(t<2);return t;}
int parity(E x){return(__builtin_popcountll((uint64_t)x)+__builtin_popcountll((uint64_t)(x>>64)))&1;}
std::string hx(E x){if(!x)return"0";std::string r;while(x){r="0123456789abcdef"[int(x&15)]+r;x>>=4;}return r;}
int main(){auto start=std::chrono::steady_clock::now();E mask=0;for(int i=0;i<83;i++)mask|=trace(E(1)<<i)<<i;unsigned N=1u<<18;std::vector<E>p(N);p[0]=1;for(unsigned x=1;x<N;x++)p[x]=gf83::mul(p[x-1],E(x));E v=inv(p.back());uint64_t rational=0,anti=0;
for(unsigned x=N-1;x>0;x--){E ix=gf83::mul(v,p[x-1]);v=gf83::mul(v,E(x));assert(gf83::mul(ix,x)==1);E t=E(x)^ix;int b=parity(t&mask);if(x<=64)assert(b==trace(t)&&gf83::mul_portable(ix,x)==1);if(b)anti++;else rational++;}
uint64_t points=2*rational+1,pairs=2*rational*(rational+1),bytes=16*pairs;unsigned indexbits=0;while((uint64_t(1)<<indexbits)<points)indexbits++;assert(84+2*indexbits<=128);rusage usage{};getrusage(RUSAGE_SELF,&usage);
std::cout<<"{\"nonzero_rational_x\":"<<rational<<",\"nonrational_x\":"<<anti<<",\"torsion_x\":1,\"source_points\":"<<points<<",\"finite_unordered_pair_records\":"<<pairs<<",\"record_bytes\":16,\"flat_pair_bytes\":"<<bytes<<",\"source_index_bits\":"<<indexbits<<",\"trace_mask_hex\":\""<<hx(mask)<<"\",\"cap_bytes\":4294967296,\"refuse_flat_materialization\":"<<(bytes>4294967296ULL?"true":"false")<<",\"complete_domain_profile\":true,\"seconds\":"<<std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()<<",\"peak_rss_kib\":"<<usage.ru_maxrss<<"}"<<std::endl;
}
