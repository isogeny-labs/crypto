// Separate PARI character computation and recursive exact label enumeration.
#include <pari/pari.h>
#include <algorithm>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs=std::filesystem;
static void check(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
static std::string read(const fs::path& p){std::ifstream f(p);check(bool(f),"missing "+p.string());return {std::istreambuf_iterator<char>(f),{}};}
static std::vector<std::string> fields(const std::string& s){std::vector<std::string> a;size_t p=0;for(;;){size_t q=s.find('\t',p);a.push_back(s.substr(p,q==std::string::npos?q:q-p));if(q==std::string::npos)return a;p=q+1;}}
static std::string dec(GEN x){char* s=GENtostr(x);std::string out(s);pari_free(s);return out;}
struct Entry{std::string id,run;int bits;bool pub;GEN p,n,t,d;};
static bool fits(uint32_t flags,const std::vector<std::vector<int>>& data,size_t column,const std::vector<Entry>& rows){
 for(size_t a=0;a<rows.size();++a)if(flags&(uint32_t(1)<<a))for(size_t b=0;b<rows.size();++b)if(!(flags&(uint32_t(1)<<b))&&rows[a].bits==rows[b].bits&&data[a][column]>=data[b][column])return false;
 return true;
}
static std::vector<uint32_t> labels(const std::vector<Entry>& rows){
 std::map<std::string,std::vector<size_t>> groups;for(size_t i=0;i<rows.size();++i)groups[rows[i].run].push_back(i);
 std::vector<std::vector<uint32_t>> choices;
 for(const auto& g:groups){int k=0;for(size_t i:g.second)k+=rows[i].pub;std::vector<uint32_t> group;
  std::function<void(size_t,int,uint32_t)> choose=[&](size_t i,int left,uint32_t mask){if(left==0){group.push_back(mask);return;}if(g.second.size()-i<static_cast<size_t>(left))return;choose(i+1,left-1,mask|(uint32_t(1)<<g.second[i]));choose(i+1,left,mask);};choose(0,k,0);choices.push_back(group);}
 std::vector<uint32_t> all;std::function<void(size_t,uint32_t)> cross=[&](size_t i,uint32_t mask){if(i==choices.size()){all.push_back(mask);return;}for(uint32_t m:choices[i])cross(i+1,mask|m);};cross(0,0);std::sort(all.begin(),all.end());return all;
}
int main(int argc,char** argv){try{
 pari_init(64000000,1000000);
 if(argc==2&&std::string(argv[1])=="--self-test"){check(kronecker(stoi(9),stoi(11))==1,"positive residue");check(kronecker(stoi(3),stoi(5))==-1,"negative residue");check(kronecker(gen_0,stoi(7))==0,"ramification");std::cout<<"PASS: PARI positive/negative/zero character controls\n";return 0;}
 check(argc==4,"verify COHORT RESULTS RECEIPT");std::istringstream source(read(argv[1]));std::string line;std::vector<Entry> rows;uint32_t actual=0;
 while(std::getline(source,line)){if(line.empty()||line[0]=='#')continue;auto c=fields(line);check(c.size()==9,"cohort width");Entry e;e.id=c[0];e.bits=std::stoi(c[1]);e.pub=c[2]=="published";check(e.pub||c[2]=="skipped","label role");e.run=c[3];e.p=gp_read_str(c[5].c_str());e.n=gp_read_str(c[7].c_str());e.t=subii(addis(e.p,1),e.n);e.d=subii(sqri(e.t),shifti(e.p,2));check(signe(e.d)<0&&mpodd(e.n),"cohort invariant");if(e.pub)actual|=uint32_t(1)<<rows.size();rows.push_back(e);}
 check(rows.size()==21&&__builtin_popcount(actual)==10,"complete census");std::vector<int> ls;for(int n=2;n<=2000;++n)if(uisprime(static_cast<ulong>(n)))ls.push_back(n);check(ls.size()==303,"prime catalog");const auto masks=labels(rows);check(masks.size()==4480,"recursive label inventory");
 std::vector<std::vector<int>> a(21,std::vector<int>(303)),b=a,z=a;
 std::ostringstream chars,table,cut,cal,inv;chars<<"label\tell\tcharacter\tstrict_count\tramified_count\tinclusive_count\n";table<<"label\tbits\trole\tstrict691\tramified691\tinclusive691\n";inv<<"label\ttrace\tdiscriminant\n";
 const size_t fixed=static_cast<size_t>(std::find(ls.begin(),ls.end(),691)-ls.begin());check(fixed<303,"fixed cutoff");
 for(size_t i=0;i<21;++i){inv<<rows[i].id<<'\t'<<dec(rows[i].t)<<'\t'<<dec(rows[i].d)<<'\n';int strict=0,inclusive=0,zero=0;for(size_t j=0;j<303;++j){if(j){pari_sp saved=avma;long ch=kronecker(rows[i].d,stoi(ls[j]));strict+=ch==1;inclusive+=ch>=0;zero+=ch==0;avma=saved;chars<<rows[i].id<<'\t'<<ls[j]<<'\t'<<ch<<'\t'<<strict<<'\t'<<zero<<'\t'<<inclusive<<'\n';}else chars<<rows[i].id<<"\t2\tNA_ell2\t0\t0\t0\n";a[i][j]=strict;b[i][j]=inclusive;z[i][j]=zero;}
  table<<rows[i].id<<'\t'<<rows[i].bits<<'\t'<<(rows[i].pub?"published":"skipped")<<'\t'<<a[i][fixed]<<'\t'<<z[i][fixed]<<'\t'<<b[i][fixed]<<'\n';}
 std::vector<int> good_cutoffs;cut<<"cutoff\tstrict_separates\tinclusive_separates\n";for(size_t j=0;j<303;++j){bool ok=fits(actual,a,j,rows);if(ok)good_cutoffs.push_back(ls[j]);cut<<ls[j]<<'\t'<<ok<<'\t'<<fits(actual,b,j,rows)<<'\n';}
 int good=0,one_count=0,inc_count=0;cal<<"published_mask\tstrict_any_cutoff\tstrict_fixed691\tinclusive_any_cutoff\n";for(uint32_t m:masks){bool any=false,inc=false;for(size_t j=0;j<303;++j){if(fits(m,a,j,rows))any=true;if(fits(m,b,j,rows))inc=true;}bool one=fits(m,a,fixed,rows);good+=any;one_count+=one;inc_count+=inc;cal<<m<<'\t'<<any<<'\t'<<one<<'\t'<<inc<<'\n';}
 std::ostringstream s;s<<"{\"kind\":\"strict-elkies-finite-replay/v1\",\"curve_choices\":21,\"published\":10,\"skipped\":11,\"cutoffs\":303,\"odd_prime_character_cells\":6342,\"exceptional_two_rows\":21,\"label_assignments\":4480,\"strict691_separates\":"<<(fits(actual,a,fixed,rows)?"true":"false")<<",\"strict_search_success_assignments\":"<<good<<",\"strict_fixed691_success_assignments\":"<<one_count<<",\"inclusive_search_success_assignments\":"<<inc_count<<",\"strict_separating_cutoffs\":[";for(size_t i=0;i<good_cutoffs.size();++i){if(i)s<<',';s<<good_cutoffs[i];}s<<"],\"historical_use_established\":false}\n";
 const fs::path result=argv[2];for(const auto& item:std::vector<std::pair<std::string,std::string>>{{"characters.tsv",chars.str()},{"at691.tsv",table.str()},{"cutoffs.tsv",cut.str()},{"calibration.tsv",cal.str()},{"invariants.tsv",inv.str()},{"summary.json",s.str()}})check(read(result/item.first)==item.second,"independent mismatch: "+item.first);
 std::ofstream receipt(argv[3]);check(bool(receipt),"receipt");receipt<<"{\"kind\":\"independent-pari-strict-elkies-verification/v1\",\"curve_choices\":21,\"odd_prime_character_cells_checked\":6342,\"cutoffs_checked\":303,\"label_assignments_checked\":4480,\"all_six_files_equal\":true,\"shared_integer_primitives\":\"GMP\",\"historical_use_established\":false}\n";std::cout<<"PASS: all21 rows,6342 characters,303 cutoffs,4480 labelings, complete summary\n";return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
