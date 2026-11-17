#include <gmpxx.h>
#include <openssl/sha.h>
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <vector>

using Z=mpz_class;namespace fs=std::filesystem;
static void need(bool b,const std::string& m){if(!b)throw std::runtime_error(m);}
static Z number(const std::string& s){need(!s.empty()&&s.find_first_not_of("-0123456789abcdefABCDEFxX")==std::string::npos,"bad integer");return Z(s,0);}
static Z mod(const Z& a,const Z& p){Z z;mpz_mod(z.get_mpz_t(),a.get_mpz_t(),p.get_mpz_t());return z;}
static Z power(const Z& a,const Z& k,const Z& p){Z z;mpz_powm(z.get_mpz_t(),a.get_mpz_t(),k.get_mpz_t(),p.get_mpz_t());return z;}
static Z inv(const Z& a,const Z& p){Z z;need(mpz_invert(z.get_mpz_t(),a.get_mpz_t(),p.get_mpz_t())!=0,"noninvertible denominator");return z;}
static int chi(const Z& a,const Z& p){Z z=mod(a,p);return mpz_legendre(z.get_mpz_t(),p.get_mpz_t());}
static long bits(const Z& a){return a==0?0:long(mpz_sizeinbase(a.get_mpz_t(),2));}
static long weight(const Z& a){return long(mpz_popcount(a.get_mpz_t()));}
static Z xor_z(const Z& a,const Z& b){Z z;mpz_xor(z.get_mpz_t(),a.get_mpz_t(),b.get_mpz_t());return z;}
static Z two_power(long k){Z z=1;mpz_mul_2exp(z.get_mpz_t(),z.get_mpz_t(),k);return z;}
static Z root(const Z& a,const Z& p){
 Z z=mod(a,p);need(chi(z,p)>=0,"nonsquare root requested");
 if(z==0)return 0;
 Z r;
 if(mod(p,4)==3)r=power(z,(p+1)/4,p);else{
  Z q=p-1;long m=long(mpz_scan1(q.get_mpz_t(),0));q>>=m;Z nonres=2;while(chi(nonres,p)!=-1)++nonres;
  Z c=power(nonres,q,p),t=power(z,q,p);r=power(z,(q+1)/2,p);
  while(t!=1){long i=0;Z v=t;while(v!=1&&i<m){v=mod(v*v,p);++i;}need(i<m,"Tonelli invariant failed");
   Z f=power(c,two_power(m-i-1),p);r=mod(r*f,p);c=mod(f*f,p);t=mod(t*c,p);m=i;}
 }
 Z other=p-r;return other<r?other:r;
}
struct Point{Z x=0,y=0;bool inf=true;};
static Point point(const Z& x,const Z& y){return {x,y,false};}
static bool eq(const Point& a,const Point& b){return a.inf==b.inf&&(a.inf||(a.x==b.x&&a.y==b.y));}
static bool pm(const Point& a,const Point& b){return !a.inf&&!b.inf&&a.x==b.x;}
static bool on(const Point& q,const Z& b,const Z& p){return q.inf||mod(q.y*q.y-q.x*q.x*q.x+3*q.x-b,p)==0;}
static Point add(const Point& a,const Point& b,const Z& p){
 if(a.inf)return b;
 if(b.inf)return a;
 if(a.x==b.x&&mod(a.y+b.y,p)==0)return {};
 Z s=a.x==b.x?mod((3*a.x*a.x-3)*inv(2*a.y,p),p):mod((b.y-a.y)*inv(b.x-a.x,p),p);
 Z x=mod(s*s-a.x-b.x,p);return point(x,mod(s*(a.x-x)-a.y,p));
}
static Point mul(Point a,Z k,const Z& p){need(k>=0,"negative scalar");Point z;while(k>0){if(mpz_odd_p(k.get_mpz_t()))z=add(z,a,p);a=add(a,a,p);k>>=1;}return z;}
static Point canonical(const Z& b,const Z& p){for(long x=0;x<1000;++x){Z v=mod(Z(x)*x*x-3*x+b,p);
 if(chi(v,p)>=0)return point(Z(x),root(v,p));}throw std::runtime_error("canonical bound exceeded");}
static std::vector<std::string> fields(const std::string& s){std::vector<std::string> r;size_t a=0;while(true){size_t b=s.find('\t',a);r.push_back(s.substr(a,b==std::string::npos?b:b-a));
 if(b==std::string::npos){break;} a=b+1;}return r;}
static std::vector<std::vector<std::string>> read(const fs::path& p){std::ifstream in(p);need(bool(in),"missing "+p.string());std::string s;std::vector<std::vector<std::string>> r;while(std::getline(in,s))if(!s.empty()&&s[0]!='#')r.push_back(fields(s));return r;}
static const std::vector<int> smallmods{2,3,4,5,7,8,11,13,16,17,19,23,31,32,64};
static std::vector<std::string> names(){
 std::vector<std::string> r{"x:bits","y:bits","x:hamming","y:hamming","x-b:bits","y-b:bits","x_xor_b:hamming","y_xor_b:hamming","xy-b:bits","y2-b:bits","x:chi_p","y:chi_p","x-b:chi_p","y-b:chi_p"};
 for(const std::string& v:std::vector<std::string>{"x","y","b"})for(int m:smallmods)r.push_back(v+":mod"+std::to_string(m));
 for(const std::string& a:std::vector<std::string>{"x","y","x2","y2","x3","y3","x+y","x-y","xy","x2+y2","x3+y2","x2y","xy2","2x","3x","3y"}) {
  for(const std::string& b:std::vector<std::string>{"b","-b","b+1","b-1","b+2","b-2","-3"}) {r.push_back(a+"=="+b);}
 }
 return r;
}
static std::vector<long> traits(const Point& q,const Z& b,const Z& p){
 const Z x=q.x,y=q.y,xb=mod(x-b,p),yb=mod(y-b,p);
 std::vector<long> r{bits(x),bits(y),weight(x),weight(y),bits(xb),bits(yb),weight(xor_z(x,b)),weight(xor_z(y,b)),bits(mod(x*y-b,p)),bits(mod(y*y-b,p)),chi(x,p),chi(y,p),chi(xb,p),chi(yb,p)};
 for(const Z& a:std::vector<Z>{x,y,b})for(int m:smallmods)r.push_back(mod(a,Z(m)).get_si());
 std::vector<Z> expressions{x,y,x*x,y*y,x*x*x,y*y*y,x+y,x-y,x*y,x*x+y*y,x*x*x+y*y,x*x*y,x*y*y,2*x,3*x,3*y};
 std::vector<Z> targets{b,-b,b+1,b-1,b+2,b-2,Z(-3)};for(const Z& a:expressions)for(const Z& z:targets)r.push_back(mod(a,p)==mod(z,p));return r;
}
static Z digest(Z value,size_t width,int hashbits){
 std::vector<unsigned char> bytes(width);for(size_t i=width;i>0;--i){bytes[i-1]=static_cast<unsigned char>(mod(value,256).get_ui());value>>=8;}need(value==0,"digest width");
 unsigned char result[64];
 if(hashbits==160)SHA1(bytes.data(),bytes.size(),result);else if(hashbits==256)SHA256(bytes.data(),bytes.size(),result);else SHA512(bytes.data(),bytes.size(),result);
 Z z=0;for(int i=0;i<hashbits/8;++i){z<<=8;z+=result[i];}return z;
}
static Z recipe(const std::string& name,const Z& b,const std::vector<std::string>& row){
 Z p=number(row[5]),n=number(row[7]),seed=number(row[8]);long size=std::stol(row[1]);
 if(name=="b")return b;
 if(name=="b-1")return b-1;
 if(name=="b+1")return b+1;
 if(name=="b2")return b*b;
 if(name=="b3")return b*b*b;
 if(name=="seed")return seed;
 if(name=="sha1_seed")return digest(seed,20,160);
 if(name=="sha256_seed")return digest(seed,20,256);
 if(name=="sha512_seed")return digest(seed,20,512);
 if(name=="sha256_b")return digest(b,size_t((size+7)/8),256);
 if(name=="b_low32")return mod(b,two_power(32));
 if(name=="b_high32")return b>>std::max(0L,size-32);
 if(name=="b_low160")return mod(b,two_power(160));
 if(name=="b_high160")return b>>std::max(0L,size-160);
 if(name=="trace")return p+1-n;
 if(name=="x962_seed"){long s=(size-1)/160,w=size-160*s-1;Z v=mod(digest(seed,20,160),two_power(w));for(long i=1;i<=s;++i){v<<=160;v+=digest(mod(seed+i,two_power(160)),20,160);}return v;}
 if(name=="index")return number(row[4]);
 if(name=="run")return number(row[3]);
 if(name=="one")return 1;
 if(name=="two")return 2;
 if(name=="three")return 3;
 if(name=="seven")return 7;
 throw std::runtime_error("unknown recipe");
}
struct Record{std::string view,label,role;int size,k,sign;Z p,b,n;Point q;std::vector<long> values;};
static std::string key(const std::string& view,const std::string& label,int k,int sign){return view+"/"+label+"/"+std::to_string(k)+"/"+std::to_string(sign);}
static void controls(){Z p=101,b=3;Point q=canonical(b,p);need(eq(q,point(1,1)),"tiny canonical");need(eq(mul(q,2,p),point(99,100)),"tiny doubling");need(eq(mul(q,3,p),point(80,81)),"tiny tripling");need(!on(point(1,2),b,p),"altered point accepted");need(pm(mul(q,b,p),point(80,81)),"planted scalar missed");need(!pm(mul(q,b,p),point(99,100)),"false scalar accepted");need(traits(q,b,p).size()==171,"trait catalog");Z i=root(p-1,p);need(on(point(mod(-q.x,p),mod(i*q.y,p)),p-b,p),"sign transport");std::cout<<"GMP affine controls pass; no production inputs read\n";}
int main(int argc,char** argv){try{
 if(argc==2&&std::string(argv[1])=="--self-test"){controls();return 0;}need(argc==4,"usage: verify INPUT_DIRECTORY RESULT_DIRECTORY FRESH_RECEIPT");
 fs::path in=argv[1],out=argv[2],receipt=argv[3];need(!fs::exists(receipt),"receipt exists");auto cohort=read(in/"cohort.tsv"),published=read(in/"published-points.tsv"),raw=read(out/"points.tsv");need(cohort.size()==21&&published.size()==21,"cohort changed");
 std::map<std::string,std::vector<std::string>> rows,pubs;for(const auto& r:cohort)need(r.size()==9&&rows.emplace(r[0],r).second,"bad cohort row");for(const auto& r:published)need(r.size()==8&&pubs.emplace(r[0],r).second,"bad point source");
 auto catalog=names();need(raw.size()==589&&raw.front().size()==182,"missing point record/catalog");for(size_t i=0;i<catalog.size();++i)need(raw[0][i+11]==catalog[i],"trait name mismatch");
 std::vector<Record> records;std::map<std::string,size_t> lookup;long cells=0;
 for(size_t i=1;i<raw.size();++i){const auto& r=raw[i];need(r.size()==182,"point row width");const auto& src=rows.at(r[1]);Z p=number(src[5]),b=number(src[6]),n=number(src[7]);
  need(r[0]=="historical"||r[0]=="smaller_b_p224","unknown coefficient view");
 if(r[0]=="smaller_b_p224"&&src[1]=="224"&&p-b<b)b=p-b;
  need(number(r[4])==p&&number(r[5])==b&&number(r[6])==n&&r[2]==src[2]&&r[3]==src[1],"point identity mismatch");
  Z bound;Z fourp=4*p;mpz_sqrt(bound.get_mpz_t(),fourp.get_mpz_t());need((p+1-bound+n-1)/n==1&&(p+1+bound)/n==1,"Hasse uniqueness mismatch");
  int k=std::stoi(r[7]),sign=std::stoi(r[8]);need(std::set<int>{1,2,3,5,7,11,17}.count(k)&&std::set<int>{1,-1}.count(sign),"undeclared base choice");
  Point expected=mul(canonical(b,p),Z(k),p);
 if(sign<0){expected.y=mod(-expected.y,p);} Point q=point(number(r[9]),number(r[10]));need(eq(q,expected)&&on(q,b,p)&&mul(q,n,p).inf,"point/group check failed");
  auto v=traits(q,b,p);for(size_t f=0;f<v.size();++f){need(r[f+11]==std::to_string(v[f]),"trait value mismatch");++cells;}
  need(lookup.emplace(key(r[0],r[1],k,sign),records.size()).second,"duplicate point row");records.push_back({r[0],r[1],r[2],std::stoi(r[3]),k,sign,p,b,n,q,v});
 }
 auto historical=read(out/"historical-points.tsv");need(historical.size()==43,"historical rows lost");std::map<std::string,Point> generators;std::set<std::string> historical_keys;
 for(size_t i=1;i<historical.size();++i){const auto& h=historical[i];need(h.size()==6,"historical width");const auto& r=records.at(lookup.at(key(h[0],h[1],1,1)));const auto& source=pubs.at(h[1]);
  need(historical_keys.insert(h[0]+"/"+h[1]).second,"duplicate historical row");
 if(r.role=="skipped"){need(h[2]=="unknown"&&h[3]=="NA"&&h[4]=="NA"&&h[5]=="NA","unknown historical point changed");continue;}
  Point g=point(number(source[6]),number(source[7]));Z old=number(rows.at(h[1])[6]),i_map=1;
  if(r.b!=old){i_map=root(r.p-1,r.p);g=point(mod(-g.x,r.p),mod(i_map*g.y,r.p));}
  need(h[2]==(r.b==old?"known":"transported-known")&&number(h[3])==g.x&&number(h[4])==g.y&&number(h[5])==i_map&&on(g,r.b,r.p)&&mul(g,r.n,r.p).inf,"historical point check failed");
  need(generators.emplace(h[0]+"/"+h[1],g).second,"duplicate historical point");
 }
 auto recipes=read(out/"recipes.tsv");need(recipes.size()==441,"recipe rows lost");long recipe_tests=0,recipe_matches=0;std::set<std::string> recipe_keys;
 for(size_t i=1;i<recipes.size();++i){const auto& t=recipes[i];need(t.size()==7,"recipe width");const auto& r=records.at(lookup.at(key(t[0],t[1],1,1)));Point g=generators.at(t[0]+"/"+t[1]);Z s=mod(recipe(t[2],r.b,rows.at(t[1])),r.n);
  need(recipe_keys.insert(t[0]+"/"+t[1]+"/"+t[2]).second,"duplicate recipe row");need(number(t[3])==s,"recipe scalar mismatch");
 if(s==0){need(t[4]=="invalid-zero"&&t[5]=="NA"&&t[6]=="NA","zero recipe encoded numerically");continue;}
  need(t[4]=="tested"&&t[5]==std::to_string(pm(mul(r.q,s,r.p),g))&&t[6]==std::to_string(pm(mul(g,s,r.p),r.q)),"recipe equality mismatch");recipe_tests+=2;recipe_matches+=long(t[5]=="1")+long(t[6]=="1");
 }
 auto cutoffs=read(out/"cutoffs.tsv");need(cutoffs.size()==19153,"cutoff rows lost");long cutoff_count=0,separators=0;std::set<std::string> cutoff_keys;
 for(size_t i=1;i<cutoffs.size();++i){const auto& t=cutoffs[i];need(t.size()==9,"cutoff width");auto it=std::find(catalog.begin(),catalog.end(),t[3]);need(it!=catalog.end(),"unknown cutoff trait");size_t f=size_t(it-catalog.begin());
  need(cutoff_keys.insert(t[0]+"/"+t[1]+"/"+t[2]+"/"+t[3]+"/"+t[4]+"/"+t[5]).second,"duplicate cutoff row");bool field=t[4]=="per_field",upper=t[5]=="upper";need((field||t[4]=="global")&&(upper||t[5]=="lower"),"cutoff model changed");std::map<int,long> bound;
  for(const auto& r:records)if(r.view==t[0]&&r.k==std::stoi(t[1])&&r.sign==std::stoi(t[2])&&r.role=="published"){int z=field?r.size:0;
 if(!bound.count(z))bound[z]=r.values[f];else if(upper)bound[z]=std::max(bound[z],r.values[f]);else bound[z]=std::min(bound[z],r.values[f]);}
  int pub=0,skip=0;std::string kept;for(const auto& r:records)if(r.view==t[0]&&r.k==std::stoi(t[1])&&r.sign==std::stoi(t[2])){long b=bound.at(field?r.size:0);bool keep=upper?r.values[f]<=b:r.values[f]>=b;
 if(keep){if(r.role=="published")++pub;else{++skip;
 if(!kept.empty()){kept+=",";} kept+=r.label;}}}
  need(pub==10&&t[6]==std::to_string(pub)&&t[7]==std::to_string(skip)&&t[8]==kept,"cutoff outcome mismatch");++cutoff_count;if(skip==0)++separators;
 }
 auto pairwise=read(out/"point-pairwise.tsv");long differences=0;std::set<std::string> difference_keys;
 for(size_t i=1;i<pairwise.size();++i){const auto& t=pairwise[i];need(t.size()==9,"difference width");need(difference_keys.insert(t[0]+"/"+t[1]+"/"+t[2]+"/"+t[3]+"/"+t[4]+"/"+t[5]).second,"duplicate difference row");const auto& a=records.at(lookup.at(key(t[0],t[3],std::stoi(t[1]),std::stoi(t[2]))));const auto& b=records.at(lookup.at(key(t[0],t[4],std::stoi(t[1]),std::stoi(t[2]))));auto f=std::find(catalog.begin(),catalog.end(),t[5]);need(f!=catalog.end()&&a.role=="published"&&b.role=="skipped"&&a.size==b.size,"invalid matched pair");size_t j=size_t(f-catalog.begin());need(t[6]==std::to_string(a.values[j])&&t[7]==std::to_string(b.values[j])&&t[8]==std::to_string(b.values[j]-a.values[j]),"difference mismatch");++differences;}
 need(differences==119700,"matched differences incomplete");
 auto supplied=read(in/"supplied-points.tsv"),checks=read(out/"supplied-point-checks.tsv");need(supplied.size()==4&&checks.size()==5,"supplied controls lost");
 for(size_t i=0;i<supplied.size();++i){const auto& s=supplied[i];const auto& c=checks[i+1];Z p=number(s[5]),b=number(s[6]);Point q=canonical(b,p);need(c.size()==7&&c[0]==s[0]&&number(c[2])==b&&number(c[3])==q.x&&number(c[4])==q.y&&q.x==number(s[7])&&q.y==number(s[8])&&c[5]=="1"&&c[6]=="1"&&mul(q,number(s[9]),p).inf,"supplied point mismatch");}
 std::map<std::string,std::map<std::string,std::string>> inherited;
 for(size_t i=0;i<cohort.size();++i){auto f=read(in/"existing-features"/("curve-"+std::to_string(i)+".features.tsv"));need(f.size()==1127,"inherited source incomplete");for(const auto& v:f)need(v.size()==2&&inherited[cohort[i][0]].emplace(v[0],v[1]).second,"duplicate inherited feature");}
 auto matrix=read(out/"existing-feature-matrix.tsv");need(matrix.size()==22&&matrix[0].size()==1131,"inherited matrix shape");std::set<std::string> matrix_rows;
 for(size_t i=1;i<matrix.size();++i){const auto& r=matrix[i];need(r.size()==1131&&matrix_rows.insert(r[0]).second&&r[1]==rows.at(r[0])[2]&&r[2]==rows.at(r[0])[1]&&r[3]=="historical","inherited matrix identity");for(size_t j=4;j<r.size();++j)need(r[j]==inherited.at(r[0]).at(matrix[0][j]),"inherited value changed");}
 auto inherited_pairs=read(out/"existing-pairwise.tsv");need(inherited_pairs.size()==28176,"inherited pair count");std::set<std::string> base_pairs;
 for(size_t i=1;i<inherited_pairs.size();++i){const auto& r=inherited_pairs[i];need(r.size()==6&&base_pairs.insert(r[0]+"/"+r[1]+"/"+r[2]).second&&rows.at(r[0])[2]=="published"&&rows.at(r[1])[2]=="skipped"&&rows.at(r[0])[1]==rows.at(r[1])[1],"inherited pair identity");const auto& a=inherited.at(r[0]).at(r[2]);const auto& b=inherited.at(r[1]).at(r[2]);need(r[3]==a&&r[4]==b&&number(r[5])==number(b)-number(a),"inherited difference mismatch");}
 bool synthetic=std::all_of(cohort.begin(),cohort.end(),[](const auto& r){return r[0].rfind("fixture-",0)==0;});
 std::ostringstream headline;headline<<"{\"kind\":\"native-generator-b-comparison/v1\",\"input_class\":\""<<(synthetic?"synthetic-development":"production-parameter-analysis")<<"\",\"curve_choices\":21,\"published\":10,\"skipped\":11,\"coefficient_views\":2,\"point_rows\":"<<records.size()<<",\"traits_per_point\":"<<catalog.size()<<",\"trait_cells\":"<<cells<<",\"cutoff_comparisons\":"<<cutoff_count<<",\"exact_single_separators\":"<<separators<<",\"scalar_recipe_tests\":"<<recipe_tests<<",\"scalar_recipe_matches\":"<<recipe_matches<<",\"historical_skip_generators\":\"unknown for all11\",\"historical_use_established\":false}";
 std::ifstream summary_file(out/"summary.json");std::string summary_line,extra;need(bool(summary_file)&&bool(std::getline(summary_file,summary_line))&&summary_line==headline.str()&&!std::getline(summary_file,extra),"headline summary mismatch");
 std::ofstream result(receipt);result<<"{\"kind\":\"independent-gmp-generator-verification/v1\",\"input_class\":\""<<(synthetic?"synthetic-development":"production-parameter-analysis")<<"\",\"curve_choices\":21,\"point_rows\":"<<records.size()<<",\"trait_cells_checked\":"<<cells<<",\"cutoffs_checked\":"<<cutoff_count<<",\"point_differences_checked\":"<<differences<<",\"inherited_differences_checked\":28175,\"recipe_tests_checked\":"<<recipe_tests<<",\"unknown_historical_skip_generators_retained\":true,\"sha_implementation\":\"shared OpenSSL; group/trait arithmetic separately GMP\",\"historical_use_established\":false}\n";
 std::cout<<"GMP verification passes all generated points, traits, recipes, cutoffs and point differences\n";return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
