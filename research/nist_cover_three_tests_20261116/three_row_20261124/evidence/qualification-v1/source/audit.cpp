#include "two_row_arithmetic.hpp"
#include <openssl/evp.h>

namespace circuits {
using namespace audit;
using two_row::scaled;
using two_row::Table;
using two_row::fresh_write;
struct Support {Row row;char type;};
Big minor(const Row& a,const Row& b0,size_t i,size_t j) {return a[i]*b0[j]-a[j]*b0[i];}
Big determinant(const Matrix& m) {return m[0][0]*minor(m[1],m[2],1,2)-m[0][1]*minor(m[1],m[2],0,2)+m[0][2]*minor(m[1],m[2],0,1);}
Row primitive(const Matrix& m) {
    Row c;
    for(size_t x=0;x<3&&c.empty();++x) for(size_t y=x+1;y<3&&c.empty();++y)
        if(minor(m[0],m[1],x,y)!=0) c={minor(m[1],m[2],x,y),minor(m[2],m[0],x,y),minor(m[0],m[1],x,y)};
    require(c.size()==3,"nonzero minor for coefficient kernel");
    Big g=0;for(const auto& v:c) mpz_gcd(g.get_mpz_t(),g.get_mpz_t(),v.get_mpz_t());
    require(g!=0,"primitive coefficient gcd");for(auto& v:c) v/=g;
    if(c[0]<0) for(auto& v:c) v=-v;
    return c;
}
void zero_equation(const Matrix& m,const Row& c) {
    require(c.size()==3,"kernel length");
    for(size_t j=0;j<3;++j) {Big total=0;for(size_t i=0;i<3;++i) total+=c[i]*m[i][j];require(total==0,"exact integer kernel equation");}
}
std::string family(const Row& c,const std::array<char,3>& types) {
    Row magnitudes,old;
    for(size_t i=0;i<3;++i) {Big v=abs(c[i]);require(v!=0,"minimal nonzero coefficients");magnitudes.push_back(v);if(types[i]=='A') old.push_back(v);}
    std::sort(magnitudes.begin(),magnitudes.end());std::sort(old.begin(),old.end());
    if(magnitudes==Row{1,1,1}) {require(old.size()<=2&&std::all_of(old.begin(),old.end(),[](const Big& v){return v==1;}),"U old residual weights");return "U";}
    require(old.empty()||old==Row{2},"V/W old residual weights");
    if(magnitudes==Row{1,1,2}) return "V";
    require(magnitudes==Row{1,2,2},"complete proposed coefficient classes");return "W";
}
int templates(const std::string& outdir) {
    std::vector<Support> catalog;
    for(char type:{'A','B'}) for(size_t j=0;j<3;++j) {Row r(3);r[j]=type=='A'?1:2;catalog.push_back({r,type});}
    for(size_t j=0;j<3;++j) for(size_t k=j+1;k<3;++k) for(int s:{1,-1}) {Row r(3);r[j]=1;r[k]=s;catalog.push_back({r,'C'});}
    require(catalog.size()==12&&subgroup_order>32,"catalog and characteristic scope");
    std::ostringstream records;
    size_t examined=0,pair_excluded=0,full_rank=0,minimal=0;
    std::map<std::string,size_t> counts;
    std::map<std::array<int,3>,std::string> retained;
    Matrix control;Row control_c;
    for(size_t i=0;i<12;++i) for(size_t j=i+1;j<12;++j) for(size_t k=j+1;k<12;++k) {
        ++examined;Matrix m{catalog[i].row,catalog[j].row,catalog[k].row};
        const auto rank=matrix_rank(m);
        const Big det=determinant(m);require(abs(det)<=16,"bounded determinant");
        require((det!=0)==(rank==3),"exact determinant and modular rank agree");
        if(matrix_rank({m[0],m[1]})<2||matrix_rank({m[0],m[2]})<2||matrix_rank({m[1],m[2]})<2) {++pair_excluded;continue;}
        if(rank==3) {++full_rank;continue;}
        require(rank==2,"minimal triple rank2");const auto c=primitive(m);zero_equation(m,c);
        const std::array<char,3> types{catalog[i].type,catalog[j].type,catalog[k].type};
        const auto f=family(c,types);++counts[f];
        retained.emplace(std::array<int,3>{int(i),int(j),int(k)},f);
        if(minimal++) records<<',';
        records<<"{\"indices\":["<<i<<','<<j<<','<<k<<"],\"types\":\""<<types[0]<<types[1]<<types[2]<<"\",\"family\":\""<<f<<"\",\"rows\":[";
        for(size_t r=0;r<3;++r) {if(r) records<<',';records<<'[';for(size_t col=0;col<3;++col) records<<(col?",":"")<<m[r][col];records<<']';}
        records<<"],\"primitive_coefficients\":["<<c[0]<<','<<c[1]<<','<<c[2]<<"]}";
        control=m;control_c=c;
    }
    require(examined==220&&examined==pair_excluded+full_rank+minimal,"complete template domain partition");
    require(retained.at({6,8,11})=="U"&&retained.count({6,8,10})==0,"signed triangle versus full-rank unsigned triangle");
    require(retained.at({0,1,6})=="U"&&retained.at({0,6,7})=="V"&&retained.at({1,3,6})=="W"&&retained.at({3,4,6})=="V","path and repeated-image known cases");
    int controls=6;
    ++control_c[0];rejects([&]{zero_equation(control,control_c);});++controls;
    rejects([&]{family({1,1,2},{'A','A','C'});});++controls;
    rejects([&]{require(catalog.size()-1==12,"missing catalog vector");});++controls;
    std::ostringstream out;
    out<<"{\"standing\":\"producer-unverified\",\"scope\":\"minimal-three-row-support-classification\",\"catalog\":[";
    for(size_t i=0;i<catalog.size();++i) {if(i) out<<',';out<<"{\"type\":\""<<catalog[i].type<<"\",\"row\":["<<catalog[i].row[0]<<','<<catalog[i].row[1]<<','<<catalog[i].row[2]<<"]}";}
    out<<"],\"examined\":"<<examined<<",\"proportional_pair_excluded\":"<<pair_excluded<<",\"full_rank_excluded\":"<<full_rank<<",\"minimal_count\":"<<minimal<<",\"class_counts\":{\"U\":"<<counts["U"]<<",\"V\":"<<counts["V"]<<",\"W\":"<<counts["W"]<<"},\"coverage_complete\":true,\"controls_passed\":"<<controls<<",\"circuits\":["<<records.str()<<"]}\n";
    fresh_write(outdir+"/result.json",out.str());
    const auto a=var("a"),b0=var("b"),c=var("c"),d=var("d"),e=var("e"),f=var("f");
    issue(outdir+"/minor.identity.json","Generic three-row minor cancellation",{"a","b","c","d","e","f"},
        add({mul({sub(mul({c,f}),mul({d,e})),a}),mul({sub(mul({e,b0}),mul({f,a})),c}),mul({sub(mul({a,d}),mul({b0,c})),e})}),num(0),3);
    return 0;
}
struct Stream {
    std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> context{EVP_MD_CTX_new(),EVP_MD_CTX_free};
    explicit Stream(const std::string& arm) {require(bool(context)&&EVP_DigestInit_ex(context.get(),EVP_sha256(),nullptr)==1,"stream SHA256 setup");const std::string header="P192 three-row/v1\n"+arm+"\n";update(header);}
    void update(const std::string& s) {require(EVP_DigestUpdate(context.get(),s.data(),s.size())==1,"stream SHA256 update");}
    void add(int i,int j,int k,int sj,int sk,const std::string& q) {
        std::string s;for(int index:{i,j,k}) for(int n=0;n<4;++n) s+=char((unsigned(index)>>(8*n))&255U);
        s+=char(sj==1);s+=char(sk==1);s+=char(q.size());s+=q;update(s);
    }
    std::string finish() {std::array<unsigned char,32> result{};unsigned size=0;require(EVP_DigestFinal_ex(context.get(),result.data(),&size)==1&&size==32,"stream SHA256 finish");return ::hex(result);}
};
struct Prepared {size_t j,k;int sj,sk;Point q,doubled;SSLPoint sq,sd;};
struct Hit {int i,j,k,sj,sk;two_row::Match labels;};
int queries(const std::string& arm,const std::array<std::string,6>& paths) {
    const auto data=read_json(paths[0]),tangents=read_json(paths[1]),prior=read_json(paths[2]),parent=read_json(paths[3]),classification=read_json(paths[4]);
    require(classification.at("coverage_complete").scalar=="true"&&classification.at("examined").integer()==220,"classified support coverage premise");
    require(parent.at("two_row_exclusion_derived").scalar=="true"&&parent.at("aa_witness_count").integer()==0&&parent.at("ab_witness_count").integer()==0&&parent.at("single_row_premise_witnesses").integer()==0,"pinned two-row premise");
    std::vector<Point> base,images,symbols(1);audit::SSL ssl;
    for(const auto& record:data.at("projected_base").arr) {const auto q=read_point(record);require(!q.zero&&on_curve(q)&&q.y<=p-q.y,"canonical old base");ssl.make(q);base.push_back(q);symbols.push_back(q);symbols.push_back(negative(q));}
    require(base.size()==225&&symbols.size()==451,"frozen old alphabet");
    for(const auto& record:data.at("images").arr) {
        const auto q=read_point(record.at("point"));const auto j=record.at("projected_index").integer(),s=record.at("sign").integer();
        require(on_curve(q),"recorded image curve equation");ssl.make(q);
        if(j<0) require(j==-1&&s==0&&q.zero,"identity image");else require(size_t(j)<base.size()&&(s==1||s==-1)&&q==scaled(base[size_t(j)],s),"image sign binding");images.push_back(q);
    }
    require(images.size()==226&&tangents.at("tangents").arr.size()==226,"frozen tangent count");
    Point kappa;auto sk=ssl.make({});
    for(const auto& record:data.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr) {
        const auto& q=images.at(size_t(record.integer()));kappa=plus(kappa,q);auto sq=ssl.make(q);require(EC_POINT_add(ssl.group.get(),sk.get(),sk.get(),sq.get(),ssl.context.get())==1,"SSL reference");
    }
    require(key(kappa)==ssl.encode(sk.get()),"reference backend agreement");
    const auto g=signed_audit::enumerate(base,false),s=signed_audit::enumerate(base,true);
    require(g.size()==101926&&fingerprint(g)==prior.at("pair_fingerprint_sha256").scalar&&fingerprint(g)==parent.at("pair_fingerprint_sha256").scalar,"frozen old pair stream");
    const Table pairs(g,s);std::vector<Item> dg,ds;
    for(size_t i=0;i<symbols.size();++i) {dg.push_back({{int(i),-1},key(scaled(symbols[i],2)),""});ds.push_back({{int(i),-1},two_row::ssl_query(ssl,symbols[i],symbols[i],1,1),""});}
    const Table doubles(dg,ds);
    std::vector<Point> traces,doubled;std::vector<int> indices;
    std::vector<SSLPoint> st,sd;
    for(size_t i=0;i<images.size();++i) {
        const auto& record=tangents.at("tangents").arr[i];require(record.at("point_index").integer()==int(i),"ordered trace indices");
        const auto t=plus(kappa,negative(scaled(images[i],2)));
        require(t==read_point(record.at("trace"))&&key(negative(t))==two_row::ssl_query(ssl,images[i],kappa,2,-1),"trace backends and frozen record agree");
        if(record.at("category").scalar!="split-distinct-rational") continue;
        require(pairs.lookup(key(t)).empty(),"single-row premise");indices.push_back(int(i));traces.push_back(t);doubled.push_back(scaled(t,2));st.push_back(ssl.make(t));
        auto twice=ssl.make({});require(EC_POINT_dbl(ssl.group.get(),twice.get(),st.back().get(),ssl.context.get())==1&&ssl.encode(twice.get())==key(doubled.back()),"trace doubling backends");sd.push_back(std::move(twice));
    }
    require(indices.size()==102&&parent.at("split_indices").arr.size()==102,"exact split domain");for(size_t i=0;i<102;++i) require(indices[i]==parent.at("split_indices").arr[i].integer(),"same parent split order");
    int controls=0;
    auto known=[&](const Table& table,const Point& q,Pair label) {const auto matches=table.lookup(key(q));require(std::find(matches.begin(),matches.end(),label)!=matches.end(),"known-answer lookup");++controls;};
    known(pairs,{}, {0,0});known(pairs,negative(base[0]),{0,2});known(doubles,{}, {0,-1});known(doubles,scaled(base[0],-2),{2,-1});
    auto control=[&](int first,int second,const Table& table,Pair label) {
        const Point a=base[2],b0=base[3],goal=scaled(base[0],first==1&&second==1?1:2);
        const auto c=plus(plus(goal,negative(scaled(a,first))),negative(scaled(b0,second)));
        // Input c has coefficient one here; construct the arm's nonunit positions separately.
        Point z=c;
        if(second==2) {z=base[4];}
        Point x=a,y=b0;
        if(second==2) x=plus(goal,negative(plus(scaled(y,2),scaled(z,2))));
        const auto q=plus(plus(scaled(x,first),scaled(y,second)),scaled(z,second==2?2:1));
        const auto sum=plus(scaled(y,second),scaled(z,second==2?2:1));
        const auto ssl_key=two_row::ssl_query(ssl,scaled(x,first),sum,1,1);
        require(q==goal&&key(q)==ssl_key,"U/V/W coefficient known-answer");known(table,q,label);
        rejects([&]{require(plus(q,base[5])==goal,"mutated query witness");});++controls;
    };
    control(1,1,pairs,{0,1});control(2,1,doubles,{1,-1});control(1,2,doubles,{1,-1});
    std::vector<Prepared> prepared;prepared.reserve(20604);
    std::vector<Item> pg,ps;
    for(size_t j=0;j<102;++j) for(size_t k=j+1;k<102;++k) for(int sj:{1,-1}) for(int sk0:{1,-1}) {
        const auto q=plus(scaled(traces[j],sj),scaled(traces[k],sk0)),twice=scaled(q,2);
        auto a=ssl.make(traces[j]),b0=ssl.make(traces[k]),sq=ssl.make({}),ssd=ssl.make({});
        if(sj<0) require(EC_POINT_invert(ssl.group.get(),a.get(),ssl.context.get())==1,"prepared first sign");
        if(sk0<0) require(EC_POINT_invert(ssl.group.get(),b0.get(),ssl.context.get())==1,"prepared second sign");
        require(EC_POINT_add(ssl.group.get(),sq.get(),a.get(),b0.get(),ssl.context.get())==1&&EC_POINT_dbl(ssl.group.get(),ssd.get(),sq.get(),ssl.context.get())==1,"prepared pair arithmetic");
        const auto gkey=key(q),skey=ssl.encode(sq.get()),gdkey=key(twice),sdkey=ssl.encode(ssd.get());
        require(gkey==skey&&gdkey==sdkey,"every prepared pair and double agrees");
        const std::string tag=std::string(sj==1?"+":"-")+(sk0==1?"+":"-");
        pg.push_back({{indices[j],indices[k]},gkey,tag+gdkey});ps.push_back({{indices[j],indices[k]},skey,tag+sdkey});
        prepared.push_back({j,k,sj,sk0,q,twice,std::move(sq),std::move(ssd)});
    }
    require(prepared.size()==20604&&fingerprint(pg)==fingerprint(ps),"complete prepared stream");
    const auto& table=arm=="U"?pairs:doubles;Stream fg(arm),fs(arm);auto query_point=ssl.make({});size_t count=0,witness_count=0;
    std::vector<Hit> hits;
    for(size_t i=0;i<102;++i) {
        for(const auto& pair:prepared) {
            if(i==pair.j||i==pair.k||(arm=="U"&&i>pair.j)) continue;
            const auto& first=arm=="V"?doubled[i]:traces[i];const auto& rest=arm=="W"?pair.doubled:pair.q;
            const auto q=plus(first,rest);const auto gkey=key(q);
            const auto* ssl_first=arm=="V"?sd[i].get():st[i].get();const auto* ssl_rest=arm=="W"?pair.sd.get():pair.sq.get();
            require(EC_POINT_add(ssl.group.get(),query_point.get(),ssl_first,ssl_rest,ssl.context.get())==1,"independent query addition");
            const auto skey=ssl.encode(query_point.get());require(gkey==skey,"every triple query backend agreement");
            fg.add(indices[i],indices[pair.j],indices[pair.k],pair.sj,pair.sk,gkey);fs.add(indices[i],indices[pair.j],indices[pair.k],pair.sj,pair.sk,skey);++count;
            const auto labels=table.lookup(gkey);
            if(!labels.empty()) {witness_count+=labels.size();hits.push_back({indices[i],indices[pair.j],indices[pair.k],pair.sj,pair.sk,labels});}
            require(hits.size()<=4096&&witness_count<=16384,"witness cap; incomplete if exceeded");
        }
        std::cout<<"arm "<<arm<<" completed anchor "<<indices[i]<<" cumulative_queries "<<count<<" matching_queries "<<hits.size()<<" witnesses "<<witness_count<<std::endl;
    }
    require(count==(arm=="U"?686800U:2060400U),"complete fixed triple query count");
    const auto gdigest=fg.finish(),sdigest=fs.finish();require(gdigest==sdigest,"whole query stream agreement");
    std::ostringstream out;
    out<<"{\"standing\":\"producer-unverified\",\"scope\":\"necessary-three-row-tangent-identities\",\"arm\":\""<<arm<<"\",\"query_count\":"<<count<<",\"matching_queries\":"<<hits.size()<<",\"witness_count\":"<<witness_count<<",\"query_fingerprint_sha256\":\""<<gdigest<<"\",\"prepared_pair_fingerprint_sha256\":\""<<fingerprint(pg)<<"\",\"backend_streams_match\":true,\"grouped_sorted_joins_match\":true,\"controls_passed\":"<<controls<<",\"complete\":true,\"split_indices\":[";
    for(size_t i=0;i<102;++i) {out<<(i?",":"")<<indices[i];}
    out<<"],\"witnesses\":[";
    for(size_t i=0;i<hits.size();++i) {if(i) out<<',';const auto& h=hits[i];out<<"{\"indices\":["<<h.i<<','<<h.j<<','<<h.k<<"],\"signs\":[1,"<<h.sj<<','<<h.sk<<"],\"old_labels\":[";for(size_t j=0;j<h.labels.size();++j) out<<(j?",":"")<<'['<<h.labels[j][0]<<','<<h.labels[j][1]<<']';out<<"]}";}
    out<<"],\"new_quartic_points\":0,\"residual_roots_extracted\":0,\"production_collector_calls\":0}\n";fresh_write(paths[5],out.str());return 0;
}
int run(int argc,char** argv) {
    require(argc>=2,"explicit mode required");const std::string mode=argv[1];
    if(mode=="--templates") {require(argc==3,"templates output directory");require(access((std::string(argv[2])+"/result.json").c_str(),F_OK)!=0&&access((std::string(argv[2])+"/minor.identity.json").c_str(),F_OK)!=0,"fresh template outputs");return templates(argv[2]);}
    require(mode=="--u"||mode=="--v"||mode=="--w","known mode");require(argc==8,"arm baseline tangents signed parent classification output");require(access(argv[7],F_OK)!=0,"fresh query output");
    const std::string arm=mode=="--u"?"U":mode=="--v"?"V":"W";
    return queries(arm,{argv[2],argv[3],argv[4],argv[5],argv[6],argv[7]});
}
}
int main(int argc,char** argv) {try{return circuits::run(argc,argv);}catch(const std::exception& error){std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}}
