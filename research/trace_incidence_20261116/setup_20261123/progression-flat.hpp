#pragma once
#include "progression.hpp"
#include "flat.hpp"
namespace labelled_progression {
inline LabelledPools generate_flat(LabelledPools out,std::size_t B,std::size_t scale,Word seed,acc::FastContext&ctx,acc::GenerationCounts&g,Extra&extra){
 require(B>0&&B<=128&&scale>=1&&scale<=8&&2*Word((2+scale)*B)<out.subgroup_order,"progression capacity");
 Arithmetic field(out.target);std::mt19937_64 random(seed);std::uniform_int_distribution<Word> draw(0,out.subgroup_order-1);
 std::vector<std::array<Word,2>> labels;for(unsigned i=0;i<6;++i){labels.push_back({draw(random),draw(random)});++extra.seed_pairs;}
 auto initial=ctx.combine(labels,false);flat_membership::Set used((2+scale)*B);for(auto&p:out.pools)p.clear();g={};
 for(unsigned pool=0;pool<3;++pool){
  auto step=initial[2*pool+1];auto delta=labels[2*pool+1];
  while(step.infinity){++extra.step_infinity;require(++extra.seed_pairs<1000000,"progression seed bound");delta={draw(random),draw(random)};step=ctx.combine({delta},false).at(0);}
  Jacobian state=field.from(initial[2*pool]);Word alpha=labels[2*pool][0],beta=labels[2*pool][1];const auto size=B*(pool==2?scale:1);
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
