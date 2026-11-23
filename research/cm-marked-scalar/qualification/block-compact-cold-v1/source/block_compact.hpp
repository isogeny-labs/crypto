#pragma once
// Separate implementation: all frozen block/census/cold sources stay unchanged.
#include "block_core.hpp"
#include <bitset>
#include <type_traits>

namespace cmcompact {
// A fallback has at most two terms; every folded block has at most one.
struct Terms {
 std::array<int,2> values{};
 unsigned used=0;
 void push_back(int value){require(used<values.size(),"compact term bound");values[used++]=value;}
 size_t size()const{return used;}
 bool empty()const{return used==0;}
 const int* begin()const{return values.data();}
 const int* end()const{return values.data()+used;}
};
struct Step {unsigned bits=1;Terms terms;};
static_assert(std::is_trivially_copyable<Step>::value,"inline compact step");
struct Program {std::vector<Step> steps;uint64_t doubles=0,adds=0,negations=0,transitions=0;unsigned table=0;bool outer_sign=false,has_residual=false;};
struct Node {bool valid=false;cmblock::Cost cost;size_t previous=0;bool was_started=false;Step step;};
constexpr unsigned maximum_label=32*((1u<<(7-1))-1); // radix * largest odd-table multiplier
struct Labels {
 std::bitset<maximum_label+1> present;
 int maximum=1;
 Labels(){present.set(1);}
 bool contains(int n)const{return n>0&&unsigned(n)<=maximum_label&&present.test(unsigned(n));}
 void insert(int n){require(n>0&&unsigned(n)<=maximum_label,"compact label bound");present.set(unsigned(n));maximum=std::max(maximum,n);}
};
static Labels labels(const cmcache::Plan&p,bool quotient){
 Labels out;
 for(const auto&op:cmcache::prefix(p.radix)){require(out.contains(op.left)&&out.contains(op.right),"compact prefix dependency");out.insert(op.subtract?op.left-op.right:op.left+op.right);}
 size_t table=quotient?(1u<<(p.width-2)):0;
 if(table>1){out.insert(int(2*p.radix));for(size_t i=0;i<table;++i)out.insert(int((2*i+1)*p.radix));}
 return out;
}
static Program prepare(const cmcache::Split&s,const cmcache::Plan&p){
 require(p.width>=2&&p.width<=7&&p.radix>=1&&p.radix<=32,"compact plan domain");
 Big k=s.k,c=s.c;bool sign=BN_is_negative(k.p);
 if(sign){k=cmcache::negative(k);c=cmcache::negative(c);}
 auto kd=cmcache::digits(k,p.width),cd=cmcache::digits(c,2);
 size_t length=std::max(kd.size(),cd.size());require(length<=2048,"compact finite scalar domain");
 auto bank=labels(p,!kd.empty());
 std::vector<std::array<Node,2>> states(length+1);states[0][0].valid=true;
 uint64_t transitions=0;
 auto column=[&](size_t index){int q=index<kd.size()?kd[index]:0,r=index<cd.size()?cd[index]:0;return std::make_pair(q*int(p.radix),r);};
 for(size_t pos=0;pos<length;++pos)for(unsigned started=0;started<2;++started)if(states[pos][started].valid){
  auto offer=[&](const Step&step){
   ++transitions;bool next=started||!step.terms.empty();auto cost=states[pos][started].cost;
   cost.doubles+=started?step.bits:0;cost.adds+=step.terms.size()-(started||step.terms.empty()?0:1);
   for(int d:step.terms)cost.negations+=d<0;
   ++cost.steps;auto&node=states[pos+step.bits][next];
   if(!node.valid||cmblock::key(cost)<cmblock::key(node.cost)){node.valid=true;node.cost=cost;node.previous=pos;node.was_started=started;node.step=step;}
  };
  auto [q,r]=column(length-pos-1);Step fallback;
  if(q&&r&&(q+r==0||bank.contains(std::abs(q+r)))){if(q+r)fallback.terms.push_back(q+r);}
  else{if(q)fallback.terms.push_back(q);if(r)fallback.terms.push_back(r);}
  offer(fallback);
  int64_t value=0;
  for(unsigned span=1;span<=cmblock::maximum_span&&pos+span<=length;++span){
   auto [a,b]=column(length-pos-span);value=2*value+a+b;
   if(span>1&&(value==0||bank.contains(int(std::abs(value))))){Step step;step.bits=span;if(value)step.terms.push_back(int(value));offer(step);}
   if(std::abs(value)>bank.maximum+1)break;
  }
 }
 unsigned end=states[length][0].valid&&(!states[length][1].valid||cmblock::key(states[length][0].cost)<cmblock::key(states[length][1].cost))?0:1;
 require(states[length][end].valid,"compact program exists");auto final=states[length][end].cost;
 Program out;out.doubles=final.doubles;out.adds=final.adds;out.negations=final.negations;out.transitions=transitions;
 out.table=kd.empty()?0:(1u<<(p.width-2));out.outer_sign=sign;out.has_residual=!cd.empty();
 out.steps.reserve(final.steps);
 for(size_t pos=length;pos;){const auto&node=states[pos][end];out.steps.push_back(node.step);pos=node.previous;end=node.was_started;}
 std::reverse(out.steps.begin(),out.steps.end());return out;
}

// Identical bank and point circuit; accepts the inline program directly, with
// no conversion to a vector-backed step or separately installed plan.
static void execute_prepared(Runtime&r,EC_POINT*out,const EC_POINT*P,const cmcache::Split&s,const cmcache::Plan&p,const Program&program,cmblock::Metrics&m,const cmcache::Audit&audit={},const std::function<void()>&after_setup={}){
 if(EC_POINT_is_at_infinity(r.group,P)){require(EC_POINT_set_to_infinity(r.group,out),"compact identity");return;}
 m.planner_transitions=program.transitions;(void)s;
 cmcache::Bank bank(r,m.bank,audit);auto input=point(r.group);require(EC_POINT_copy(input.get(),P),"compact input copy");bank.retain(1,std::move(input));
 for(const auto&op:cmcache::prefix(p.radix))bank.operation(op);
 size_t table=program.table;
 if(table>1){bank.operation({int(p.radix),int(p.radix)});for(size_t i=1;i<table;++i)bank.operation({int((2*i-1)*p.radix),int(2*p.radix)});
  std::vector<EC_POINT*> raw;for(size_t i=0;i<table;++i)raw.push_back(const_cast<EC_POINT*>(bank.get(int((2*i+1)*p.radix))));
  require(EC_POINTs_make_affine(r.group,raw.size(),raw.data(),r.field.ctx),"compact table affine");m.bank.affine_entries=table;
 }
 m.bank.calls.table_points=uint64_t(p.radix>1)+table+uint64_t(program.has_residual);
 bool started=false;auto selected=point(r.group);require(EC_POINT_set_to_infinity(r.group,out),"compact zero");
 uint64_t main_d=0,main_a=0,main_n=0;if(after_setup)after_setup();
 for(const auto&step:program.steps){
  if(step.bits>1){++m.folded_blocks;m.folded_bits+=step.bits;}
  if(started)for(unsigned i=0;i<step.bits;++i){require(EC_POINT_dbl(r.group,out,out,r.field.ctx),"compact main double");++m.bank.calls.doubles;++main_d;}
  else m.leading_bits+=step.bits;
  for(int coefficient:step.terms){
   const EC_POINT*source=bank.get(std::abs(coefficient));
   if(coefficient<0){require(EC_POINT_copy(selected.get(),source)&&EC_POINT_invert(r.group,selected.get(),r.field.ctx),"compact signed selection");source=selected.get();++m.bank.calls.negations;++main_n;}
   if(started){require(EC_POINT_add(r.group,out,out,source,r.field.ctx),"compact main add");++m.bank.calls.adds;++main_a;}
   else{require(EC_POINT_copy(out,source),"compact first term");started=true;}
  }
 }
 require(main_d==program.doubles&&main_a==program.adds&&main_n==program.negations,"compact execution program cost");
 if(program.outer_sign&&started){require(EC_POINT_invert(r.group,out,r.field.ctx),"compact outer sign");++m.bank.calls.negations;}
}
}
