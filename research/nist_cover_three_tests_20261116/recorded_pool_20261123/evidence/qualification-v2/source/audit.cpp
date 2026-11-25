#include "rank_arithmetic.hpp"
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <memory>
#include <set>

namespace audit {
using Big=mpz_class;
using Pair=std::array<int,2>;
using Quad=std::array<int,4>;
using Line=std::array<Big,3>;
const Big p("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFF",16);
const Big b("64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1",16);
Big mod(Big x){x%=p;if(x<0)x+=p;return x;}
Big inv(Big x){Big y; x=mod(x);require(mpz_invert(y.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t())!=0,"Fp inverse");return y;}
Big power(Big x,int n){Big y=1;while(n--){y=mod(y*x);}return y;}
struct Point {bool zero=true;Big x=0,y=0;};
bool operator==(const Point& x,const Point& y){return x.zero==y.zero&&(x.zero||(x.x==y.x&&x.y==y.y));}
Point negative(Point x){if(!x.zero)x.y=mod(-x.y);return x;}
bool on_curve(const Point& x){return x.zero||(x.x>=0&&x.x<p&&x.y>=0&&x.y<p&&mod(x.y*x.y)==mod(x.x*x.x*x.x-3*x.x+b));}
Point plus(const Point& x,const Point& y){
    if(x.zero){return y;}
    if(y.zero){return x;}
    Big slope;
    if(x.x==y.x){if(mod(x.y+y.y)==0)return {};require(x.y==y.y,"equal-x elliptic inputs");slope=mod((3*x.x*x.x-3)*inv(2*x.y));}
    else slope=mod((y.y-x.y)*inv(y.x-x.x));
    Big xx=mod(slope*slope-x.x-y.x);return {false,xx,mod(slope*(x.x-xx)-x.y)};
}
Point read_point(const Json& j){if(j.at("infinity").scalar=="true")return {};require(j.at("infinity").scalar=="false","point infinity flag");return {false,Big(j.at("x").scalar),Big(j.at("y").scalar)};}
std::string key(const Point& q){
    if(q.zero)return std::string(1,'\0');
    std::array<unsigned char,24> bytes{};size_t count=0;mpz_export(bytes.data(),&count,1,1,1,0,q.x.get_mpz_t());require(count<=24,"point width");
    std::string out(25,'\0');out[0]=char(mpz_odd_p(q.y.get_mpz_t())?3:2);
    for(size_t i=0;i<count;++i){out[25-count+i]=char(bytes[i]);}
    return out;
}
struct DeletePoint{void operator()(EC_POINT* q)const{EC_POINT_free(q);}};
struct DeleteBN{void operator()(BIGNUM* q)const{BN_free(q);}};
using SSLPoint=std::unique_ptr<EC_POINT,DeletePoint>;
using BN=std::unique_ptr<BIGNUM,DeleteBN>;
BN bn(const Big& x){BIGNUM* q=nullptr;require(BN_dec2bn(&q,x.get_str().c_str())!=0,"BN conversion");return BN(q);}
struct SSL {
    std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)> group{EC_GROUP_new_by_curve_name(NID_X9_62_prime192v1),EC_GROUP_free};
    std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)> context{BN_CTX_new(),BN_CTX_free};
    SSL(){require(bool(group)&&bool(context),"OpenSSL setup");BN pp(BN_new()),aa(BN_new()),bb(BN_new()),nn(BN_new()),hh(BN_new());
        require(EC_GROUP_get_curve(group.get(),pp.get(),aa.get(),bb.get(),context.get())==1&&EC_GROUP_get_order(group.get(),nn.get(),context.get())==1&&EC_GROUP_get_cofactor(group.get(),hh.get(),context.get())==1,"named group parameters");
        auto check=[&](const BN& got,const Big& expected){auto value=bn(expected);require(BN_cmp(got.get(),value.get())==0,"exact P192 parameters");};
        check(pp,p);check(aa,mod(-3));check(bb,b);check(nn,subgroup_order);require(BN_is_one(hh.get()),"P192 cofactor1");}
    SSLPoint make(const Point& q)const{SSLPoint out(EC_POINT_new(group.get()));require(bool(out),"EC point allocation");
        if(q.zero)require(EC_POINT_set_to_infinity(group.get(),out.get())==1,"identity setup");
        else{auto x=bn(q.x),y=bn(q.y);require(EC_POINT_set_affine_coordinates(group.get(),out.get(),x.get(),y.get(),context.get())==1,"SSL affine point");}
        require(EC_POINT_is_on_curve(group.get(),out.get(),context.get())==1,"SSL curve equation");return out;}
    std::string encode(const EC_POINT* q)const{size_t n=EC_POINT_point2oct(group.get(),q,POINT_CONVERSION_COMPRESSED,nullptr,0,context.get());require(n==1||n==25,"compressed point length");std::string out(n,'\0');require(EC_POINT_point2oct(group.get(),q,POINT_CONVERSION_COMPRESSED,reinterpret_cast<unsigned char*>(out.data()),n,context.get())==n,"compressed point encoding");return out;}
};
struct Item{Pair indices;std::string sum,complement;};
using Relations=std::set<Quad>;
Quad quartet(Pair a,Pair c){Quad q{a[0],a[1],c[0],c[1]};std::sort(q.begin(),q.end());return q;}
Quad pair_relation(Pair a,Pair c){if(c<a)std::swap(a,c);return {a[0],a[1],c[0],c[1]};}
std::string fingerprint(const std::vector<Item>& rows){std::string stream="P192 recorded-pool pairs/v1\n";for(const auto& r:rows){for(int index:r.indices)for(int k=0;k<4;++k)stream+=char((unsigned(index)>>(8*k))&255U);stream+=char(r.sum.size());stream+=r.sum;stream+=char(r.complement.size());stream+=r.complement;}return ::hex(::hash(stream));}
void bounded(const Relations& a,const Relations& c){require(a.size()<=4096&&c.size()<=4096,"candidate cap exceeded; not exhaustive");}
std::pair<Relations,Relations> grouped(const std::vector<Item>& rows){
    std::map<std::string,std::vector<Pair>> buckets;for(const auto& r:rows)buckets[r.sum].push_back(r.indices);Relations pairs,quads;
    for(const auto& entry:buckets){const auto& v=entry.second;for(size_t i=0;i<v.size();++i)for(size_t j=i+1;j<v.size();++j){pairs.insert(pair_relation(v[i],v[j]));bounded(pairs,quads);}}
    for(const auto& r:rows){auto hit=buckets.find(r.complement);if(hit!=buckets.end())for(auto v:hit->second){quads.insert(quartet(r.indices,v));bounded(pairs,quads);}}
    return {pairs,quads};
}
std::pair<Relations,Relations> sorted_join(const std::vector<Item>& rows){
    using Entry=std::pair<std::string,Pair>;std::vector<Entry> direct,complements;for(const auto& r:rows){direct.emplace_back(r.sum,r.indices);complements.emplace_back(r.complement,r.indices);}std::sort(direct.begin(),direct.end());std::sort(complements.begin(),complements.end());Relations pairs,quads;
    for(size_t i=0;i<direct.size();){size_t e=i+1;while(e<direct.size()&&direct[e].first==direct[i].first)++e;
        for(size_t a=i;a<e;++a){for(size_t c=a+1;c<e;++c){pairs.insert(pair_relation(direct[a].second,direct[c].second));bounded(pairs,quads);}}
        i=e;}
    size_t i=0,j=0;while(i<direct.size()&&j<complements.size()){
        if(direct[i].first<complements[j].first){++i;continue;}if(complements[j].first<direct[i].first){++j;continue;}
        size_t ie=i+1,je=j+1;while(ie<direct.size()&&direct[ie].first==direct[i].first)++ie;while(je<complements.size()&&complements[je].first==complements[j].first)++je;
        for(size_t a=i;a<ie;++a){for(size_t c=j;c<je;++c){quads.insert(quartet(direct[a].second,complements[c].second));bounded(pairs,quads);}}
        i=ie;j=je;}
    return {pairs,quads};
}
struct CP{Big x,y;};
struct Term{int x,y;Big c;};
Big evaluate(const std::vector<Term>& terms,const CP& q,int dx=0,int dy=0){Big out=0;for(const auto& t:terms)if(t.x>=dx&&t.y>=dy){Big c=t.c;for(int i=0;i<dx;++i)c*=t.x-i;for(int i=0;i<dy;++i)c*=t.y-i;out=mod(out+c*power(q.x,t.x-dx)*power(q.y,t.y-dy));}return out;}
Line normalize(Line l){require(l[0]!=0||l[1]!=0,"affine line normal");Big scale=inv(l[0]!=0?l[0]:l[1]);for(auto& c:l)c=mod(c*scale);return l;}
Line through(const CP& a,const CP& c){return normalize({mod(a.y-c.y),mod(c.x-a.x),mod(a.x*c.y-c.x*a.y)});}
using Poly=std::vector<Big>;
void trim(Poly& a){while(a.size()>1&&a.back()==0)a.pop_back();}
Poly product(const Poly& a,const Poly& c){Poly out(a.size()+c.size()-1);for(size_t i=0;i<a.size();++i)for(size_t j=0;j<c.size();++j)out[i+j]=mod(out[i+j]+a[i]*c[j]);trim(out);return out;}
Poly ppow(Poly x,int n){Poly out{1};while(n--)out=product(out,x);return out;}
Big peval(const Poly& a,const Big& t){Big out=0;for(auto i=a.rbegin();i!=a.rend();++i)out=mod(out*t+*i);return out;}
Poly divide(const Poly& a,const Big& t){require(a.size()>1,"root division degree");Poly q(a.size()-1);q.back()=a.back();for(size_t k=q.size()-1;k>0;--k)q[k-1]=mod(a[k]+t*q[k]);require(mod(a[0]+t*q[0])==0,"known-root exact division");trim(q);return q;}
struct Candidate{std::set<int> support;bool tangent=false;};
struct Section{Line line;Poly restriction;};
std::map<Quad,Section> canonical(const std::vector<CP>& points,const std::vector<Term>& terms,size_t& line_count,size_t& inspected){
    std::map<Line,Candidate> lines;
    for(size_t i=0;i<points.size();++i)for(size_t j=i+1;j<points.size();++j){auto& c=lines[through(points[i],points[j])];c.support.insert(int(i));c.support.insert(int(j));}
    for(size_t i=0;i<points.size();++i){Big a=evaluate(terms,points[i],1,0),b0=evaluate(terms,points[i],0,1);auto& c=lines[normalize({a,b0,mod(-a*points[i].x-b0*points[i].y)})];c.tangent=true;c.support.insert(int(i));}
    line_count=lines.size();std::map<Quad,Section> out;inspected=0;
    for(const auto& entry:lines){const auto& l=entry.first;const auto& c=entry.second;require(c.support.size()<=4,"more than four collinear recorded points");if(c.support.size()<4&&!c.tangent)continue;++inspected;
        const auto& origin=points[size_t(*c.support.begin())];Poly f(5);for(const auto& t:terms){auto term=product(ppow({origin.x,l[1]},t.x),ppow({origin.y,mod(-l[0])},t.y));for(size_t k=0;k<term.size();++k)f[k]=mod(f[k]+t.c*term[k]);}trim(f);require(!(f.size()==1&&f[0]==0),"line is not a quartic component");if(f.size()!=5)continue;
        Poly rest=f;std::vector<int> support;std::map<int,Big> roots;
        for(int index:c.support){const auto& q=points[size_t(index)];Big t=l[1]!=0?mod((q.x-origin.x)*inv(l[1])):mod((q.y-origin.y)*inv(-l[0]));roots.emplace(index,t);
            while(rest.size()>1&&peval(rest,t)==0){support.push_back(index);rest=divide(rest,t);}}
        if(rest.size()!=1||support.size()!=4){continue;}
        Quad q{};std::copy(support.begin(),support.end(),q.begin());std::sort(q.begin(),q.end());Poly expanded{f.back()};for(int index:q)expanded=product(expanded,{mod(-roots.at(index)),1});require(expanded==f,"complete section product reconstruction");require(out.emplace(q,Section{l,f}).second,"unique canonical line");}
    return out;
}
void emit_quad(std::ostream& out,const Quad& q){out<<'[';for(size_t i=0;i<4;++i)out<<(i?",":"")<<q[i];out<<']';}
void emit_row(std::ostream& out,const Row& row){out<<'[';bool first=true;for(size_t i=0;i<row.size();++i)if(row[i]!=0){if(!first)out<<',';first=false;out<<'['<<i<<",\""<<row[i]<<"\"]";}out<<']';}
void emit_poly(std::ostream& out,const Poly& a){out<<'[';for(size_t i=0;i<a.size();++i){if(i)out<<',';out<<'"'<<a[i]<<'"';}out<<']';}
Point row_sum(const Row& row,const std::vector<Point>& base){Point out;for(size_t j=0;j<row.size();++j){require(row[j]>=-8&&row[j]<=8,"small audited coefficients");long n=row[j].get_si();Point q=n<0?negative(base[j]):base[j];if(n<0)n=-n;while(n--)out=plus(out,q);}return out;}
int run(int argc,char** argv){
    require(argc==4,"usage: audit FROZEN.json MODEL.json NEW-output-directory");auto data=read_json(argv[1]),model=read_json(argv[2]);require(model.at("kernel").integer()==3,"actual kernel3");
    std::vector<Term> terms;for(const auto& t:model.at("terms").arr){auto e=t.at("exponents");require(e.arr.size()==3&&e.arr[0].integer()+e.arr[1].integer()+e.arr[2].integer()==4,"homogeneous quartic term");terms.push_back({e.arr[0].integer(),e.arr[1].integer(),Big(t.at("coefficient").scalar)});}
    std::vector<CP> points;for(const auto& q:data.at("pool").at("points").arr){CP point{Big(q.at("x").scalar),Big(q.at("y").scalar)};require(evaluate(terms,point)==0,"recorded quartic equation");points.push_back(point);}require(points.size()==226,"frozen point count");
    std::vector<Point> images,base;std::vector<Pair> projection;for(const auto& q:data.at("projected_base").arr){Point v=read_point(q);require(!v.zero&&on_curve(v)&&v.y<=p-v.y,"canonical projected base");base.push_back(v);}require(base.size()==225,"frozen base count");
    for(const auto& q:data.at("images").arr){Point v=read_point(q.at("point"));require(on_curve(v),"GMP recorded elliptic equation");int index=q.at("projected_index").integer(),sign=q.at("sign").integer();if(index<0)require(index==-1&&sign==0&&v.zero,"identity projection");else{require(size_t(index)<base.size()&&(sign==1||sign==-1),"projection index/sign");require(v==(sign==1?base[size_t(index)]:negative(base[size_t(index)])),"exact projection binding");}images.push_back(v);projection.push_back({index,sign});}require(images.size()==points.size(),"point/image count");
    Quad reference{};const auto& ref=data.at("pool").at("split_fibers").arr[0].at("point_indices").arr;require(ref.size()==4,"reference support");for(size_t i=0;i<4;++i)reference[i]=ref[i].integer();std::sort(reference.begin(),reference.end());
    Point kappa;for(int i:reference)kappa=plus(kappa,images.at(size_t(i)));SSL ssl;std::vector<SSLPoint> ss;for(const auto& q:images)ss.push_back(ssl.make(q));auto sk=ssl.make({});for(int i:reference)require(EC_POINT_add(ssl.group.get(),sk.get(),sk.get(),ss.at(size_t(i)).get(),ssl.context.get())==1,"SSL reference sum");require(key(kappa)==ssl.encode(sk.get()),"reference arithmetic cross-check");
    std::vector<Item> first,second;auto sum=ssl.make({}),complement=ssl.make({});
    for(size_t i=0;i<images.size();++i)for(size_t j=i;j<images.size();++j){
        require(EC_POINT_add(ssl.group.get(),sum.get(),ss[i].get(),ss[j].get(),ssl.context.get())==1,"SSL pair addition");require(EC_POINT_copy(complement.get(),sum.get())==1&&EC_POINT_invert(ssl.group.get(),complement.get(),ssl.context.get())==1&&EC_POINT_add(ssl.group.get(),complement.get(),sk.get(),complement.get(),ssl.context.get())==1,"SSL complement");
        Pair indices{int(i),int(j)};first.push_back({indices,ssl.encode(sum.get()),ssl.encode(complement.get())});auto q=plus(images[i],images[j]);second.push_back({indices,key(q),key(plus(kappa,negative(q)))});}
    const size_t expected=points.size()*(points.size()+1)/2;require(first.size()==expected&&second.size()==expected,"complete pair enumeration");const auto digest=fingerprint(first);require(digest==fingerprint(second),"all independent pair fingerprints");auto found=grouped(first),checked=sorted_join(second);require(found==checked,"independent exhaustive joins");
    size_t line_count=0,inspected=0;auto sections=canonical(points,terms,line_count,inspected);for(const auto& entry:sections)require(found.second.count(entry.first),"canonical divisor norm-to-reference");for(const auto& f:data.at("pool").at("split_fibers").arr){Quad q{};const auto& a=f.at("point_indices").arr;for(size_t i=0;i<4;++i)q[i]=a[i].integer();std::sort(q.begin(),q.end());require(sections.count(q),"all frozen vertical fibers retained");}
    Matrix old;for(const auto& sparse:data.at("rows").arr){Row row(base.size());for(const auto& item:sparse.arr)row.at(size_t(item.arr[0].integer()))+=Big(item.arr[1].scalar);require(row_sum(row,base).zero,"old row sum");old.push_back(row);}require(matrix_rank(old)==6,"original coefficient rank");
    auto row_for=[&](const std::vector<int>& positive,const std::vector<int>& negative_indices){Row row(base.size());
        for(int i:positive){auto image=projection.at(size_t(i));if(image[0]>=0)row[size_t(image[0])]+=image[1];}
        for(int i:negative_indices){auto image=projection.at(size_t(i));if(image[0]>=0)row[size_t(image[0])]-=image[1];}
        require(row_sum(row,base).zero,"audited elliptic row sum");return row;};
    std::vector<int> ref_indices(reference.begin(),reference.end());Matrix principal=old,all=old;std::map<Quad,Row> quartet_rows,pair_rows;
    for(const auto& q:found.second){auto row=row_for(std::vector<int>(q.begin(),q.end()),ref_indices);quartet_rows.emplace(q,row);all.push_back(row);if(sections.count(q))principal.push_back(row);}
    for(const auto& q:found.first){auto row=row_for({q[0],q[1]},{q[2],q[3]});pair_rows.emplace(q,row);all.push_back(row);}
    int controls=0,inconclusive=0;CP mutation=points[0];mutation.y=mod(mutation.y+1);if(evaluate(terms,mutation)!=0){rejects([&]{require(evaluate(terms,mutation)==0,"changed quartic coordinate");});++controls;}else ++inconclusive;
    rejects([&]{require(on_curve({false,0,0}),"off-curve elliptic point");});++controls;auto missing=first;missing.pop_back();rejects([&]{require(missing.size()==expected&&fingerprint(missing)==fingerprint(second),"missing pair");});++controls;
    Quad bad_reference=reference;int outside=0;while(std::find(reference.begin(),reference.end(),outside)!=reference.end())++outside;bad_reference[0]=outside;std::sort(bad_reference.begin(),bad_reference.end());rejects([&]{require(sections.count(bad_reference),"changed reference divisor");});++controls;
    auto bad_row=old.at(0);auto c=std::find_if(bad_row.begin(),bad_row.end(),[](const Big& x){return x!=0;});require(c!=bad_row.end(),"nonzero row control");++*c;rejects([&]{require(row_sum(bad_row,base).zero,"changed row coefficient");});++controls;
    require(found.first.size()+found.second.size()<=256,"target-check cap; preserve bounded failure");
    std::ofstream out(std::string(argv[3])+"/result.json");require(bool(out),"new result file");out<<"{\"scope\":\"frozen-recorded-pool-only\",\"standing\":\"producer-unverified\",\"point_count\":"<<points.size()<<",\"projected_base_size\":"<<base.size()<<",\"unordered_pairs_with_diagonals\":"<<expected<<",\"pair_fingerprint_sha256\":\""<<digest<<"\",\"independent_pair_streams_match\":true,\"independent_join_sets_match\":true,\"candidate_lines\":"<<line_count<<",\"lines_restricted\":"<<inspected<<",\"baseline_rank\":6,\"principal_rank\":"<<matrix_rank(principal)<<",\"all_norm_rank\":"<<matrix_rank(all)<<",\"controls_rejected\":"<<controls<<",\"controls_inconclusive\":"<<inconclusive<<",\"new_curve_points\":0,\"reference\":";emit_quad(out,reference);
    out<<",\"canonical_sections\":[";bool comma=false;for(const auto& entry:sections){if(comma)out<<',';comma=true;out<<"{\"indices\":";emit_quad(out,entry.first);out<<",\"line\":";emit_poly(out,Poly(entry.second.line.begin(),entry.second.line.end()));out<<",\"restricted_coefficients\":";emit_poly(out,entry.second.restriction);out<<'}';}
    out<<"],\"norm_quartets\":[";comma=false;for(const auto& q:found.second){if(comma)out<<',';comma=true;out<<"{\"indices\":";emit_quad(out,q);out<<",\"canonical\":"<<(sections.count(q)?"true":"false")<<",\"row\":";emit_row(out,quartet_rows.at(q));out<<'}';}
    out<<"],\"pair_relations\":[";comma=false;for(const auto& q:found.first){if(comma)out<<',';comma=true;out<<"{\"indices\":";emit_quad(out,q);out<<",\"row\":";emit_row(out,pair_rows.at(q));out<<'}';}out<<"],\"corrupted_reference\":";emit_quad(out,bad_reference);out<<"}\n";require(bool(out),"write complete result");
    auto u=var("u"),r1=var("r1"),r2=var("r2"),r3=var("r3"),r4=var("r4");
    Expr symmetric2=add({mul({r1,r2}),mul({r1,r3}),mul({r1,r4}),mul({r2,r3}),mul({r2,r4}),mul({r3,r4})});Expr symmetric3=add({mul({r1,r2,r3}),mul({r1,r2,r4}),mul({r1,r3,r4}),mul({r2,r3,r4})});
    issue(std::string(argv[3])+"/quartic-product.identity.json","Generic quartic root-product expansion",{"u","r1","r2","r3","r4"},mul({sub(u,r1),sub(u,r2),sub(u,r3),sub(u,r4)}),add({::power(u,4),neg(mul({add({r1,r2,r3,r4}),::power(u,3)})),mul({symmetric2,::power(u,2)}),neg(mul({symmetric3,u})),mul({r1,r2,r3,r4})}),4);
    std::cout<<"recorded pool audit complete\n";return 0;
}
}
int main(int argc,char** argv){try{return audit::run(argc,argv);}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
