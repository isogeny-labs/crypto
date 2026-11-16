#pragma once
// A separate version: cache_core.hpp and its frozen consumers are unchanged.
#include "cache_core.hpp"
#include <array>
#include <limits>
namespace cmblock {
constexpr unsigned maximum_span=16;
struct Step {unsigned bits=1;std::vector<int> terms;};
struct Program {std::vector<Step> steps;uint64_t doubles=0,adds=0,negations=0,transitions=0;};
struct Metrics {cmcache::Metrics bank;uint64_t folded_blocks=0,folded_bits=0,leading_bits=0,planner_transitions=0;};
static std::set<int> labels(const cmcache::Split&s,const cmcache::Plan&p){
 std::set<int> out{1};for(const auto&op:cmcache::prefix(p.radix)){require(out.count(op.left)&&out.count(op.right),"block prefix dependency");out.insert(op.subtract?op.left-op.right:op.left+op.right);}
 auto kd=cmcache::digits(s.k,p.width);size_t table=kd.empty()?0:(1u<<(p.width-2));if(table>1){out.insert(int(2*p.radix));for(size_t i=0;i<table;++i)out.insert(int((2*i+1)*p.radix));}return out;
}
struct Cost {uint64_t doubles=0,adds=0,negations=0,steps=0;};
static auto key(const Cost&c){return std::make_tuple(c.doubles+c.adds,c.negations,c.doubles,c.steps);}
struct Node {bool valid=false;Cost cost;size_t previous=0;bool was_started=false;Step step;};
static Program prepare(const cmcache::Split&s,const cmcache::Plan&p){
 require(p.width>=2&&p.width<=7&&p.radix>=1&&p.radix<=32,"block plan domain");
 Big k=s.k,c=s.c;if(BN_is_negative(k.p)){k=cmcache::negative(k);c=cmcache::negative(c);}auto kd=cmcache::digits(k,p.width),cd=cmcache::digits(c,2);size_t length=std::max(kd.size(),cd.size());require(length<=2048,"block finite scalar domain");auto bank=labels(s,p);int maximum=*bank.rbegin();
 std::vector<std::array<Node,2>> states(length+1);states[0][0].valid=true;uint64_t transitions=0;
 auto column=[&](size_t index){int q=index<kd.size()?kd[index]:0,r=index<cd.size()?cd[index]:0;return std::make_pair(q*int(p.radix),r);};
 for(size_t pos=0;pos<length;++pos)for(unsigned started=0;started<2;++started)if(states[pos][started].valid){
  auto offer=[&](Step step){++transitions;bool next_started=started||!step.terms.empty();Cost cost=states[pos][started].cost;cost.doubles+=started?step.bits:0;cost.adds+=step.terms.size()-(started||step.terms.empty()?0:1);for(int d:step.terms)cost.negations+=d<0;++cost.steps;auto&node=states[pos+step.bits][next_started];if(!node.valid||key(cost)<key(node.cost)){node.valid=true;node.cost=cost;node.previous=pos;node.was_started=started;node.step=std::move(step);}};
  auto [q,r]=column(length-pos-1);Step fallback;
  if(q&&r&&(q+r==0||bank.count(std::abs(q+r)))){if(q+r)fallback.terms.push_back(q+r);}else{if(q)fallback.terms.push_back(q);if(r)fallback.terms.push_back(r);}offer(std::move(fallback));
  int64_t value=0;for(unsigned span=1;span<=maximum_span&&pos+span<=length;++span){auto [a,b]=column(length-pos-span);value=2*value+a+b;
   if(span>1&&(value==0||bank.count(int(std::abs(value))))){Step step;step.bits=span;if(value)step.terms.push_back(int(value));offer(std::move(step));}
   // Each remaining column has absolute value at most maximum+1. Once
   // outside that interval, no extension can return to a retained label.
   if(std::abs(value)>maximum+1)break;
  }
 }
 unsigned end=states[length][0].valid&&(!states[length][1].valid||key(states[length][0].cost)<key(states[length][1].cost))?0:1;require(states[length][end].valid,"block program exists");auto final=states[length][end].cost;Program out;out.doubles=final.doubles;out.adds=final.adds;out.negations=final.negations;out.transitions=transitions;
 for(size_t pos=length;pos;){const auto&node=states[pos][end];out.steps.push_back(node.step);pos=node.previous;end=node.was_started;}std::reverse(out.steps.begin(),out.steps.end());return out;
}
static void multiply(Runtime&r,EC_POINT*out,const EC_POINT*P,const cmcache::Split&s,const cmcache::Plan&p,Metrics&m,const cmcache::Audit&audit={}){
 if(EC_POINT_is_at_infinity(r.group,P)){require(EC_POINT_set_to_infinity(r.group,out),"block identity");return;}
 auto program=prepare(s,p);m.planner_transitions=program.transitions;
 Big k=s.k,c=s.c;bool outer_sign=BN_is_negative(k.p);if(outer_sign){k=cmcache::negative(k);c=cmcache::negative(c);}auto kd=cmcache::digits(k,p.width),cd=cmcache::digits(c,2);
 cmcache::Bank bank(r,m.bank,audit);auto input=point(r.group);require(EC_POINT_copy(input.get(),P),"block input copy");bank.retain(1,std::move(input));for(const auto&op:cmcache::prefix(p.radix))bank.operation(op);
 size_t table=kd.empty()?0:(1u<<(p.width-2));if(table>1){bank.operation({int(p.radix),int(p.radix)});for(size_t i=1;i<table;++i)bank.operation({int((2*i-1)*p.radix),int(2*p.radix)});std::vector<EC_POINT*> raw;for(size_t i=0;i<table;++i)raw.push_back(const_cast<EC_POINT*>(bank.get(int((2*i+1)*p.radix))));require(EC_POINTs_make_affine(r.group,raw.size(),raw.data(),r.field.ctx),"block table affine");m.bank.affine_entries=table;}
 m.bank.calls.table_points=uint64_t(p.radix>1)+table+uint64_t(!cd.empty());bool started=false;auto selected=point(r.group);require(EC_POINT_set_to_infinity(r.group,out),"block zero");uint64_t main_d=0,main_a=0,main_n=0;
 for(const auto&step:program.steps){if(step.bits>1){++m.folded_blocks;m.folded_bits+=step.bits;}if(started)for(unsigned i=0;i<step.bits;++i){require(EC_POINT_dbl(r.group,out,out,r.field.ctx),"block main double");++m.bank.calls.doubles;++main_d;}else m.leading_bits+=step.bits;
  for(int coefficient:step.terms){const EC_POINT*source=bank.get(std::abs(coefficient));if(coefficient<0){require(EC_POINT_copy(selected.get(),source)&&EC_POINT_invert(r.group,selected.get(),r.field.ctx),"block signed selection");source=selected.get();++m.bank.calls.negations;++main_n;}if(started){require(EC_POINT_add(r.group,out,out,source,r.field.ctx),"block main add");++m.bank.calls.adds;++main_a;}else{require(EC_POINT_copy(out,source),"block first term");started=true;}}
 }
 require(main_d==program.doubles&&main_a==program.adds&&main_n==program.negations,"block execution program cost");if(outer_sign&&started){require(EC_POINT_invert(r.group,out,r.field.ctx),"block outer sign");++m.bank.calls.negations;}
}
}
