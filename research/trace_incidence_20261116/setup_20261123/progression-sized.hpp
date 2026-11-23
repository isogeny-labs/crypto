#pragma once
#include "progression-barrett.hpp"
#include <cmath>
namespace labelled_progression {
inline Word floor_root(Word n){Word x=static_cast<Word>(std::sqrt(static_cast<long double>(n)));while(__uint128_t(x)*x>n)--x;while(__uint128_t(x+1)*(x+1)<=n)++x;return x;}
inline Word ceil_root(Word n){const Word x=floor_root(n);return x+(__uint128_t(x)*x<n);}
inline std::array<std::size_t,3> shape_sizes(Word B,Word r,const std::string&shape){
 if(shape=="balanced")return {std::size_t(B),std::size_t(B),std::size_t(8*B)};
 const Word root=ceil_root(r);Word C=0;
 if(shape=="quarter")C=(root+3)/4;else if(shape=="half")C=(root+1)/2;else if(shape=="one")C=root;else if(shape=="two")C=2*root;else throw std::invalid_argument("unknown pool shape");
 const Word m=ceil_root((r+C-1)/C);return {std::size_t(m),std::size_t(m),std::size_t(C)};
}
template<class Math> inline LabelledPools generate_sizes(LabelledPools out,const std::array<std::size_t,3>&sizes,Word seed,acc::FastContext&ctx,acc::GenerationCounts&g,Extra&extra){
 const std::size_t total=sizes[0]+sizes[1]+sizes[2];require(sizes[0]>0&&sizes[1]>0&&sizes[2]>0&&sizes[0]<=2048&&sizes[1]<=2048&&sizes[2]<=131072&&total<=262144&&2*Word(total)<out.subgroup_order,"sized progression capacity");
 Math field(out.target);std::mt19937_64 random(seed);std::uniform_int_distribution<Word> draw(0,out.subgroup_order-1);
 std::vector<std::array<Word,2>> labels;for(unsigned i=0;i<6;++i){labels.push_back({draw(random),draw(random)});++extra.seed_pairs;}
 auto initial=ctx.combine(labels,false);flat_membership::Set used(total);for(auto&p:out.pools)p.clear();g={};
 for(unsigned pool=0;pool<3;++pool){
  auto step=initial[2*pool+1];auto delta=labels[2*pool+1];
  while(step.infinity){++extra.step_infinity;require(++extra.seed_pairs<1000000,"progression seed bound");delta={draw(random),draw(random)};step=ctx.combine({delta},false).at(0);}
  Jacobian state=field.from(initial[2*pool]);Word alpha=labels[2*pool][0],beta=labels[2*pool][1];const auto size=sizes[pool];
  while(out.pools[pool].size()<size){
   const auto remaining=size-out.pools[pool].size();std::vector<Jacobian> pending;std::vector<std::array<Word,2>> coefficients;pending.reserve(remaining);coefficients.reserve(remaining);
   for(std::size_t i=0;i<remaining;++i){require(++g.draws<1000000,"progression draw bound");pending.push_back(state);coefficients.push_back({alpha,beta});state=field.plus(state,step);++extra.projective_additions;alpha=add_label(alpha,delta[0],out.subgroup_order);beta=add_label(beta,delta[1],out.subgroup_order);}
   const auto points=field.affine(pending);++extra.affine_batches;
   for(std::size_t i=0;i<points.size();++i){const auto point=points[i];if(point.infinity){++g.rejected_infinity;continue;}if(!used.insert(point.x)){++g.rejected_coordinate;continue;}out.pools[pool].push_back({point,coefficients[i][0],coefficients[i][1]});}
  }
 }
 return out;
}
}
