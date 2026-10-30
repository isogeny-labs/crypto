// Complete signed TWO-summand row structure. Reuses JSON/file/hash only.
#include "json.hpp"
#include <functional>
#include <set>
using Row=std::vector<int>;using Matrix=std::vector<Row>;
int md(int x,int p){x%=p;return x<0?x+p:x;}
int inv(int x,int p){int a=1;for(int i=0;i<p-2;++i)a=md(a*x,p);return a;}
int rank_of(Matrix a,int p){int k=0;if(a.empty())return 0;for(size_t j=0;j<a[0].size();++j){int i=k;while(i<int(a.size())&&!md(a[i][j],p))++i;if(i==int(a.size()))continue;std::swap(a[i],a[k]);int z=inv(md(a[k][j],p),p);for(auto&v:a[k])v=md(v*z,p);for(int t=k+1;t<int(a.size());++t){z=a[t][j];for(size_t c=0;c<a[t].size();++c)a[t][c]=md(a[t][c]-z*a[k][c],p);}++k;}return k;}
std::string ns(const Row&a){std::string s;for(auto x:a){if(!s.empty())s+=',';s+=std::to_string(x);}return s;}
std::string matrix_bytes(const Matrix&a){std::string s="[";for(auto&r:a){if(s.size()>1)s+=',';s+='[';s+=ns(r);s+=']';}return s+"]";}
Matrix rows_for(const Row&base,int r,int m,int coef){std::set<Row>out;for(size_t i=0;i<base.size();++i)for(size_t j=0;j<base.size();++j)for(int si:{-1,1})for(int sj:{-1,1})if(md(si*base[i]+sj*base[j],r)==m){Row row(base.size());row[i]=md(row[i]+si,coef);row[j]=md(row[j]+sj,coef);out.insert(row);}return {out.begin(),out.end()};}
int pairs_for(const Row&base,int r,int m){Row endpoints;for(auto x:base){endpoints.push_back(x);endpoints.push_back(md(-x,r));}int n=0;for(size_t i=0;i<endpoints.size();++i)for(size_t j=i;j<endpoints.size();++j)n+=md(endpoints[i]+endpoints[j],r)==m;return n;}
Row ints(const Json&a){Row r;for(auto&x:a.items)r.push_back(int(x.integer()));return r;}
Json number(int n){Json j;j.kind=Json::Number;j.text=std::to_string(n);return j;}
Json string(const std::string&s){Json j;j.kind=Json::String;j.text=s;return j;}
std::string dump(const Json &j) {
  if (j.kind == Json::Null)
    return "null";
  if (j.kind == Json::Number || j.kind == Json::Bool)
    return j.text;
  if (j.kind == Json::String) {
    std::string s = "\"";
    for (unsigned char c : j.text) {
      if (c == '"' || c == '\\') {
        s += '\\';
        s += char(c);
      } else if (c == '\n')
        s += "\\n";
      else {
        require(c >= 32, "unserializable control");
        s += char(c);
      }
    }
    return s + '"';
  }
  std::string s = j.kind == Json::Array ? "[" : "{";
  bool first = true;
  if (j.kind == Json::Array)
    for (auto &x : j.items) {
      if (!first)
        s += ',';
      first = false;
      s += dump(x);
    }
  else
    for (auto &[k, v] : j.fields) {
      if (!first)
        s += ',';
      first = false;
      s += dump(string(k)) + ':' + dump(v);
    }
  return s + (j.kind == Json::Array ? "]" : "}");
}
int main(int argc,char**argv){try{if(argc==3 && std::string(argv[1])=="preflight"){auto a=rows_for({1},3,1,3);std::cout<<rank_of(a,3)<<"\n";return 0;}require(argc==3,"pair-rank ROOT FRESH_OUT");fs::path root=argv[1],out=argv[2];require(!fs::exists(out),"fresh output");fs::create_directories(out);std::ostringstream data,bindings;data<<"r\tb\tbase\trank_sum\tpair_sum\tranks\tpairs\trows_sha256\n";int cases=0,queries=0,defects=0;for(int r:{3,5,7,11,13,17,19,23,29,31})for(int b=1;b<=std::min(4,(r-1)/2);++b){std::function<void(int,Row)>visit=[&](int next,Row base){if(int(base.size())==b){Row ranks,pairs;std::string stream;int sr=0,se=0;for(int m=1;m<r;++m){auto rows=rows_for(base,r,m,r);int rk=rank_of(rows,r),e=pairs_for(base,r,m);ranks.push_back(rk);pairs.push_back(e);sr+=rk;se+=e;defects+=rk!=e;stream+=std::to_string(m)+":"+matrix_bytes(rows)+"\n";++queries;}data<<r<<'\t'<<b<<'\t'<<ns(base)<<'\t'<<sr<<'\t'<<se<<'\t'<<ns(ranks)<<'\t'<<ns(pairs)<<'\t'<<digest(stream)<<'\n';++cases;return;}for(int x=next;x<=(r-1)/2;++x){auto a=base;a.push_back(x);visit(x+1,a);}};visit(1,{});}
const std::string prefix="research/quadratic_twist_evidence_20261021/";auto input=root/prefix/"native/uniform-20261029/cases";auto index=parse(read(input/"index.json"));bindings<<"name\tinput_sha256\tr\tb\tbase_logs\trank_sum\trows_sha256\n";int arms=0,actual_queries=0;for(auto&pin:index.at("arms").items){auto name=pin.at("name").string(),bytes=read(input/(name+".json"));require(digest(bytes)==pin.at("sha256").string(),"frozen input SHA");auto arm=parse(bytes);auto base=ints(arm.at("base_logs"));int r=int(arm.at("r").integer()),sum=0;std::string stream;require(arm.at("queries").items.size()==size_t(r-1),"all queries");for(int m=1;m<r;++m){auto rows=rows_for(base,r,m,r);auto&q=arm.at("queries").items.at(size_t(m-1));require(q.at("m").integer()==m,"query order");require(matrix_bytes(rows)==dump(q.at("rows")),"actual frozen rows");sum+=rank_of(rows,r);stream+=std::to_string(m)+":"+matrix_bytes(rows)+"\n";++actual_queries;}require(sum==arm.at("uniform").at("rank_sum").integer(),"old rank binding");bindings<<name<<'\t'<<digest(bytes)<<'\t'<<r<<'\t'<<base.size()<<'\t'<<ns(base)<<'\t'<<sum<<'\t'<<digest(stream)<<'\n';++arms;}require(arms==96&&actual_queries==896,"actual scope");
std::string controls;for(auto cfg:std::vector<std::tuple<int,Row,int,int>>{{9,{1,2,4},3,101},{7,{1},2,2}}){auto[r,base,m,c]=cfg;auto rows=rows_for(base,r,m,c);int rk=rank_of(rows,c),count=pairs_for(base,r,m);require(rk<count,"positive domain control");controls+=std::to_string(r)+"\t"+std::to_string(c)+"\t"+ns(base)+"\t"+std::to_string(m)+"\t"+std::to_string(rk)+"\t"+std::to_string(count)+"\t"+matrix_bytes(rows)+"\n";}
std::ofstream(out/"cases.tsv")<<data.str();std::ofstream(out/"bindings.tsv")<<bindings.str();std::ofstream(out/"domain-controls.tsv")<<controls;Json summary;summary.kind=Json::Object;for(auto [key,n]:std::map<std::string,int>{{"cases",cases},{"queries",queries},{"rank_pair_defects",defects},{"actual_arms",arms},{"actual_queries",actual_queries}})summary.fields[key]=number(n);for(auto file:{"cases.tsv","bindings.tsv","domain-controls.tsv"})summary.fields[std::string(file)+"_sha256"]=string(digest(read(out/file)));for(auto file:{"native/pair-rank.cpp","native/pair-rank-audit/main.rs","lean-pair-rank/PairRank.lean","PAIR_RANK_20261029_PROTOCOL.md"})summary.fields[file]=string(digest(read(root/prefix/file)));summary.fields["uniform_index_sha256"]=string(digest(read(input/"index.json")));std::ofstream(out/"summary.json")<<dump(summary)<<'\n';std::cout<<dump(summary)<<'\n';}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
