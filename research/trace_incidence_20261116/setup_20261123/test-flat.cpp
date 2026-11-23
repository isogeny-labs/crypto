#include "flat.hpp"
#include <set>
#include <unordered_map>
#include <iostream>
int main(){try{
 flat_membership::Set set(10000);flat_membership::Map map(10000);std::set<std::uint64_t> reference;std::unordered_map<std::uint64_t,std::size_t> values;std::uint64_t checks=0;
 for(std::size_t i=0;i<100000;++i){const std::uint64_t key=flat_membership::mix(i%10000)&0x7fffffffffffffffULL;const bool actual=set.insert(key),expected=reference.insert(key).second;if(actual!=expected)throw std::runtime_error("set discrepancy");map.put(key,i);values[key]=i;++checks;}
 for(const auto&v:values){std::size_t value=0;if(!map.get(v.first,value)||value!=v.second)throw std::runtime_error("map discrepancy");++checks;}
 for(std::uint64_t i=0;i<10000;++i){const auto key=flat_membership::mix(i+20000)&0x7fffffffffffffffULL;std::size_t value=0;if(map.get(key,value)!=(values.find(key)!=values.end()))throw std::runtime_error("absent discrepancy");++checks;}
 flat_membership::Set collisions(16);flat_membership::Map collision_map(16);unsigned stored=0;
 for(std::uint64_t key=0;stored<16;++key)if((flat_membership::mix(key)&31)==0){if(!collisions.insert(key)||collisions.insert(key))throw std::runtime_error("collision set");collision_map.put(key,stored);std::size_t value=0;if(!collision_map.get(key,value)||value!=stored)throw std::runtime_error("collision map");++stored;checks+=3;}
 std::cout<<"{\"status\":\"passed\",\"checks\":"<<checks<<",\"scope\":\"exact set/map equivalence, duplicate updates, absent lookups and forced long collision chain\"}\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
