#define main preceding_fixture_main
#include "../trace-fixture-20261115/fixture.cpp"
#undef main
#include <sstream>
std::string codes(const std::vector<int>&v){if(v.empty())return "-";std::ostringstream o;for(unsigned j=0;j<v.size();j++){if(j)o<<',';o<<v[j];}return o.str();}
int main(){try{AA=1;BB=8;delta=8;auto source=points(AA,BB);if(source.size()!=47)throw std::runtime_error("source domain");std::cout<<"TQ1 7 3 8 1 8 47\n";unsigned id=0;for(P q:source){std::vector<int> roots,small;if(!q.o){F s=q.x.a,t=q.y.a;for(F l=0;l<49;l++){F l2=mul(l,l);F f=sub(sub(sub(sub(mul(l2,l2),scale(mul(s,l2),6)),scale(mul(s,s),3)),scale(mul(t,l),8)),scale(add(AA,delta),4));if(f)continue;F a=mul(sub(l2,s),inv(2)),c=sub(mul(l,sub(a,s)),t);P r{{a,1},{c,l},false};if(!on(r)||!(sum(r,frob(r))==q))throw std::runtime_error("quartic lift");roots.push_back(l);if(a/7==0)small.push_back(l);}}std::cout<<id++<<' '<<(q.o?-1:q.x.a)<<' '<<(q.o?-1:q.y.a)<<' '<<roots.size()<<' '<<codes(roots)<<' '<<small.size()<<' '<<codes(small)<<'\n';}}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}return 0;}
