#include "../native/common.hpp"
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>

namespace fs = std::filesystem;
static const std::vector<int> mods{2,3,4,5,7,8,11,13,16,17,19,23,31,32,64};
static const std::vector<int> multiples{1,2,3,5,7,11,17};
static const std::vector<std::string> forms{
 "x","y","x2","y2","x3","y3","x+y","x-y","xy","x2+y2",
 "x3+y2","x2y","xy2","2x","3x","3y"};
static const std::vector<std::string> targets{"b","-b","b+1","b-1","b+2","b-2","-3"};

static std::vector<std::string> split(const std::string& s) {
 std::vector<std::string> v; std::stringstream in(s); std::string t;
 while(std::getline(in,t,'\t')) v.push_back(t); return v;
}
static std::vector<std::vector<std::string>> table(const fs::path& p, size_t width) {
 std::ifstream in(p); require(bool(in),"cannot open "+p.string());
 std::vector<std::vector<std::string>> rows; std::string s;
 while(std::getline(in,s)) { if(s.empty()||s[0]=='#')continue;
  auto v=split(s); require(v.size()==width,"wrong table width "+p.string()); rows.push_back(v); }
 return rows;
}
static GEN multiple(GEN e,GEN q,GEN k) { return liftall(ellmul(e,q,k)); }
static bool infinity(GEN p) { return lg(p)==2&&gequal0(gel(p,1)); }
static bool same_x(GEN p,GEN q) { return !infinity(p)&&!infinity(q)&&equalii(gel(p,1),gel(q,1)); }
static GEN rhs(GEN x,GEN b,GEN p) { return modii(addii(subii(powiu(x,3),mulsi(3,x)),b),p); }
static GEN canonical(GEN b,GEN p) {
 for(long i=0;i<1000;++i) {GEN x=stoi(i),r=rhs(x,b,p); if(kronecker(r,p)>=0) {
  GEN y=Fp_sqrt(r,p), other=subii(p,y); if(cmpii(other,y)<0)y=other; return mkvec2(x,y); }}
 throw std::runtime_error("no canonical point in declared 1000-x bound");
}
static std::vector<std::string> catalog() {
 std::vector<std::string> v{"x:bits","y:bits","x:hamming","y:hamming",
  "x-b:bits","y-b:bits","x_xor_b:hamming","y_xor_b:hamming",
  "xy-b:bits","y2-b:bits","x:chi_p","y:chi_p","x-b:chi_p","y-b:chi_p"};
 for(const auto& s:std::vector<std::string>{"x","y","b"})for(int m:mods)v.push_back(s+":mod"+std::to_string(m));
 for(const auto& s:forms)for(const auto& t:targets)v.push_back(s+"=="+t); return v;
}
static std::vector<long> features(GEN x,GEN y,GEN b,GEN p) {
 GEN xb=modii(subii(x,b),p),yb=modii(subii(y,b),p);
 GEN x2=sqri(x),y2=sqri(y),x3=mulii(x2,x),y3=mulii(y2,y);
 std::vector<long> v{bit_length(x),bit_length(y),popcount(x),popcount(y),
  bit_length(xb),bit_length(yb),popcount(ibitxor(x,b)),popcount(ibitxor(y,b)),
  bit_length(modii(subii(mulii(x,y),b),p)),bit_length(modii(subii(y2,b),p)),
  kronecker(x,p),kronecker(y,p),kronecker(xb,p),kronecker(yb,p)};
 for(GEN a:std::vector<GEN>{x,y,b})for(int m:mods)v.push_back(umodiu(a,m));
 std::vector<GEN> f{x,y,x2,y2,x3,y3,addii(x,y),subii(x,y),mulii(x,y),
  addii(x2,y2),addii(x3,y2),mulii(x2,y),mulii(x,y2),shifti(x,1),mulsi(3,x),mulsi(3,y)};
 std::vector<GEN> t{b,negi(b),addis(b,1),subis(b,1),addis(b,2),subis(b,2),stoi(-3)};
 for(GEN a:f)for(GEN c:t)v.push_back(equalii(modii(a,p),modii(c,p))?1:0);
 require(v.size()==catalog().size(),"feature catalog mismatch"); return v;
}
static GEN digest_integer(GEN value,size_t bytes,int bits) {
 std::vector<unsigned char> in(bytes); GEN t=value;
 for(size_t i=bytes;i>0;--i){in[i-1]=umodiu(t,256);t=shifti(t,-8);} require(signe(t)==0,"digest input width");
 unsigned char out[64]; if(bits==160)SHA1(in.data(),in.size(),out);
 else if(bits==256)SHA256(in.data(),in.size(),out); else SHA512(in.data(),in.size(),out);
 return from_bytes(out,size_t(bits/8));
}
static std::vector<std::pair<std::string,GEN>> recipes(GEN b,GEN seed,GEN p,GEN n,long bits,long index,long run) {
 return {{"b",b},{"b-1",subis(b,1)},{"b+1",addis(b,1)},{"b2",sqri(b)},{"b3",powiu(b,3)},
  {"seed",seed},{"sha1_seed",digest_integer(seed,20,160)},
  {"sha256_seed",digest_integer(seed,20,256)},{"sha512_seed",digest_integer(seed,20,512)},
  {"sha256_b",digest_integer(b,size_t((bits+7)/8),256)},
  {"b_low32",modii(b,int2n(32))},{"b_high32",shifti(b,-std::max(0L,bits-32))},
  {"b_low160",modii(b,int2n(160))},{"b_high160",shifti(b,-std::max(0L,bits-160))},
  {"trace",subii(addis(p,1),n)},{"x962_seed",x962(seed,bits)},
  {"index",stoi(index)},{"run",stoi(run)},{"one",gen_1},{"two",gen_2},{"three",stoi(3)},{"seven",stoi(7)}};
}
struct Record {std::string view,label,role;int bits,k,sign;std::vector<long> values;};
static GEN signed_integer(const std::string& s) {
 return !s.empty()&&s[0]=='-'?negi(integer(s.substr(1))):integer(s);
}
static void existing_inventory(const fs::path& input,const fs::path& out,
 const std::vector<std::vector<std::string>>& rows) {
 std::vector<std::map<std::string,std::string>> values;std::set<std::string> keys;
 for(size_t i=0;i<rows.size();++i){std::map<std::string,std::string> row;
  for(const auto& v:table(input/"existing-features"/("curve-"+std::to_string(i)+".features.tsv"),2)){
   require(row.emplace(v[0],v[1]).second,"duplicate inherited feature");
   pari_sp mark=avma;(void)signed_integer(v[1]);avma=mark;
  }
  require(row.size()==1127,"incomplete inherited feature catalog");
  std::set<std::string> now;for(const auto& v:row)now.insert(v.first);
  if(i==0)keys=now;else require(keys==now,"inherited feature catalogs differ");values.push_back(std::move(row));
 }
 std::ofstream matrix(out/"existing-feature-matrix.tsv"),diff(out/"existing-pairwise.tsv");
 matrix<<"label\trole\tbits\tcoefficient_convention";for(const auto& key:keys)matrix<<'\t'<<key;matrix<<'\n';
 for(size_t i=0;i<rows.size();++i){matrix<<rows[i][0]<<'\t'<<rows[i][2]<<'\t'<<rows[i][1]<<"\thistorical";for(const auto& key:keys)matrix<<'\t'<<values[i].at(key);matrix<<'\n';}
 diff<<"published\tskipped\tfeature\tpublished_value\tskipped_value\tskipped_minus_published\n";
 for(size_t i=0;i<rows.size();++i)for(size_t j=0;j<rows.size();++j)if(rows[i][2]=="published"&&rows[j][2]=="skipped"&&rows[i][1]==rows[j][1])for(const auto& key:keys){
  pari_sp mark=avma;GEN delta=subii(signed_integer(values[j].at(key)),signed_integer(values[i].at(key)));
  diff<<rows[i][0]<<'\t'<<rows[j][0]<<'\t'<<key<<'\t'<<values[i].at(key)<<'\t'<<values[j].at(key)<<'\t'<<decimal(delta)<<'\n';avma=mark;
 }
}
static void supplied_controls(const fs::path& input,const fs::path& out,
 const std::vector<std::vector<std::string>>& rows) {
 auto supplied=table(input/"supplied-points.tsv",12);require(supplied.size()==4,"retain all four supplied points");
 std::ofstream f(out/"supplied-point-checks.tsv");f<<"label\tcohort_label\tb\tx\ty\tcanonical_equal\tn_kills\n";
 for(const auto& s:supplied){pari_sp mark=avma;const std::vector<std::string>* found=nullptr;
  for(const auto& r:rows)if(r[1]==s[1]&&r[4]==s[4]){require(!found,"ambiguous supplied identity");found=&r;}
  require(found,"supplied curve absent from cohort");GEN p=integer(s[5]),b=integer(s[6]),n=integer(s[9]);
  require(equalii(p,integer((*found)[5]))&&equalii(n,integer((*found)[7]))&&equalii(integer(s[10]),integer((*found)[8])),"supplied identity mismatch");
  GEN old=integer((*found)[6]);require(equalii(b,old)||(s[1]=="224"&&equalii(b,subii(p,old))),"unexpected supplied coefficient model");
  GEN q=canonical(b,p),g=mkvec2(integer(s[7]),integer(s[8])),e=ellinit(mkvec2(stoi(-3),b),p,DEFAULTPREC);
  require(gequal(q,g)&&!gequal0(ellisoncurve(e,g))&&infinity(multiple(e,g,n)),"supplied canonical point failed");
  f<<s[0]<<'\t'<<(*found)[0]<<'\t'<<decimal(b)<<'\t'<<decimal(gel(g,1))<<'\t'<<decimal(gel(g,2))<<"\t1\t1\n";avma=mark;
 }
}
static void self_test() {
 GEN p=stoi(101),b=stoi(3),e=ellinit(mkvec2(stoi(-3),b),p,DEFAULTPREC),q=canonical(b,p);
 require(equali1(gel(q,1))&&equali1(gel(q,2)),"canonical tiny fixture");
 GEN q2=multiple(e,q,gen_2),q3=multiple(e,q,stoi(3));
 require(equaliu(gel(q2,1),99)&&equaliu(gel(q2,2),100),"tiny doubling");
 require(equaliu(gel(q3,1),80)&&equaliu(gel(q3,2),81),"tiny tripling");
 require(same_x(multiple(e,q,b),q3),"planted b-scalar relation missed");
 require(!same_x(multiple(e,q,b),q2),"negative scalar control accepted");
 require(features(gel(q,1),gel(q,2),b,p).size()==171,"catalog count");
 GEN i=Fp_sqrt(stoi(100),p),mapped=mkvec2(Fp_neg(gel(q,1),p),Fp_mul(i,gel(q,2),p));
 GEN other=ellinit(mkvec2(stoi(-3),stoi(98)),p,DEFAULTPREC);
 require(!gequal0(ellisoncurve(other,mapped)),"sign-map tiny fixture");
 require(!gequal0(ellisoncurve(e,q)),"positive on-curve control");
 require(gequal0(ellisoncurve(e,mkvec2(gen_1,gen_2))),"altered point accepted");
 std::cout<<"native controls passed;171 traits; no production inputs read\n";
}
int main(int argc,char** argv) {
 try {pari_init(128000000,2000000);
  if(argc==2&&std::string(argv[1])=="--self-test"){self_test();return 0;}
  if(argc==2&&std::string(argv[1])=="--catalog"){for(const auto& x:catalog())std::cout<<x<<'\n';return 0;}
  require(argc==3,"usage: analyze INPUT_DIRECTORY FRESH_OUTPUT_DIRECTORY");fs::path input=argv[1],out=argv[2];
  require(!fs::exists(out),"output directory already exists");fs::create_directories(out);
  auto rows=table(input/"cohort.tsv",9),pub=table(input/"published-points.tsv",8);
  require(rows.size()==21&&pub.size()==21,"retain exactly21 rows");
  existing_inventory(input,out,rows);supplied_controls(input,out,rows);
  std::map<std::string,std::vector<std::string>> points;for(const auto& r:pub)require(points.emplace(r[0],r).second,"duplicate published point label");
  std::set<std::string> labels;long published=0,skipped=0,recipe_tests=0,recipe_matches=0;
  std::ofstream pts(out/"points.tsv"),hist(out/"historical-points.tsv"),rec(out/"recipes.tsv"),cuts(out/"cutoffs.tsv");
  auto names=catalog();pts<<"view\tlabel\trole\tbits\tp\tb\tn\tk\tsign\tx\ty";for(const auto& x:names)pts<<'\t'<<x;pts<<'\n';
  hist<<"view\tlabel\tstatus\tx\ty\ttransport_i\n";
  rec<<"view\tlabel\trecipe\tscalar\tstatus\tG_eq_pm_sQ\tQ_eq_pm_sG\n";
  std::vector<Record> records;
  for(const auto& r:rows){require(labels.insert(r[0]).second,"duplicate cohort label");require(r[2]=="published"||r[2]=="skipped","unknown role");
   if(r[2]=="published")++published;else++skipped;
   for(const std::string& view:std::vector<std::string>{"historical","smaller_b_p224"}){
    pari_sp mark=avma;GEN p=integer(r[5]),source_b=integer(r[6]),n=integer(r[7]),seed=integer(r[8]);long bits=std::stol(r[1]);GEN b=source_b;
    if(view=="smaller_b_p224"&&bits==224&&cmpii(subii(p,b),b)<0)b=subii(p,b);
    require(isprime(p)&&isprime(n),"prior prime input check failed");require(bit_length(p)==bits&&bit_length(n)<=bits,"field/order bit grouping mismatch");require(cmpii(n,stoi(17))>0,"small order outside cohort");
    require(signe(source_b)>=0&&cmpii(source_b,p)<0,"coefficient out of range");
    GEN hb=sqrtint(shifti(p,2)),center=addis(p,1);
    require(equali1(divii(addii(subii(center,hb),subis(n,1)),n))&&equali1(divii(addii(center,hb),n)),"group-order Hasse uniqueness failed");
    require(signe(modii(subis(sqri(b),4),p))!=0,"singular input");
    GEN e=ellinit(mkvec2(stoi(-3),b),p,DEFAULTPREC),q=canonical(b,p);
    require(infinity(multiple(e,q,n)),"canonical point not killed by n");
    const auto& u=points.at(r[0]);require(equalii(integer(u[2]),p)&&equalii(integer(u[3]),source_b)&&equalii(integer(u[4]),n)&&equalii(integer(u[5]),seed),"published table identity mismatch");
    if(r[2]=="published"){
     require(u[6]!="NA"&&u[7]!="NA","missing published generator");GEN g=mkvec2(integer(u[6]),integer(u[7])),i=gen_1;
     require(signe(gel(g,1))>=0&&signe(gel(g,2))>=0&&cmpii(gel(g,1),p)<0&&cmpii(gel(g,2),p)<0,"standard generator coordinate out of range");
     GEN se=ellinit(mkvec2(stoi(-3),source_b),p,DEFAULTPREC);require(!gequal0(ellisoncurve(se,g))&&infinity(multiple(se,g,n)),"standard generator invalid");
     if(!equalii(b,source_b)){require(bits==224&&umodiu(p,4)==1,"unsupported sign transport");i=Fp_sqrt(subis(p,1),p);if(cmpii(subii(p,i),i)<0)i=subii(p,i);g=mkvec2(Fp_neg(gel(g,1),p),Fp_mul(i,gel(g,2),p));}
     require(!gequal0(ellisoncurve(e,g))&&infinity(multiple(e,g,n)),"transported generator invalid");
     hist<<view<<'\t'<<r[0]<<'\t'<<(equalii(b,source_b)?"known":"transported-known")<<'\t'<<decimal(gel(g,1))<<'\t'<<decimal(gel(g,2))<<'\t'<<decimal(i)<<'\n';
     for(const auto& model:recipes(b,seed,p,n,bits,std::stol(r[4]),std::stol(r[3]))){GEN s=modii(model.second,n);if(!signe(s)){rec<<view<<'\t'<<r[0]<<'\t'<<model.first<<"\t0\tinvalid-zero\tNA\tNA\n";continue;}
      bool f=same_x(multiple(e,q,s),g),v=same_x(multiple(e,g,s),q);recipe_tests+=2;recipe_matches+=long(f)+long(v);
      rec<<view<<'\t'<<r[0]<<'\t'<<model.first<<'\t'<<decimal(s)<<"\ttested\t"<<f<<'\t'<<v<<'\n';}
    }else{require(u[6]=="NA"&&u[7]=="NA","invented skipped historical generator");hist<<view<<'\t'<<r[0]<<"\tunknown\tNA\tNA\tNA\n";}
    for(int k:multiples)for(int sign:std::vector<int>{1,-1}){
     pari_sp pm=avma;GEN z=multiple(e,q,stoi(k));require(!infinity(z),"zero multiple");if(sign<0)z=liftall(ellneg(e,z));
     require(!gequal0(ellisoncurve(e,z)),"computed point off curve");auto v=features(gel(z,1),gel(z,2),b,p);
     pts<<view<<'\t'<<r[0]<<'\t'<<r[2]<<'\t'<<bits<<'\t'<<decimal(p)<<'\t'<<decimal(b)<<'\t'<<decimal(n)<<'\t'<<k<<'\t'<<sign<<'\t'<<decimal(gel(z,1))<<'\t'<<decimal(gel(z,2));for(long a:v)pts<<'\t'<<a;pts<<'\n';
     records.push_back({view,r[0],r[2],int(bits),k,sign,std::move(v)});avma=pm;
    }avma=mark;
   }
  }
  require(published==10&&skipped==11,"cohort roles changed");
  cuts<<"view\tk\tsign\tfeature\tscope\torientation\tpublished_kept\tskipped_kept\tretained_skips\n";
  std::ofstream pairs(out/"point-pairwise.tsv");pairs<<"view\tk\tsign\tpublished\tskipped\tfeature\tpublished_value\tskipped_value\tskipped_minus_published\n";
  for(const auto& a:records)if(a.role=="published")for(const auto& b:records)if(b.role=="skipped"&&a.view==b.view&&a.k==b.k&&a.sign==b.sign&&a.bits==b.bits)for(size_t f=0;f<names.size();++f)
   pairs<<a.view<<'\t'<<a.k<<'\t'<<a.sign<<'\t'<<a.label<<'\t'<<b.label<<'\t'<<names[f]<<'\t'<<a.values[f]<<'\t'<<b.values[f]<<'\t'<<b.values[f]-a.values[f]<<'\n';
  long comparisons=0,separators=0;
  for(const std::string& view:std::vector<std::string>{"historical","smaller_b_p224"})for(int k:multiples)for(int sign:std::vector<int>{1,-1})for(size_t f=0;f<names.size();++f)for(bool field:std::vector<bool>{false,true})for(bool upper:std::vector<bool>{true,false}){
   std::map<int,long> bound;std::vector<const Record*> subset;for(const auto& z:records)if(z.view==view&&z.k==k&&z.sign==sign){subset.push_back(&z);if(z.role=="published"){int key=field?z.bits:0;if(!bound.count(key))bound[key]=z.values[f];else bound[key]=upper?std::max(bound[key],z.values[f]):std::min(bound[key],z.values[f]);}}
   int pk=0,sk=0;std::string retained;for(const auto* z:subset){long b=bound.at(field?z->bits:0);bool keep=upper?z->values[f]<=b:z->values[f]>=b;if(keep){if(z->role=="published")++pk;else{++sk;if(!retained.empty())retained+=",";retained+=z->label;}}}
   require(pk==10&&subset.size()==21,"cutoff lost a row/control");++comparisons;if(sk==0)++separators;
   cuts<<view<<'\t'<<k<<'\t'<<sign<<'\t'<<names[f]<<'\t'<<(field?"per_field":"global")<<'\t'<<(upper?"upper":"lower")<<'\t'<<pk<<'\t'<<sk<<'\t'<<retained<<'\n';
  }
  bool synthetic=std::all_of(rows.begin(),rows.end(),[](const auto& r){return r[0].rfind("fixture-",0)==0;});
  std::ofstream summary(out/"summary.json");summary<<"{\"kind\":\"native-generator-b-comparison/v1\",\"input_class\":\""<<(synthetic?"synthetic-development":"production-parameter-analysis")<<"\",\"curve_choices\":21,\"published\":10,\"skipped\":11,\"coefficient_views\":2,\"point_rows\":"<<records.size()<<",\"traits_per_point\":"<<names.size()<<",\"trait_cells\":"<<records.size()*names.size()<<",\"cutoff_comparisons\":"<<comparisons<<",\"exact_single_separators\":"<<separators<<",\"scalar_recipe_tests\":"<<recipe_tests<<",\"scalar_recipe_matches\":"<<recipe_matches<<",\"historical_skip_generators\":\"unknown for all11\",\"historical_use_established\":false}\n";
  std::cout<<"completed21 choices; retain both conventions and all unknown historical points\n";
  pari_close();return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
