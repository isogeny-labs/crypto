// Separate fixed-fixture review checker. No candidate source is included.
// Verifies retained records using extended-Euclid arithmetic and direct point sums.
#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>

void need(bool b,const std::string&s){if(!b)throw std::runtime_error(s);}
int residue(long long x,int p){int y=static_cast<int>(x%p);return y<0?y+p:y;}
int inverse(int x,int p){int a=residue(x,p),b=p,s=1,t=0;while(b){int q=a/b;int c=a-q*b;a=b;b=c;c=s-q*t;s=t;t=c;}need(a==1,"inverse");return residue(s,p);}
int power(int x,unsigned e,int p){int y=1;for(unsigned i=0;i<e;++i)y=residue(y*x,p);return y;}
struct Point{int x=0,y=0;bool inf=true;};
bool equal(Point a,Point b){return a.inf==b.inf&&(a.inf||(a.x==b.x&&a.y==b.y));}
bool on(Point a){return a.inf||(a.x>=0&&a.x<17&&a.y>=0&&a.y<17&&residue(a.y*a.y,17)==residue(a.x*a.x*a.x+2*a.x+2,17));}
Point add(Point a,Point b){need(on(a)&&on(b),"off curve");if(a.inf)return b;if(b.inf)return a;if(a.x==b.x&&residue(a.y+b.y,17)==0)return {};int s=a.x==b.x?residue((3*a.x*a.x+2)*inverse(2*a.y,17),17):residue((b.y-a.y)*inverse(b.x-a.x,17),17);int x=residue(s*s-a.x-b.x,17);return{x,residue(s*(a.x-x)-a.y,17),false};}
// Deliberately simple fixed-unit scalar multiplication, not a logarithm search.
Point multiple(Point a,int n){Point r;for(int i=0;i<residue(n,19);++i)r=add(r,a);return r;}
std::vector<std::string> split(const std::string&s,char c){std::vector<std::string>v;std::stringstream f(s);std::string x;while(std::getline(f,x,c))v.push_back(x);return v;}
using Row=std::map<std::string,std::string>;
std::vector<Row> read(const std::string&p){std::ifstream f(p);need(bool(f),"open "+p);std::string s;need(bool(std::getline(f,s)),"header");auto h=split(s,'\t');std::vector<Row>out;while(std::getline(f,s)){auto v=split(s,'\t');need(v.size()==h.size(),"TSV width "+p);Row r;for(std::size_t i=0;i<h.size();++i)r[h[i]]=v[i];out.push_back(r);}return out;}
int val(const Row&r,const std::string&k){return std::stoi(r.at(k));}
Point point(const Row&r,const std::string&x,const std::string&y){return{val(r,x),val(r,y),false};}
std::string name(Point p){return std::to_string(p.x)+":"+std::to_string(p.y);}
std::string model(std::vector<Point>v){std::sort(v.begin(),v.end(),[](Point a,Point b){return std::tie(a.x,a.y)<std::tie(b.x,b.y);});std::string z;for(auto p:v){if(!z.empty())z+=';';z+=name(p);}return z;}
std::vector<Point> parse_model(const std::string&s){std::vector<Point>v;for(const auto&w:split(s,';')){auto xy=split(w,':');need(xy.size()==2,"model syntax");v.push_back({std::stoi(xy[0]),std::stoi(xy[1]),false});}need(v.size()==3,"model arity");return v;}
int rank(std::vector<std::array<int,3>> C){int r=0;for(int c=0;c<3&&r<static_cast<int>(C.size());++c){int k=r;while(k<static_cast<int>(C.size())&&!C[k][c])++k;if(k==static_cast<int>(C.size()))continue;std::swap(C[k],C[r]);int z=inverse(C[r][c],19);for(int j=c;j<3;++j)C[r][j]=residue(C[r][j]*z,19);for(std::size_t i=0;i<C.size();++i)if(static_cast<int>(i)!=r){z=C[i][c];for(int j=c;j<3;++j)C[i][j]=residue(C[i][j]-z*C[r][j],19);}++r;}return r;}
int main(int argc,char**argv){try{
    need(argc==2,"usage checks-directory");std::string p=argv[1];Point P{5,1,false};std::vector<Point>all;
    for(int x=0;x<17;++x)for(int y=0;y<17;++y)if(on({x,y,false}))all.push_back({x,y,false});
    need(all.size()==18,"point count");
    auto cases=read(p+"/checks.tsv"),models=read(p+"/candidate-models.tsv"),oracle=read(p+"/oracle-models.tsv");
    std::map<int,std::set<std::string>> retained,retained_oracle;
    for(const auto&r:models)need(retained[val(r,"case")].insert(r.at("model")).second,"duplicate retained model");
    for(const auto&r:oracle)need(retained_oracle[val(r,"case")].insert(r.at("model")).second,"duplicate oracle model");
    need(retained==retained_oracle,"retained models differ");std::size_t complete_models=0;int sat=0,unsat=0;
    for(const auto&r:cases){int id=val(r,"case"),t=val(r,"t"),c=val(r,"c");Point R=point(r,"target_x","target_y");std::vector<Point>b;std::set<int>xs;
        for(Point q:all)if(q.x!=R.x&&power(q.x,1U<<t,17)==c){b.push_back(q);xs.insert(q.x);}
        std::set<std::string>expected;
        for(std::size_t i=0;i<b.size();++i)for(std::size_t j=i+1;j<b.size();++j)for(std::size_t k=j+1;k<b.size();++k)if(b[i].x!=b[j].x&&b[i].x!=b[k].x&&b[j].x!=b[k].x&&equal(add(add(b[i],b[j]),b[k]),R))expected.insert(model({b[i],b[j],b[k]}));
        need(expected==retained[id],"complete model set case "+std::to_string(id));need(expected.size()==static_cast<std::size_t>(val(r,"models")),"model count");need(xs.size()==static_cast<std::size_t>(val(r,"usable_abscissas_after_target_exclusion")),"domain count");need(val(r,"coefficient_pairs")==272,"coefficient count");need(r.at("status")== (expected.empty()?"UNSAT":"SAT"),"status");complete_models+=expected.size();if(expected.empty())++unsat;else ++sat;
    }
    auto checks=read(p+"/direct-checks.tsv"),rels=read(p+"/direct-relations.tsv"),kernels=read(p+"/direct-kernels.tsv"),cli=read(p+"/direct-cli.tsv");
    using Key=std::tuple<int,int,int>;std::map<Key,std::vector<Row>> batches;std::array<Point,3>base{{{9,1,false},{13,7,false},{16,4,false}}};
    for(const auto&r:rels){Point Q=point(r,"Qx","Qy"),R=point(r,"Rx","Ry");need(equal(add(multiple(P,val(r,"a")),multiple(Q,val(r,"b"))),R),"mixed target relation");auto pts=parse_model(r.at("model"));Point sum;std::array<int,3>C{};std::set<int>xs;
        for(Point q:pts){need(on(q)&&q.x!=R.x&&power(q.x,8,17)==1&&xs.insert(q.x).second,"relation point chart");sum=add(sum,q);int index=-1;for(int i=0;i<3;++i)if(base[i].x==q.x)index=i;need(index>=0,"base binding");C[index]=q.y==base[index].y?1:18;need(q.y==base[index].y||q.y==residue(-base[index].y,17),"base sign");}
        need(equal(sum,R),"point model sum");for(int i=0;i<3;++i)need(C[i]==val(r,"c"+std::to_string(i)),"coefficient sign");auto&v=batches[{Q.x,Q.y,val(r,"batch")}];need(v.empty()||val(v.back(),"attempt")<val(r,"attempt"),"attempt order");v.push_back(r);
    }
    int low_rank=0;std::set<Key>seen;
    for(const auto&r:kernels){Point Q=point(r,"Qx","Qy");Key key{Q.x,Q.y,val(r,"batch")};need(seen.insert(key).second,"duplicate kernel");const auto&b=batches.at(key);need(b.size()==4,"batch size");std::vector<std::array<int,3>>C;std::array<int,4>w{};bool nz=false;int alpha=0,beta=0;
        for(int i=0;i<4;++i){w[i]=val(r,"w"+std::to_string(i));nz=nz||w[i]!=0;C.push_back({val(b[i],"c0"),val(b[i],"c1"),val(b[i],"c2")});alpha=residue(alpha+w[i]*val(b[i],"a"),19);beta=residue(beta+w[i]*val(b[i],"b"),19);}
        need(nz,"zero kernel");for(int j=0;j<3;++j){int sum=0;for(int i=0;i<4;++i)sum=residue(sum+w[i]*C[i][j],19);need(sum==0,"kernel equation");}need(rank(C)==val(r,"C_rank"),"matrix rank");need(alpha==val(r,"alpha")&&beta==val(r,"beta"),"alpha beta");if(val(r,"C_rank")<3)++low_rank;
        if(val(r,"retry"))need(alpha==0&&beta==0,"retry denominator");else{need(beta!=0,"nonretry denominator");int k=residue(-alpha*inverse(beta,19),19);need(equal(multiple(P,k),Q),"recovered scalar group check");}
    }
    need(seen.size()==batches.size(),"missing kernel");std::uint64_t attempts=0,miss=0,identity=0,pdp=0,pairs=0,accepted=0;
    for(const auto&r:checks){Point Q=point(r,"Qx","Qy");need(equal(multiple(P,val(r,"fixture_expected")),Q)&&val(r,"recovered")==val(r,"fixture_expected"),"internal fixture answer");need(val(r,"B")==3&&val(r,"setup_curve_pairs")==289&&val(r,"setup_membership_tests")==9,"cold setup");need(val(r,"attempts")==val(r,"identity_misses")+val(r,"pdp_misses")+val(r,"accepted_rows"),"attempt partition");need(val(r,"pdp_calls")==val(r,"pdp_misses")+val(r,"accepted_rows")&&val(r,"coefficient_pairs")==272*val(r,"pdp_calls"),"call work");need(val(r,"accepted_rows")==4*val(r,"batches"),"batch row count");need(val(r,"accepted_rows")==4,"retained current single batch");auto key=Key{Q.x,Q.y,1};need(batches.at(key).size()==4&&val(batches.at(key).back(),"attempt")<=val(r,"attempts"),"trace count");attempts+=val(r,"attempts");miss+=val(r,"pdp_misses");identity+=val(r,"identity_misses");pdp+=val(r,"pdp_calls");pairs+=val(r,"coefficient_pairs");accepted+=val(r,"accepted_rows");}
    for(const auto&r:cli){Point Q=point(r,"Qx","Qy");need(val(r,"recovered")==val(r,"fixture_expected")&&equal(multiple(P,val(r,"recovered")),Q),"external CLI answer");need(r.at("verified")=="verified"&&val(r,"coefficient_pairs")==272*val(r,"pdp_calls"),"external CLI work");}
    need(cases.size()==90&&complete_models==400&&sat==42&&unsat==48,"fixed model totals");need(rels.size()==72&&kernels.size()==18&&checks.size()==18&&cli.size()==18,"fixed direct totals");
    std::cout<<"PASS fixed model census: cases="<<cases.size()<<" models="<<complete_models<<" SAT="<<sat<<" UNSAT="<<unsat<<"\nPASS mixed-target relations="<<rels.size()<<" kernels="<<kernels.size()<<" low_rank="<<low_rank<<" internal_DLP="<<checks.size()<<" CLI_DLP="<<cli.size()<<"\nPASS retained counters: attempts="<<attempts<<" identity="<<identity<<" PDP_misses="<<miss<<" PDP_calls="<<pdp<<" pairs="<<pairs<<" accepted="<<accepted<<"\nNo candidate source included; no timings, scaling, unknown-solver rerun or logarithm search.\n";
    return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
