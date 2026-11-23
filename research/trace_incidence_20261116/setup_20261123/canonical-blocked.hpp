#pragma once
#include "progression.hpp"
#include "flat.hpp"
#include <flint/ulong_extras.h>
#include <unordered_map>
namespace canonical_pairs {
using namespace paper_ec;namespace acc=paper_ec::accelerated;
inline void require(bool x,const char*m){if(!x)throw std::invalid_argument(m);}
inline Word mix(Word x){x^=x>>30;x*=0xbf58476d1ce4e5b9ULL;x^=x>>27;x*=0x94d049bb133111ebULL;return x^(x>>31);}
template<bool Small>struct Field{
 Word p,reciprocal;nmod_t mod;
 explicit Field(Word prime):p(prime),reciprocal(0){nmod_init(&mod,p);if constexpr(Small){require(p<(Word(1)<<32),"Barrett32 field range");reciprocal=Word((__uint128_t(1)<<64)/p);}}
 Word add(Word a,Word b)const{const auto s=a+b;return s>=p?s-p:s;}
 Word sub(Word a,Word b)const{return a>=b?a-b:p-(b-a);}
 Word neg(Word a)const{return a?p-a:0;}
 Word mul(Word a,Word b)const{
  if constexpr(Small){const Word product=a*b;const Word q=Word((__uint128_t(product)*reciprocal)>>64);const Word r=product-q*p;return r>=p?r-p:r;}
  else{return nmod_mul(a,b,mod);}
 }
};
// Standard cache-local blocked Bloom filter: both bits are in one word.
// A positive filter result still requires exact flat-map equality.
class BlockedBloom {
 std::vector<Word> words_;std::size_t mask_;
 static Word bits(Word h){return (Word(1)<<((h>>32)&63))|(Word(1)<<((h>>38)&63));}
public:
 explicit BlockedBloom(std::size_t n){std::size_t count=1;while(count<n){if(count>std::numeric_limits<std::size_t>::max()/2)throw std::length_error("blocked bloom capacity");count*=2;}words_.resize(count);mask_=count-1;}
 void add(Word x){const Word h=mix(x);words_[std::size_t(h)&mask_]|=bits(h);}
 bool possible(Word x)const{const Word h=mix(x),m=bits(h);return (words_[std::size_t(h)&mask_]&m)==m;}
};
// Standard Barrett32 contract: inputs below p<2^32 make a*b fit in u64;
// floor(z*floor(2^64/p)/2^64) underestimates floor(z/p) by at most one.
// General word fields use FLINT's existing nmod arithmetic.
class Selector{
 Curve curve_;Word r_;Point g_,q_;acc::FastContext context_;acc::FastCounts work_{};
 template<bool Small>IncidenceAnswer run(const LabelledPools&p){
  Field<Small> f(curve_.p);flat_membership::Map index(p.pools[2].size());
  BlockedBloom bloom(p.pools[2].size());
  for(std::size_t k=0;k<p.pools[2].size();++k){const Word x=p.pools[2][k].point.x;index.put(x,k);bloom.add(x);}
  const auto n=p.pools[1].size();std::vector<Word> prefix(n),inverse(n);IncidenceAnswer answer;nmod_t scalars;nmod_init(&scalars,r_);
  auto signed_label=[&](Word a,int sign){return sign<0&&a?r_-a:a;};
  for(std::size_t i=0;i<p.pools[0].size();++i){const auto&left=p.pools[0][i];Word product=1;
   for(std::size_t j=0;j<n;++j){prefix[j]=product;product=f.mul(product,f.sub(p.pools[1][j].point.x,left.point.x));}
   Word back=n_invmod(product,curve_.p);++work_.batch_inversions;
   for(std::size_t j=n;j-->0;){inverse[j]=f.mul(back,prefix[j]);back=f.mul(back,f.sub(p.pools[1][j].point.x,left.point.x));}
   for(std::size_t j=0;j<n;++j){const auto&second=p.pools[1][j];const Word xsum=f.add(left.point.x,second.point.x);
    for(const int sign:{1,-1}){++work_.pair_coordinates;const Word y2=sign==1?second.point.y:f.neg(second.point.y);const Word slope=f.mul(f.sub(y2,left.point.y),inverse[j]);const Word x=f.sub(f.mul(slope,slope),xsum);
     if(!bloom.possible(x)) { continue; }
     ++work_.exact_lookups;std::size_t found=0;if(!index.get(x,found))continue;
     answer.coordinate_relation=true;const auto k=found;const auto&third=p.pools[2][k];const Word y=f.sub(f.mul(slope,f.sub(left.point.x,x)),left.point.y);const int s3=y==third.point.y?-1:1;require(y==third.point.y||y==f.neg(third.point.y),"ordinate mismatch");
     const Word A=Word((__uint128_t(left.alpha)+signed_label(second.alpha,sign)+signed_label(third.alpha,s3))%r_);const Word B=Word((__uint128_t(left.beta)+signed_label(second.beta,sign)+signed_label(third.beta,s3))%r_);
     if(B==0){require(A==0,"inconsistent uninformative labels");++work_.uninformative;continue;}
     const Word scalar=nmod_mul(A?r_-A:0,n_invmod(B,r_),scalars);require(context_.combine({{scalar,0}},false).at(0)==q_,"public scalar mismatch");answer.informative_relation=true;answer.positions={i,j,k};answer.signs={1,sign,s3};answer.recovered_scalar=scalar;return answer;
    }
   }
  }
  return answer;
 }
public:
 explicit Selector(const LabelledPools&p,bool prepare=false):curve_(p.target),r_(p.subgroup_order),g_(p.generator),q_(p.public_target),context_(p,prepare){}
 acc::FastContext& context(){return context_;}
 acc::FastCounts counts()const{auto c=context_.counts();c.batch_inversions+=work_.batch_inversions;c.pair_coordinates+=work_.pair_coordinates;c.exact_lookups+=work_.exact_lookups;c.uninformative+=work_.uninformative;return c;}
 IncidenceAnswer search(const LabelledPools&p,bool barrett){
  require(p.target.p==curve_.p&&p.target.a==curve_.a&&p.target.b==curve_.b&&p.subgroup_order==r_&&p.generator==g_&&p.public_target==q_,"selector belongs to another target");
  flat_membership::Set seen(p.pools[0].size()+p.pools[1].size()+p.pools[2].size());
  const Field<true> validation_field(curve_.p<(Word(1)<<32)?curve_.p:5);
  auto valid=[&](const Point&point){if(curve_.p>=(Word(1)<<32))return on_curve(curve_,point);const auto&f=validation_field;return f.mul(point.y,point.y)==f.add(f.add(f.mul(f.mul(point.x,point.x),point.x),f.mul(curve_.a,point.x)),curve_.b);};
  for(const auto&pool:p.pools){require(!pool.empty(),"empty pool");for(const auto&l:pool)require(!l.point.infinity&&l.point.x<curve_.p&&l.point.y<curve_.p&&valid(l.point)&&l.alpha<r_&&l.beta<r_&&seen.insert(l.point.x),"invalid labelled pool");}
  if(barrett&&curve_.p<(Word(1)<<32)) { return run<true>(p); }
  return run<false>(p);
 }
};
} // namespace canonical_pairs
