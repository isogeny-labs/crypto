// Exact finite census. No point counting, DLP, cost model or historical attribution.
#include <gmpxx.h>
#include <algorithm>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs=std::filesystem;
static void need(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
static std::vector<std::string> split(const std::string& s){std::vector<std::string> v;std::stringstream f(s);std::string x;while(std::getline(f,x,'\t'))v.push_back(x);return v;}
static std::vector<int> primes(){std::vector<int> v;for(int x=2;x<=2000;++x){bool ok=true;for(int d=2;d<=x/d;++d)if(x%d==0){ok=false;break;}if(ok)v.push_back(x);}need(v.size()==303,"prime catalog");return v;}
struct Row{std::string label,run;int bits;bool pub;mpz_class p,n,t,d;};
struct Group{uint32_t mask=0;int count=0;};
static bool separates(uint32_t mask,const std::vector<std::vector<int>>& counts,size_t j,const std::vector<Row>& rows){
 std::map<int,std::pair<int,int>> bounds;
 for(size_t i=0;i<rows.size();++i){auto[it,inserted]=bounds.emplace(rows[i].bits,std::make_pair(-1,10000));(void)inserted;if(mask&(uint32_t(1)<<i))it->second.first=std::max(it->second.first,counts[i][j]);else it->second.second=std::min(it->second.second,counts[i][j]);}
 for(const auto& b:bounds)if(b.second.first>=b.second.second)return false;
 return true;
}
static std::vector<uint32_t> assignments(const std::vector<Row>& rows){
 std::map<std::string,Group> groups;for(size_t i=0;i<rows.size();++i){auto& g=groups[rows[i].run];g.mask|=uint32_t(1)<<i;g.count+=rows[i].pub;}
 std::vector<uint32_t> out;for(uint32_t m=0;m<(uint32_t(1)<<rows.size());++m){bool ok=true;for(const auto& g:groups)if(__builtin_popcount(m&g.second.mask)!=g.second.count){ok=false;break;}if(ok)out.push_back(m);}return out;
}
static void selftest(){
 const auto ls=primes();for(int l:ls){if(l==2)continue;std::vector<bool> square(l);for(int x=0;x<l;++x)square[x*x%l]=true;int nonzero=0;for(int x=1;x<l;++x)nonzero+=square[x];need(nonzero==(l-1)/2,"residue enumeration");}
 std::vector<Row> r(4);for(auto& x:r){x.bits=7;x.run="toy";}r[0].pub=r[1].pub=true;std::vector<std::vector<int>> c{{0},{1},{2},{3}};need(separates(3,c,0,r),"planted separator");need(!separates(10,c,0,r),"negative separator");need(assignments(r).size()==6,"toy label inventory");
 std::cout<<"PASS: residue catalogs, planted separator, negative separator, six toy assignments\n";
}
static void write(const fs::path& p,const std::string& s){std::ofstream f(p);need(bool(f),"output "+p.string());f<<s;need(bool(f),"output write");}
int main(int argc,char** argv){try{
 if(argc==2&&std::string(argv[1])=="--self-test"){selftest();return 0;}
 need(argc==3,"producer COHORT FRESH_OUTPUT");need(!fs::exists(argv[2]),"fresh output required");
 std::ifstream in(argv[1]);need(bool(in),"cohort");std::string line;std::vector<Row> rows;uint32_t actual=0;
 while(std::getline(in,line)){if(line.empty()||line[0]=='#')continue;auto v=split(line);need(v.size()==9,"cohort columns");Row r;r.label=v[0];r.bits=std::stoi(v[1]);need(v[2]=="published"||v[2]=="skipped","role");r.pub=v[2]=="published";r.run=v[3];need(r.p.set_str(v[5],0)==0&&r.n.set_str(v[7],0)==0,"integer input");r.t=r.p+1-r.n;r.d=r.t*r.t-4*r.p;need(r.d<0&&r.p>2000&&mpz_odd_p(r.n.get_mpz_t()),"public cohort invariant");need(mpz_sizeinbase(r.p.get_mpz_t(),2)==static_cast<size_t>(r.bits),"field bits");if(r.pub)actual|=uint32_t(1)<<rows.size();rows.push_back(r);}
 need(rows.size()==21&&__builtin_popcount(actual)==10,"all21 labels required");const auto ls=primes();const auto masks=assignments(rows);need(masks.size()==4480,"all4480 assignments required");
 std::vector<std::vector<int>> strict(21,std::vector<int>(303)),inclusive=strict,ramified=strict,chi=strict;
 for(size_t j=0;j<ls.size();++j){int l=ls[j];std::vector<bool> square(l);for(int x=0;x<l;++x)square[x*x%l]=true;
  for(size_t i=0;i<rows.size();++i){if(j){strict[i][j]=strict[i][j-1];ramified[i][j]=ramified[i][j-1];inclusive[i][j]=inclusive[i][j-1];}if(l==2){chi[i][j]=2;continue;}unsigned long rem=mpz_fdiv_ui(rows[i].d.get_mpz_t(),static_cast<unsigned long>(l));int ch=rem==0?0:square[rem]?1:-1;chi[i][j]=ch;strict[i][j]+=ch==1;ramified[i][j]+=ch==0;inclusive[i][j]+=ch>=0;}
 }
 const size_t fixed=static_cast<size_t>(std::find(ls.begin(),ls.end(),691)-ls.begin());need(fixed<ls.size(),"fixed691");
 std::ostringstream raw,table,cut,cal,inv;raw<<"label\tell\tcharacter\tstrict_count\tramified_count\tinclusive_count\n";table<<"label\tbits\trole\tstrict691\tramified691\tinclusive691\n";inv<<"label\ttrace\tdiscriminant\n";
 for(size_t i=0;i<21;++i){inv<<rows[i].label<<'\t'<<rows[i].t<<'\t'<<rows[i].d<<'\n';table<<rows[i].label<<'\t'<<rows[i].bits<<'\t'<<(rows[i].pub?"published":"skipped")<<'\t'<<strict[i][fixed]<<'\t'<<ramified[i][fixed]<<'\t'<<inclusive[i][fixed]<<'\n';for(size_t j=0;j<303;++j)raw<<rows[i].label<<'\t'<<ls[j]<<'\t'<<(j?std::to_string(chi[i][j]):"NA_ell2")<<'\t'<<strict[i][j]<<'\t'<<ramified[i][j]<<'\t'<<inclusive[i][j]<<'\n';}
 std::vector<int> separating;cut<<"cutoff\tstrict_separates\tinclusive_separates\n";for(size_t j=0;j<303;++j){bool s=separates(actual,strict,j,rows);if(s)separating.push_back(ls[j]);cut<<ls[j]<<'\t'<<s<<'\t'<<separates(actual,inclusive,j,rows)<<'\n';}
 int good=0,good_fixed=0,good_inclusive=0;cal<<"published_mask\tstrict_any_cutoff\tstrict_fixed691\tinclusive_any_cutoff\n";
 for(uint32_t m:masks){bool any=false,inc=false;for(size_t j=0;j<303;++j){any=any||separates(m,strict,j,rows);inc=inc||separates(m,inclusive,j,rows);}bool one=separates(m,strict,fixed,rows);good+=any;good_fixed+=one;good_inclusive+=inc;cal<<m<<'\t'<<any<<'\t'<<one<<'\t'<<inc<<'\n';}
 std::ostringstream summary;summary<<"{\"kind\":\"strict-elkies-finite-replay/v1\",\"curve_choices\":21,\"published\":10,\"skipped\":11,\"cutoffs\":303,\"odd_prime_character_cells\":6342,\"exceptional_two_rows\":21,\"label_assignments\":4480,\"strict691_separates\":"<<(separates(actual,strict,fixed,rows)?"true":"false")<<",\"strict_search_success_assignments\":"<<good<<",\"strict_fixed691_success_assignments\":"<<good_fixed<<",\"inclusive_search_success_assignments\":"<<good_inclusive<<",\"strict_separating_cutoffs\":[";for(size_t i=0;i<separating.size();++i){if(i)summary<<',';summary<<separating[i];}summary<<"],\"historical_use_established\":false}\n";
 fs::create_directories(argv[2]);fs::path out=argv[2];write(out/"characters.tsv",raw.str());write(out/"at691.tsv",table.str());write(out/"cutoffs.tsv",cut.str());write(out/"calibration.tsv",cal.str());write(out/"invariants.tsv",inv.str());write(out/"summary.json",summary.str());std::cout<<"Wrote all21 rows,303 cutoffs and4480 label assignments\n";return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
