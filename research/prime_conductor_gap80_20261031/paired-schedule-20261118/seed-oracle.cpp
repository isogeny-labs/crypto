// Independent canonical seed vectors; no EC arithmetic or timer.
#include <openssl/sha.h>
#include <cstdint>
#include <iostream>
#include <string>
int main() {
  std::cout<<"{\"schema\":\"paired-curves-v1-seed-vectors\",\"columns\":[\"r\",\"target_index\",\"warmup\",\"seed_round\",\"seed\"],\"rows\":[";
  bool first=true;
  for(uint64_t r:{uint64_t(41),uint64_t(1125023250121),uint64_t(125000000302579)})
    for(uint64_t index=0;index<(r==41?4U:32U);++index)
      for(bool warmup:{true,false})
        for(uint64_t round=0;round<(warmup?3U:7U);++round) {
          std::string domain=warmup?"ecbench.paired_curves_v1.warmup_seed":"ecbench.paired_curves_v1.algorithm_seed";
          std::string text="{\"domain\":\""+domain+"\",\"parts\":[20261118,"+std::to_string(r)+",20261118,"+std::to_string(index)+","+std::to_string(round)+"]}";
          unsigned char digest[SHA256_DIGEST_LENGTH];
          SHA256(reinterpret_cast<const unsigned char*>(text.data()),text.size(),digest);
          uint64_t value=0;for(unsigned i=0;i<8;++i)value=(value<<8)|digest[i];
          if(!first)std::cout<<',';
          first=false;
          std::cout<<'['<<r<<','<<index<<','<<(warmup?"true":"false")<<','<<round<<','<<value<<']';
        }
  std::cout<<"]}\n";
}
