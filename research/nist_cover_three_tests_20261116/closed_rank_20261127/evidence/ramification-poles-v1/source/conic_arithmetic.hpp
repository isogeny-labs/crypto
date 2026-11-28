#include "sections_arithmetic.hpp"
#include <optional>

using CV=std::array<F,6>;
using Anchors=std::array<int,4>;
struct CPoint {SPoint value;CV monomial,derivative;};
struct CSection {CV coefficients;std::map<int,int> support;SCRow row;};
F cdot(const CV&a,const CV&b){F v;for(size_t i=0;i<6;++i)v=v+a[i]*b[i];return v;}
CV cnormalize(CV q){size_t i=0;while(i<6&&q[i]==F(0))++i;require(i<6,"nonzero conic");F inverse=q[i].inverse();for(auto&v:q)v=v*inverse;return q;}
std::string ckey(const CV&q){std::string out;for(const auto&v:q)out+=decimal(v)+":";return out;}
BP cbp(const CV&q){BP p;const std::array<Exponent,6> e{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};for(size_t i=0;i<6;++i)p.term(e[i],q[i]);return p;}
bool cgeneral(const Anchors&a,const std::vector<CPoint>&points){
 for(size_t i=0;i<4;++i)for(size_t j=i+1;j<4;++j)for(size_t k=j+1;k<4;++k){const auto&p=points.at(size_t(a[i])).value;const auto&q=points.at(size_t(a[j])).value;const auto&r=points.at(size_t(a[k])).value;if((q.x-p.x)*(r.y-p.y)==(q.y-p.y)*(r.x-p.x))return false;}
 return true;
}
std::array<CV,2> cbasis(const Anchors&a,const std::vector<CPoint>&points){
 FM matrix;for(int i:a){const auto&v=points.at(size_t(i)).monomial;matrix.emplace_back(v.begin(),v.end());}
 auto r=fs_rref(matrix);require(r.pivots.size()==4,"four independent conic constraints");
 std::vector<size_t> free;for(size_t j=0;j<6;++j)if(std::find(r.pivots.begin(),r.pivots.end(),j)==r.pivots.end())free.push_back(j);
 require(free.size()==2,"two-dimensional conic pencil");std::array<CV,2> basis;
 for(size_t j=0;j<2;++j){basis[j][free[j]]=F(1);for(size_t i=0;i<4;++i)basis[j][r.pivots[i]]=-r.a[i][free[j]];for(int i:a)require(cdot(basis[j],points.at(size_t(i)).monomial)==F(0),"pencil basis incidence");}
 return basis;
}
CV cthrough(const std::array<CV,2>&basis,const CV&constraint){F u=cdot(basis[0],constraint),v=cdot(basis[1],constraint);require(u!=F(0)||v!=F(0),"no extra pencil base condition");CV q;for(size_t i=0;i<6;++i)q[i]=v*basis[0][i]-u*basis[1][i];return cnormalize(q);}
