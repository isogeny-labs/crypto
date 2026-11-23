#include <gmpxx.h>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

// Candidate coordinates/slopes only; all edges require actual Lean point proofs.
const mpz_class p("730750818665451459112596905638433048232067471723");
const mpz_class a("425706413842211054102700238164133538302169176474");
mpz_class mod(mpz_class n) {n%=p;if(n<0)n+=p;return n;}
mpz_class inverse(const mpz_class& n) {
  mpz_class r;if(!mpz_invert(r.get_mpz_t(),n.get_mpz_t(),p.get_mpz_t()))
    throw std::runtime_error("nonunit");
  return r;
}
struct Node {mpz_class scalar,x,y,slope;bool infinity;int left,right;std::string kind;};
std::vector<Node> nodes;
std::map<mpz_class,int> indexByScalar;
int edge(int left,int right) {
  const Node u=nodes.at(left),v=nodes.at(right);
  const mpz_class scalar=u.scalar+v.scalar;
  if(indexByScalar.count(scalar))return indexByScalar.at(scalar);
  if(u.infinity||v.infinity)throw std::runtime_error("unexpected infinity input");
  Node w{scalar,0,0,0,false,left,right,"secant"};
  if(u.x==v.x&&mod(u.y+v.y)==0){w.infinity=true;w.kind="vertical";}
  else {
    if(u.x==v.x){w.kind="tangent";w.slope=mod((3*u.x*u.x+a)*inverse(mod(2*u.y)));}
    else w.slope=mod((u.y-v.y)*inverse(mod(u.x-v.x)));
    w.x=mod(w.slope*w.slope-u.x-v.x);w.y=mod(w.slope*(u.x-w.x)-u.y);
  }
  int i=int(nodes.size());nodes.push_back(w);indexByScalar.emplace(scalar,i);return i;
}
int chain(const mpz_class& scalar) {
  const std::string bits=scalar.get_str(2);int current=0;
  for(size_t i=1;i<bits.size();i++) {
    current=edge(current,current);
    if(bits[i]=='1')current=edge(current,0);
  }
  return current;
}
int main() {
  nodes.push_back({1,1,mpz_class("310536468939899693718962354338996655381367569020"),0,false,-1,-1,"base"});
  indexByScalar.emplace(1,0);
  const int order=chain(p);
  const mpz_class answer("210393287966660756596132643172172640405085536179");
  const int target=chain(answer);
  std::cout<<"{\"order_node\":"<<order<<",\"target_node\":"<<target<<",\"nodes\":[\n";
  for(size_t i=0;i<nodes.size();i++) {
    if(i)std::cout<<",\n";
    const auto& n=nodes[i];
    std::cout<<"{\"scalar\":\""<<n.scalar<<"\",\"x\":\""<<n.x<<"\",\"y\":\""<<n.y
      <<"\",\"slope\":\""<<n.slope<<"\",\"infinity\":"<<(n.infinity?"true":"false")
      <<",\"left\":"<<n.left<<",\"right\":"<<n.right<<",\"kind\":\""<<n.kind<<"\"}";
  }
  std::cout<<"\n]}\n";
}
