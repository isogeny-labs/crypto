#pragma once
// New implementation version. The earlier frozen producers stay unchanged.
#define main cm_cache_native_entrypoint
#include "native.cpp"
#undef main
#include <functional>

namespace cmcache {
struct Plan {unsigned radix=1,width=4;int offset=0,shift=0;};
struct Metrics {Counts calls;uint64_t bank_points=0,cache_hits=0,cancellations=0,affine_entries=0;};
struct Split {Big k,c;};
struct Op {int left,right;bool subtract=false;};
static Big negative(Big k){if(!k.zero())BN_set_negative(k.p,!BN_is_negative(k.p));return k;}
static std::vector<int> digits(const Big&k,unsigned width){auto d=wnaf(k,width);if(BN_is_negative(k.p))for(auto&v:d)v=-v;return d;}
static Split decompose(Runtime&r,const Plan&p){
 require(p.radix&&p.width>=2&&p.width<=7&&p.offset>=-64&&p.offset<=64,"cache plan domain");
 Big s=centered(r.mu,r.n),radix{long(p.radix)},rem,c,k,delta(p.offset),twice;
 require(BN_nnmod(rem.p,s.p,radix.p,r.field.ctx)&&BN_lshift1(twice.p,rem.p),"cache remainder");
 if(BN_cmp(twice.p,radix.p)>0)require(BN_sub(rem.p,rem.p,radix.p),"cache centered remainder");
 require(BN_mul(delta.p,delta.p,radix.p,r.field.ctx)&&BN_add(c.p,rem.p,delta.p)&&BN_sub(k.p,s.p,c.p),"cache residual");
 require(BN_div(k.p,rem.p,k.p,radix.p,r.field.ctx)&&rem.zero(),"cache exact quotient");
 require(BN_mul(delta.p,k.p,radix.p,r.field.ctx)&&BN_add(delta.p,delta.p,c.p)&&delta==s,"cache fixed label identity");return {k,c};
}
static std::vector<Op> prefix(unsigned radix){
 if(radix==1)return {};
 if(radix==7)return {{1,1},{2,2},{4,4},{8,1,true}};
 if(radix==11)return {{1,1},{2,2},{4,1},{5,5},{10,1}};
 if(radix==13)return {{1,1},{2,1},{3,3},{6,6},{12,1}};
 if(radix==19)return {{1,1},{2,2},{4,1},{5,5},{10,10},{20,1,true}};
 require(radix==2||radix==4||radix==8||radix==16||radix==32,"cache supported radix");
 std::vector<Op> ops;for(unsigned v=1;v<radix;v*=2)ops.push_back({int(v),int(v)});return ops;
}
using Audit=std::function<void(int,const EC_POINT*)>;
struct Bank {
 Runtime&r;Metrics&m;const Audit&audit;std::vector<std::pair<int,Point>> values;
 Bank(Runtime&runtime,Metrics&metrics,const Audit&check):r(runtime),m(metrics),audit(check){values.reserve(48);}
 int find(int coefficient)const {for(size_t i=0;i<values.size();++i)if(values[i].first==coefficient)return int(i);return -1;}
 const EC_POINT* get(int coefficient)const {int i=find(coefficient);require(i>=0,"bank coefficient missing");return values[size_t(i)].second.get();}
 void retain(int coefficient,Point p){require(coefficient>0&&find(coefficient)<0,"unique bank coefficient");if(audit)audit(coefficient,p.get());values.emplace_back(coefficient,std::move(p));++m.bank_points;}
 void operation(const Op&op){int coefficient=op.subtract?op.left-op.right:op.left+op.right;require(coefficient>0,"positive prefix label");if(find(coefficient)>=0)return;auto out=point(r.group);
  if(op.left==op.right&&!op.subtract){require(EC_POINT_dbl(r.group,out.get(),get(op.left),r.field.ctx),"bank double");++m.calls.doubles;}
  else {auto rhs=point(r.group);require(EC_POINT_copy(rhs.get(),get(op.right)),"bank right input");if(op.subtract){require(EC_POINT_invert(r.group,rhs.get(),r.field.ctx),"bank prefix sign");++m.calls.negations;}require(EC_POINT_add(r.group,out.get(),get(op.left),rhs.get(),r.field.ctx),"bank add");++m.calls.adds;}
  retain(coefficient,std::move(out));
 }
};
static void multiply(Runtime&r,EC_POINT*out,const EC_POINT*P,const Split&s,const Plan&p,bool reuse,Metrics&m,const Audit&audit={}){
 require(p.width>=2&&p.width<=7,"cached width");if(EC_POINT_is_at_infinity(r.group,P)){require(EC_POINT_set_to_infinity(r.group,out),"cache identity");return;}
 Big k=s.k,c=s.c;bool outer_sign=BN_is_negative(k.p);if(outer_sign){k=negative(k);c=negative(c);}auto kd=digits(k,p.width),cd=digits(c,2);
 Bank bank(r,m,audit);auto input=point(r.group);require(EC_POINT_copy(input.get(),P),"cache input copy");bank.retain(1,std::move(input));
 for(const auto&op:prefix(p.radix))bank.operation(op);
 require(bank.find(int(p.radix))>=0,"prefix endpoint present");
 size_t table=kd.empty()?0:(1u<<(p.width-2));if(table>1){bank.operation({int(p.radix),int(p.radix)});for(size_t i=1;i<table;++i)bank.operation({int((2*i-1)*p.radix),int(2*p.radix)});
  std::vector<EC_POINT*> raw;for(size_t i=0;i<table;++i)raw.push_back(const_cast<EC_POINT*>(bank.get(int((2*i+1)*p.radix))));
  require(EC_POINTs_make_affine(r.group,raw.size(),raw.data(),r.field.ctx),"cached odd-table affine normalization");m.affine_entries+=raw.size();
 }
 m.calls.table_points=uint64_t(p.radix>1)+table+uint64_t(!cd.empty()); // Logical prefix/Q/P table entries; bank_points records retained objects separately.
 auto selected=point(r.group);bool started=false;require(EC_POINT_set_to_infinity(r.group,out),"cache zero");
 auto apply=[&](int coefficient){require(coefficient!=0,"nonzero cached term");require(EC_POINT_copy(selected.get(),bank.get(std::abs(coefficient))),"cache lookup");if(coefficient<0){require(EC_POINT_invert(r.group,selected.get(),r.field.ctx),"cache selected sign");++m.calls.negations;}
  if(started){require(EC_POINT_add(r.group,out,out,selected.get(),r.field.ctx),"cache main add");++m.calls.adds;}else{require(EC_POINT_copy(out,selected.get()),"cache first digit");started=true;}
 };
 for(size_t i=std::max(kd.size(),cd.size());i>0;--i){if(started){require(EC_POINT_dbl(r.group,out,out,r.field.ctx),"cache main double");++m.calls.doubles;}int q=i<=kd.size()?kd[i-1]:0,rem=i<=cd.size()?cd[i-1]:0;
  if(reuse&&q&&rem){int merged=q*int(p.radix)+rem;if(!merged){++m.cancellations;continue;}if(bank.find(std::abs(merged))>=0){++m.cache_hits;apply(merged);continue;}}
  if(q)apply(q*int(p.radix));
  if(rem)apply(rem);
 }
 if(outer_sign&&started){require(EC_POINT_invert(r.group,out,r.field.ctx),"cache outer sign");++m.calls.negations;}
}
[[maybe_unused]] static Plan deterministic(const Fixture&f,bool binary=false){
 unsigned bits=unsigned(BN_num_bits(Big(f.curve.at("n").str()).p));return {binary?1u:unsigned(f.marked.at("degree").integer()),bits<=200?4u:(bits<=300?5u:6u),0,0};
}
[[maybe_unused]] static Plan installed(const Json&plans,const std::string&curve,const std::string&method){
 const Json* found=nullptr;for(const auto&row:plans.at("plans").array)if(row.at("curve").str()==curve&&row.at("addition_weight").integer()==1){require(!found,"duplicate installed plan");found=&row.at(method);}require(found,"installed plan missing");return {unsigned(found->at("radix").integer()),unsigned(found->at("window").integer()),found->at("offset").integer(),found->at("offset").integer()};
}
}
