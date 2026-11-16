// Synthetic development fixture only. Never reads the actual curve census.
#include "../../native/common.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <set>

namespace fs=std::filesystem;
static GEN canonical(GEN b,GEN p){for(long x=0;x<1000;++x){GEN rhs=modii(addii(subii(powiu(stoi(x),3),stoi(3*x)),b),p);if(kronecker(rhs,p)>=0){GEN y=Fp_sqrt(rhs,p);if(cmpii(subii(p,y),y)<0)y=subii(p,y);return mkvec2(stoi(x),y);}}throw std::runtime_error("fixture point missing");}
int main(int argc,char** argv){try{
 require(argc==2,"make_fixture FRESH_DIRECTORY");fs::path out=argv[1];require(!fs::exists(out),"fixture directory exists");fs::create_directories(out/"existing-features");pari_init(64000000,1000000);
 std::ofstream cohort(out/"cohort.tsv"),pub(out/"published-points.tsv"),given(out/"supplied-points.tsv");
 cohort<<"# synthetic labels and literal prime-field fixtures only\n";pub<<"# synthetic standardized-point stand-in; G=[b]Q planted\n";given<<"# synthetic supplied-point stand-in\n";
 std::set<int> selected{0,2,4,8,11,12,16,18,19,20},supplied{1,5,9,17};std::vector<int> counts{5,4,8,2,1,1};int row=0;
 for(size_t group=0;group<counts.size();++group){GEN p=addii(int2n(long(group)+6),gen_1);while(!isprime(p)||umodiu(p,4)!=1)p=addis(p,4);GEN b=nullptr,n=nullptr,e=nullptr;
  for(long trial=3;trial<1000;++trial){GEN t=stoi(trial);if(gequal0(modii(subis(sqri(t),4),p)))continue;GEN curve=ellinit(mkvec2(stoi(-3),t),p,DEFAULTPREC),order=ellcard(curve,nullptr);
   if(isprime(order)&&cmpii(order,stoi(17))>0&&bit_length(order)<=bit_length(p)){b=t;n=order;e=curve;break;}}
  require(b,"no tiny prime-order fixture");GEN q=canonical(b,p),g=liftall(ellmul(e,q,b));require(lg(g)==3,"planted fixture scalar zero");
  for(int j=0;j<counts[group];++j,++row){std::string label="fixture-"+std::to_string(row),role=selected.count(row)?"published":"skipped";GEN seed=stoi(row+1);
   cohort<<label<<'\t'<<bit_length(p)<<'\t'<<role<<"\t1\t"<<row+1<<'\t'<<decimal(p)<<'\t'<<decimal(b)<<'\t'<<decimal(n)<<'\t'<<decimal(seed)<<'\n';
   pub<<label<<"\tfixture\t"<<decimal(p)<<'\t'<<decimal(b)<<'\t'<<decimal(n)<<'\t'<<decimal(seed)<<'\t';if(selected.count(row))pub<<decimal(gel(g,1))<<'\t'<<decimal(gel(g,2));else pub<<"NA\tNA";pub<<'\n';
   if(supplied.count(row))given<<label<<'\t'<<bit_length(p)<<'\t'<<role<<"\t1\t"<<row+1<<'\t'<<decimal(p)<<'\t'<<decimal(b)<<'\t'<<decimal(gel(q,1))<<'\t'<<decimal(gel(q,2))<<'\t'<<decimal(n)<<'\t'<<decimal(seed)<<"\tfixture-only\n";
   std::ofstream features(out/"existing-features"/("curve-"+std::to_string(row)+".features.tsv"));for(int k=0;k<1127;++k)features<<"fixture_feature_"<<k<<'\t'<<long(k%3)-1+row<<'\n';
  }
 }
 require(row==21,"fixture row count");std::ofstream marker(out/"SYNTHETIC_FIXTURE.txt");marker<<"Six tiny prime-field examples repeated to exercise the21-row parser, unknowns, traits, cutoffs, pairwise tables and planted scalar matches. Not the production census.\n";
 std::cout<<"Synthetic-only21-row integration fixture written\n";return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
