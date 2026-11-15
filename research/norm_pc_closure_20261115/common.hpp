#pragma once
#include "../macaulay_provenance_20261115/common.hpp"
namespace pc {
using namespace mca;
constexpr std::size_t node_cap=40000;
struct Node {
    char kind;
    std::size_t first=0,second=0;
    std::vector<std::pair<std::size_t,U>> links;
    Polynomial poly;
};
inline void check_degree(const Polynomial&f,unsigned d){require(f.largest_degree()<=d,"node degree cap");}
inline Polynomial variable_multiple(const Polynomial&f,std::size_t variable){
    Monomial m(f.n);m.at(variable)=1;return multiple(f,m);
}
} // namespace pc
