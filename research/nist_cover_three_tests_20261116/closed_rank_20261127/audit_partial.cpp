#include "typed_json.hpp"
#include "rank_roots.hpp"
#include <openssl/evp.h>
#include <memory>
#include <iostream>
#include <iomanip>
#include <set>

using AP=mpz_class;
using AL=std::array<AP,3>;
struct AC {AP x,y;};
struct AT {int x,y;AP coefficient;};
const AP ap=sc_p;
AP amod(AP x){mpz_mod(x.get_mpz_t(),x.get_mpz_t(),ap.get_mpz_t());return x;}
AP ainv(const AP&x){AP out;require(mpz_invert(out.get_mpz_t(),x.get_mpz_t(),ap.get_mpz_t())!=0,"GMP geometry inverse");return out;}
AP apower(AP x,int n){AP out=1;while(n--){out=amod(out*x);}return out;}
AP avalue(const std::vector<AT>&c,const AC&p,int dx=0,int dy=0){AP out=0;for(const auto&t:c)if(t.x>=dx&&t.y>=dy){AP v=t.coefficient;for(int i=0;i<dx;++i)v*=t.x-i;for(int i=0;i<dy;++i)v*=t.y-i;out=amod(out+v*apower(p.x,t.x-dx)*apower(p.y,t.y-dy));}return out;}
void aincidence(const std::vector<AT>&c,const AC&p){require(p.x>=0&&p.x<ap&&p.y>=0&&p.y<ap&&avalue(c,p)==0,"GMP actual canonical curve point");}
AL anormalize(AL line){require(line[0]!=0||line[1]!=0,"GMP nonconstant line");AP inverse=ainv(line[0]!=0?line[0]:line[1]);for(auto&v:line)v=amod(v*inverse);return line;}
std::string akey(const AL&line){return line[0].get_str()+":"+line[1].get_str()+":"+line[2].get_str();}
AL athrough(const AC&a,const AC&b){return anormalize({amod(a.y-b.y),amod(b.x-a.x),amod(a.x*b.y-a.y*b.x)});}
std::array<AP,5> acontact(const std::vector<AT>&c,const AC&p,const AL&line){
 const int choose[5][5]={{1,0,0,0,0},{1,1,0,0,0},{1,2,1,0,0},{1,3,3,1,0},{1,4,6,4,1}};AP dx=line[1],dy=amod(-line[0]);std::array<AP,5> out{};
 for(const auto&t:c)for(int i=0;i<=t.x;++i)for(int j=0;j<=t.y;++j){int k=i+j;require(k<=4,"quartic expansion degree");out[size_t(k)]=amod(out[size_t(k)]+t.coefficient*choose[t.x][i]*choose[t.y][j]*apower(p.x,t.x-i)*apower(p.y,t.y-j)*apower(dx,i)*apower(dy,j));}
 return out;
}
struct AH {
 std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> ctx{EVP_MD_CTX_new(),EVP_MD_CTX_free};
 AH(){require(bool(ctx)&&EVP_DigestInit_ex(ctx.get(),EVP_sha256(),nullptr)==1,"audit line digest initialize");}
 void add(size_t i,size_t j,const std::string&key){std::array<unsigned char,12> bytes{};for(size_t k=0;k<4;++k){bytes[k]=static_cast<unsigned char>((i>>(8*k))&255U);bytes[k+4]=static_cast<unsigned char>((j>>(8*k))&255U);bytes[k+8]=static_cast<unsigned char>((key.size()>>(8*k))&255U);}require(EVP_DigestUpdate(ctx.get(),bytes.data(),bytes.size())==1&&EVP_DigestUpdate(ctx.get(),key.data(),key.size())==1,"audit line digest update");}
 std::string finish(){std::array<unsigned char,32> bytes{};unsigned int size=0;require(EVP_DigestFinal_ex(ctx.get(),bytes.data(),&size)==1&&size==32,"audit digest finish");std::ostringstream out;for(auto b:bytes)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);return out.str();}
};
template<class T>void areject_partial(T action){bool bad=false;try{action();}catch(const std::exception&){bad=true;}require(bad,"independent geometry corruption control accepted");}
void ahistogram(const Json&j,const std::map<int,size_t>&counts){require(j.obj.size()==counts.size(),"same support-degree histogram bins");for(auto[d,n]:counts)require(j.at(std::to_string(d)).integer()==int(n),"same exact support-degree histogram");}
int main(int argc,char**argv){try{
 require(argc==5,"usage: audit_partial QUARTIC SCAN PRIMARY TANGENTS");auto model=read_json(argv[1]),scan=read_json(argv[2]),primary=read_json(argv[3]),tangents=read_json(argv[4]);require(AP(model.at("p").scalar)==ap,"exact independent prime field");
 std::vector<AT> c;for(const auto&t:model.at("terms").arr){const auto&e=t.at("exponents").arr;require(e.size()==3&&e[0].integer()+e[1].integer()+e[2].integer()==4,"homogeneous quartic terms");int x=e[0].integer(),y=e[1].integer();require(x>=0&&y>=0&&x+y<=4,"quartic affine exponents");c.push_back({x,y,AP(t.at("coefficient").scalar)});}
 std::vector<AC> points;for(const auto&q:scan.at("points").arr){AC p{AP(q.at("x").scalar),AP(q.at("y").scalar)};aincidence(c,p);points.push_back(std::move(p));}require(points.size()==652&&primary.at("pool_size").integer()==652,"same frozen point population");
 std::map<std::string,std::map<int,int>> lines;AH hash;size_t pairs=0;
 for(size_t i=0;i<points.size();++i)for(size_t j=i+1;j<points.size();++j){AL line=athrough(points[i],points[j]);std::string key=akey(line);require(amod(line[0]*points[i].x+line[1]*points[i].y+line[2])==0&&amod(line[0]*points[j].x+line[1]*points[j].y+line[2])==0,"independent pair incidence");hash.add(i,j,key);auto&support=lines[key];support[int(i)]=1;support[int(j)]=1;++pairs;}
 auto coverage=[&](size_t n){require(2*n==points.size()*(points.size()-1),"independent complete pair coverage");};coverage(pairs);areject_partial([&]{coverage(pairs-1);});size_t pair_lines=lines.size(),flexes=0;
 for(size_t i=0;i<points.size();++i){const auto&p=points[i];AP fx=avalue(c,p,1,0),fy=avalue(c,p,0,1);require(fx!=0||fy!=0,"independent smooth tangent");AL line=anormalize({fx,fy,amod(-fx*p.x-fy*p.y)});auto polynomial=acontact(c,p,line);require(polynomial[0]==0&&polynomial[1]==0&&polynomial[4]!=0,"complete independent tangent restriction");int order=2;while(order<=4&&polynomial[size_t(order)]==0)++order;require(order<=4,"proper independent tangent contact");lines[akey(line)][int(i)]=order;if(order>=3)++flexes;}
 std::map<int,size_t> histogram;std::set<std::string> retained;
 for(const auto&[key,support]:lines){int degree=0;for(auto[i,m]:support){(void)i;degree+=m;}require(degree>=2&&degree<=4,"independent old degree bound");++histogram[degree];if(degree>=3)retained.insert(key);}
 std::string digest=hash.finish();require(primary.at("pair_line_stream_sha256").scalar==digest,"independent complete line-key digest");require(primary.at("unordered_pairs").integer()==int(pairs)&&primary.at("pair_lines").integer()==int(pair_lines)&&primary.at("all_lines").integer()==int(lines.size())&&primary.at("tangents").integer()==652&&primary.at("flex_tangents").integer()==int(flexes),"all independent line and contact counts match");ahistogram(primary.at("old_support_degree_histogram"),histogram);
 auto altered=primary.at("old_support_degree_histogram");auto&bin=altered.obj.begin()->second.scalar;bin=std::to_string(std::stoi(bin)+1);areject_partial([&]{ahistogram(altered,histogram);});
 AC changed=points[0];bool found_bad=false;for(int i=0;i<5;++i){changed.y=amod(changed.y+1);if(avalue(c,changed)!=0){found_bad=true;break;}}require(found_bad,"off-curve point control constructed");areject_partial([&]{aincidence(c,changed);});
 auto matrix=sc_read_matrix(tangents.at("matrix"));require(matrix.size()==108&&matrix[0].size()==429&&sc_rank(matrix)==108,"frozen full matrix");
 for(const auto&section:primary.at("sections").arr){AL line;require(section.at("line").arr.size()==3,"recorded line coefficient width");for(size_t i=0;i<3;++i)line[i]=AP(section.at("line").arr[i].scalar);require(anormalize(line)==line,"canonical recorded line");std::string key=akey(line);require(retained.erase(key)==1,"every retained line once");std::map<int,int> expected;for(const auto&entry:section.at("old_support").arr){require(entry.arr.size()==2&&expected.emplace(entry.arr[0].integer(),entry.arr[1].integer()).second,"unique retained support");}require(lines.at(key)==expected,"independent exact retained old support");
  SCRow row(429);const auto&stored=section.at("old_row").arr;require(stored.size()==225,"old row width");for(size_t i=0;i<225;++i)row[i]=SCBig(stored[i].scalar);matrix.push_back(std::move(row));
 }
 require(retained.empty(),"no omitted retained line");size_t degree3=histogram.count(3)?histogram.at(3):0;require(degree3==0&&primary.at("partial_sections").integer()==0&&primary.at("new_auxiliary_columns").integer()==0,"geometry-only null replay scope; positives require separate residual-map/KM audit");
 size_t full=sc_rank(matrix);SCMat aux;for(const auto&r:matrix)aux.emplace_back(r.begin()+225,r.end());size_t arank=sc_rank(aux);require(primary.at("full_rank").integer()==int(full)&&primary.at("auxiliary_rank").integer()==int(arank)&&primary.at("closed_rank").integer()==int(full-arank),"recorded scalar ranks recomputed");
 std::cout<<"{\"scope\":\"separate-GMP-partial-line-null-geometry-replay\",\"pool_size\":652,\"unordered_pairs\":"<<pairs<<",\"pair_lines\":"<<pair_lines<<",\"all_lines\":"<<lines.size()<<",\"flex_tangents\":"<<flexes<<",\"partial_sections\":"<<degree3<<",\"complete_line_stream_matches\":true,\"retained_supports_match\":true,\"controls_pass\":true,\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<full-arank<<",\"candidate_gain\":"<<full-arank-6<<",\"positive_gain_established\":false,\"standing\":\"producer-unverified; geometry audit does not re-prove the elliptic map\"}\n";return 0;
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
