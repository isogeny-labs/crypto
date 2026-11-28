#pragma once
#include <cstdint>
#include <cstddef>
#include <limits>
#include <vector>
#include <stdexcept>
namespace flat_membership {
using Word=std::uint64_t;
inline Word mix(Word x){x^=x>>30;x*=0xbf58476d1ce4e5b9ULL;x^=x>>27;x*=0x94d049bb133111ebULL;return x^(x>>31);}
inline std::size_t capacity(std::size_t wanted){std::size_t n=8;while(n/2<wanted){if(n>std::numeric_limits<std::size_t>::max()/2)throw std::length_error("flat capacity");n*=2;}return n;}
// Keys are canonical field coordinates below2^62; UINT64_MAX is unavailable.
// Exact equality resolves every collision. Hashing affects cost, not acceptance.
class Set{
 static constexpr Word empty=std::numeric_limits<Word>::max();std::vector<Word> keys_;std::size_t mask_;
public:
 explicit Set(std::size_t n):keys_(capacity(n),empty),mask_(keys_.size()-1){}
 bool insert(Word key){if(key==empty)throw std::invalid_argument("reserved flat key");auto slot=std::size_t(mix(key))&mask_;while(keys_[slot]!=empty){if(keys_[slot]==key)return false;slot=(slot+1)&mask_;}keys_[slot]=key;return true;}
};
class Map{
 static constexpr Word empty=std::numeric_limits<Word>::max();struct Slot{Word key=empty;std::size_t value=0;};std::vector<Slot> slots_;std::size_t mask_;
public:
 explicit Map(std::size_t n):slots_(capacity(n)),mask_(slots_.size()-1){}
 void put(Word key,std::size_t value){if(key==empty)throw std::invalid_argument("reserved flat key");auto slot=std::size_t(mix(key))&mask_;while(slots_[slot].key!=empty&&slots_[slot].key!=key)slot=(slot+1)&mask_;slots_[slot]={key,value};}
 bool get(Word key,std::size_t&value)const{auto slot=std::size_t(mix(key))&mask_;while(slots_[slot].key!=empty){if(slots_[slot].key==key){value=slots_[slot].value;return true;}slot=(slot+1)&mask_;}return false;}
};
} // namespace flat_membership
