#include <nlohmann/json.hpp>
#include <array>
#include <vector>
#include <iostream>
#include <cstdint>
using W=std::uint64_t;using V=std::array<W,6>;using J=nlohmann::json;
std::vector<V> vectors(W r,W n){std::vector<V> out;for(W i=0;i<n;++i)for(W j=0;j<n;++j)for(W k=0;k<n;++k)for(W s:{W(1),r-1})for(W t:{W(1),r-1})out.push_back({1,i,s,s*j%r,t,t*k%r});return out;}
int main(){try{W rank_checks=0;for(const auto&rn:std::vector<std::pair<W,W>>{{5,2},{7,3},{17,4},{101,5}}){const auto r=rn.first;const auto v=vectors(r,rn.second);for(std::size_t i=0;i<v.size();++i)for(std::size_t j=0;j<i;++j){bool rank=false;for(unsigned a=0;a<6;++a)for(unsigned b=0;b<a;++b)rank=rank||(v[i][a]*v[j][b]%r!=v[i][b]*v[j][a]%r);if(!rank){throw std::runtime_error("dependent distinct relation vectors");}++rank_checks;}}
 const auto v=vectors(5,2);W sum=0,factorial=0,nonzero=0;for(W code=0;code<15625;++code){V x;W rest=code;for(auto&a:x){a=rest%5;rest/=5;}W hits=0;for(const auto&c:v){W value=0;for(unsigned i=0;i<6;++i)value+=x[i]*c[i];hits+=value%5==0;}sum+=hits;factorial+=hits*(hits-1);nonzero+=hits>0;}
 if(sum!=100000||factorial!=620000||nonzero*(sum+factorial)<sum*sum){throw std::runtime_error("exact second-moment mismatch");}std::cout<<J({{"status","passed"},{"rank_pair_checks",rank_checks},{"enumerated_unfiltered_models",15625},{"sum_relation_counts",sum},{"sum_second_factorial_moments",factorial},{"models_with_relation",nonzero},{"scope","finite linear-coefficient and second-moment transcription checks; no EC runtime or signed general proof"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
