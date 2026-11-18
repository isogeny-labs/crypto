#include <array>
#include <cstddef>
#include <cstdint>
#include "../k0_zero_cubic_20261115_v43/lab_variant_02/native_gf83.h"
using E=gf83::Element;using R=std::array<E,5>;
R squaremod(const R&A,const R&H){std::array<E,9>T{};for(unsigned i=0;i<5;i++)T[2*i]=gf83::square(A[i]);for(int j=8;j>=5;j--){E v=T[j];T[j]=0;if(v)for(unsigned i=0;i<5;i++)T[j-5+i]^=gf83::mul(v,H[i]);}R out{};for(unsigned i=0;i<5;i++)out[i]=T[i];return out;}
extern "C" int support83(const uint64_t*data,std::size_t count,uint8_t*out){
 if(count>2048)return -1;static const std::array<E,18>lam=[](){std::array<E,18>L{};for(unsigned i=0;i<18;i++){E x=E(1)<<i;for(unsigned j=0;j<i;j++)x=gf83::square(x)^gf83::mul(L[j],x);L[i]=x;}return L;}();
 for(std::size_t k=0;k<count;k++){
  std::array<E,6>poly{};for(unsigned j=0;j<6;j++){poly[j]=E(data[k*12+2*j])|(E(data[k*12+2*j+1])<<64);if(!gf83::canonical(poly[j]))return -2;}if(poly[5]!=1)return -3;
  R H{},r{};for(unsigned j=0;j<5;j++)H[j]=poly[j];r[1]=1;
  for(E c:lam){R next=squaremod(r,H);for(unsigned j=0;j<5;j++)next[j]^=gf83::mul(c,r[j]);r=next;}
  for(unsigned j=0;j<3;j++)r=squaremod(r,H);E value=0;for(E v:r)value|=v;out[k]=value==0;
 }
 return 0;
}
