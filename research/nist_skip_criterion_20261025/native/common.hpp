#pragma once
#include <pari/pari.h>
#include <openssl/sha.h>
#include <array>
#include <cstdint>
#include <iomanip>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

inline void require(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
inline std::string decimal(GEN n) {
    char* s=GENtostr(n); std::string out(s); pari_free(s); return out;
}
inline GEN integer(const std::string& s) {
    require(!s.empty() && s.find_first_not_of("0123456789abcdefABCDEFxX")==std::string::npos,"invalid integer");
    return gp_read_str(s.c_str());
}
inline GEN from_bytes(const unsigned char* data, size_t size) {
    std::ostringstream out; out<<"0x"<<std::hex<<std::setfill('0');
    for (size_t i=0;i<size;++i) out<<std::setw(2)<<unsigned(data[i]);
    return integer(out.str());
}
inline std::array<unsigned char,20> bytes20(GEN n) {
    std::array<unsigned char,20> out{};
    for(int i=19;i>=0;--i){out[i]=umodiu(n,256); n=shifti(n,-8);}
    require(signe(n)==0,"seed exceeds 160 bits"); return out;
}
inline GEN stream_seed(uint32_t run,uint32_t index) {
    require(index>0,"index is one based");
    uint64_t state=(uint64_t(run)<<16)|0x330e;
    std::array<unsigned char,20> block{};
    for(uint32_t i=0;i<index;++i) for(auto& c:block){
        state=(state*UINT64_C(0x5deece66d)+0xb)&((UINT64_C(1)<<48)-1);
        c=state>>40;
    }
    return from_bytes(block.data(),block.size());
}
inline GEN sha1_integer(GEN seed) {
    auto bytes=bytes20(seed); unsigned char digest[20];
    SHA1(bytes.data(),bytes.size(),digest); return from_bytes(digest,20);
}
inline GEN x962(GEN seed,long bits) {
    long chunks=(bits-1)/160, first=bits-160*chunks-1;
    GEN out=modii(sha1_integer(seed),int2n(first));
    for(long i=1;i<=chunks;++i)
        out=addii(shifti(out,160),sha1_integer(modii(addis(seed,i),int2n(160))));
    return out;
}
inline std::vector<int> primes_through(int maximum) {
    std::vector<int> primes; std::vector<bool> composite(maximum+1);
    for(int i=2;i<=maximum;++i){if(composite[i])continue;primes.push_back(i);
        if(i<=maximum/i)for(int j=i*i;j<=maximum;j+=i)composite[j]=true;}
    return primes;
}
inline long bit_length(GEN n) {return signe(n)==0?0:expi(absi(n))+1;}
inline long popcount(GEN n) {
    long result=0; n=absi(n);
    while(signe(n)){result+=__builtin_popcount(umodiu(n,UINT64_C(1)<<32));n=shifti(n,-32);}
    return result;
}
