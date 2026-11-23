#pragma once
#include "progression-flat.hpp"
namespace labelled_progression {
struct Arithmetic32{
 Word p,a,reciprocal;
 explicit Arithmetic32(const Curve&c):p(c.p),a(c.a),reciprocal(0){require(p>3&&p<(Word(1)<<32),"Barrett32 progression range");reciprocal=Word((__uint128_t(1)<<64)/p);}
 Word add(Word x,Word y)const{const Word s=x+y;return s>=p?s-p:s;}
 Word sub(Word x,Word y)const{return x>=y?x-y:p-(y-x);}
 Word mul(Word x,Word y)const{const Word z=x*y;const Word q=Word((__uint128_t(z)*reciprocal)>>64);const Word r=z-q*p;return r>=p?r-p:r;}
 Word sq(Word x)const{return mul(x,x);}
 Word power(Word x,Word n)const{Word out=1;while(n){if(n&1)out=mul(out,x);x=sq(x);n>>=1;}return out;}
 Jacobian from(Point t)const{return t.infinity?Jacobian{}:Jacobian{t.x,t.y,1};}
 Jacobian twice(Jacobian t)const{
  if(t.z==0||t.y==0)return {};
  const Word yy=sq(t.y),yyyy=sq(yy),zz=sq(t.z),xx=sq(t.x);
  Word s=mul(t.x,yy);s=add(s,s);s=add(s,s);
  const Word m=add(add(add(xx,xx),xx),mul(a,sq(zz)));
  const Word x=sub(sq(m),add(s,s));Word eight=add(yyyy,yyyy);eight=add(eight,eight);eight=add(eight,eight);
  return {x,sub(mul(m,sub(s,x)),eight),mul(add(t.y,t.y),t.z)};
 }
 Jacobian plus(Jacobian t,Point step)const{
  if(step.infinity) { return t; }
  if(t.z==0) { return from(step); }
  const Word zz=sq(t.z),u=mul(step.x,zz),s=mul(step.y,mul(t.z,zz));
  const Word h=sub(u,t.x),r=sub(s,t.y);if(h==0)return r==0?twice(t):Jacobian{};
  const Word hh=sq(h),hhh=mul(h,hh),v=mul(t.x,hh),x=sub(sub(sq(r),hhh),add(v,v));
  return {x,sub(mul(r,sub(v,x)),mul(t.y,hhh)),mul(t.z,h)};
 }
 std::vector<Point> affine(const std::vector<Jacobian>& points)const{
  std::vector<Word> prefix(points.size());Word product=1;
  for(std::size_t i=0;i<points.size();++i){prefix[i]=product;if(points[i].z)product=mul(product,points[i].z);}
  Word inverse=power(product,p-2);std::vector<Point> out(points.size());
  for(std::size_t i=points.size();i-->0;){if(points[i].z==0)continue;const Word zinv=mul(inverse,prefix[i]);inverse=mul(inverse,points[i].z);const Word z2=sq(zinv);out[i]={mul(points[i].x,z2),mul(points[i].y,mul(z2,zinv)),false};}
  return out;
 }
};
inline LabelledPools generate_barrett(LabelledPools out,std::size_t B,std::size_t scale,Word seed,acc::FastContext&ctx,acc::GenerationCounts&g,Extra&extra){
 require(B>0&&B<=128&&scale>=1&&scale<=8&&2*Word((2+scale)*B)<out.subgroup_order,"progression capacity");
 Arithmetic32 field(out.target);std::mt19937_64 random(seed);std::uniform_int_distribution<Word> draw(0,out.subgroup_order-1);
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
