#define main frozen_expanded_conic_norm_entrypoint
#include "norms.cpp"
#undef main
#include "conic_arithmetic.hpp"
#include <cstdint>

using ECSparse=norm_test::Sparse;
using JAnchors=std::array<int,2>;
struct ECPencil {JAnchors anchors;};
struct ECDigest {
    std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> state{EVP_MD_CTX_new(),EVP_MD_CTX_free};
    ECDigest(){require(bool(state)&&EVP_DigestInit_ex(state.get(),EVP_sha256(),nullptr)==1,"conic digest initialize");}
    void add(size_t pencil,const std::string& key,const std::vector<int>& ids) {
        std::string payload=std::to_string(pencil)+":"+key;
        for(int id:ids)payload+=std::to_string(id)+",";
        require(payload.size()<65536,"bounded candidate digest frame");
        const auto n=std::uint32_t(payload.size());
        std::array<unsigned char,4> frame{};
        for(size_t i=0;i<4;++i)frame[i]=static_cast<unsigned char>((n>>(8*i))&255U);
        require(EVP_DigestUpdate(state.get(),frame.data(),frame.size())==1&&
                EVP_DigestUpdate(state.get(),payload.data(),payload.size())==1,"conic digest update");
    }
    std::string finish() {
        std::array<unsigned char,32> bytes{};unsigned int n=0;
        require(EVP_DigestFinal_ex(state.get(),bytes.data(),&n)==1&&n==32,"conic digest final");
        std::ostringstream out;
        for(auto b:bytes)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);
        return out.str();
    }
};
audit::Point ec_gmp(const ::Point& p) {
    if(p.zero)return {};
    return {false,audit::Big(decimal(p.x.p.at(0))),audit::Big(decimal(p.y.p.at(0)))};
}
ECSparse ec_sparse(const SCRow& row) {
    ECSparse out;
    for(size_t i=0;i<row.size();++i) {
        SCBig value=sc_reduce(row[i]);if(value>sc_n/2)value-=sc_n;
        require(value>=-16&&value<=16,"small conic source coefficient");
        if(value!=0)out[int(i)]=int(value.get_si());
    }
    return out;
}
ECSparse ec_read_sparse(const Json& row,size_t width) {
    ECSparse out;
    for(const auto& pair:row.arr) {
        const int column=pair.arr.at(0).integer(),coefficient=pair.arr.at(1).integer();
        require(column>=0&&size_t(column)<width&&coefficient&&coefficient>=-16&&coefficient<=16&&
                out.emplace(column,coefficient).second,"canonical saved sparse row");
    }
    return out;
}
bool ec_ssl_zero(const ECSparse& row,const std::vector<audit::Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,coefficient]:row) {
        auto p=ssl.make(base.at(size_t(column)));
        auto scalar=audit::bn(audit::Big(std::abs(coefficient)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
                "conic named coefficient multiplication");
        if(coefficient<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,"conic named negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,"conic named row sum");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}
SCRow ec_embed(const SCRow& row) {
    require(row.size()==481,"expanded geometric row width");
    SCRow result(685);
    for(size_t i=0;i<row.size();++i)result[i<225?i:i+204]=row[i];
    return result;
}
size_t ec_aux_rank(const SCMat& matrix) {
    SCMat auxiliary;
    for(const auto& row:matrix)auxiliary.emplace_back(row.begin()+225,row.end());
    return sc_rank(auxiliary);
}
void ec_emit_sparse(const SCRow& row) {
    std::cout<<'[';bool comma=false;
    for(auto [column,coefficient]:ec_sparse(row)) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<'['<<column<<','<<coefficient<<']';
    }
    std::cout<<']';
}
CV mi_coefficients(const BP& q) {
    CV out{};const std::array<Exponent,6> powers{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};
    for(size_t i=0;i<6;++i){const auto at=q.t.find(powers[i]);out[i]=at==q.t.end()?F(0):at->second;}
    return cnormalize(out);
}
struct MIBasis {
    SCMat source,reduced,provenance;
    std::vector<size_t> pivots;
    std::vector<std::string> labels;
    explicit MIBasis(SCMat rows):source(std::move(rows)) {
        for(size_t i=0;i<source.size();++i)labels.push_back("base:"+std::to_string(i));
        rebuild();
    }
    void rebuild() {
        require(!source.empty()&&source.size()<=256&&source[0].size()<=1024,"native independent-basis caps");
        reduced=source;provenance.assign(source.size(),SCRow(source.size()));pivots.clear();
        for(size_t i=0;i<source.size();++i) {
            require(source[i].size()==source[0].size(),"basis rectangle");
            provenance[i][i]=1;
            for(auto& value:reduced[i])value=sc_reduce(value);
        }
        size_t rank=0;
        for(size_t column=0;column<source[0].size()&&rank<source.size();++column) {
            size_t pivot=rank;while(pivot<source.size()&&reduced[pivot][column]==0)++pivot;
            if(pivot==source.size())continue;
            std::swap(reduced[pivot],reduced[rank]);std::swap(provenance[pivot],provenance[rank]);
            SCBig inverse;
            require(mpz_invert(inverse.get_mpz_t(),reduced[rank][column].get_mpz_t(),sc_n.get_mpz_t())==1,"basis inverse");
            for(auto& value:reduced[rank])value=sc_reduce(value*inverse);
            for(auto& value:provenance[rank])value=sc_reduce(value*inverse);
            for(size_t i=0;i<source.size();++i)if(i!=rank&&reduced[i][column]!=0) {
                const SCBig coefficient=reduced[i][column];
                for(size_t j=0;j<source[0].size();++j)reduced[i][j]=sc_reduce(reduced[i][j]-coefficient*reduced[rank][j]);
                for(size_t j=0;j<source.size();++j)provenance[i][j]=sc_reduce(provenance[i][j]-coefficient*provenance[rank][j]);
            }
            pivots.push_back(column);++rank;
        }
        require(rank==source.size(),"stored numerical basis stays independent");
    }
    bool verifies(const SCRow& row,const SCRow& coefficients)const {
        if(row.size()!=source[0].size()||coefficients.size()!=source.size())return false;
        SCRow reconstructed(row.size());
        for(size_t i=0;i<source.size();++i)if(coefficients[i]!=0)
            for(size_t j=0;j<row.size();++j)if(source[i][j]!=0)reconstructed[j]+=coefficients[i]*source[i][j];
        for(size_t j=0;j<row.size();++j)if(sc_reduce(reconstructed[j]-row[j])!=0)return false;
        return true;
    }
    std::pair<bool,SCRow> representation(const SCRow& row)const {
        require(row.size()==source[0].size(),"representation width");
        SCRow work=row,coefficients(source.size());
        for(auto& value:work)value=sc_reduce(value);
        for(size_t i=0;i<pivots.size();++i) {
            const SCBig value=work[pivots[i]];
            if(value==0)continue;
            for(size_t j=0;j<work.size();++j)work[j]=sc_reduce(work[j]-value*reduced[i][j]);
            for(size_t j=0;j<source.size();++j)coefficients[j]=sc_reduce(coefficients[j]+value*provenance[i][j]);
        }
        const bool dependent=std::all_of(work.begin(),work.end(),[](const SCBig& value){return value==0;});
        if(dependent)require(verifies(row,coefficients),"exact row-space reconstruction");
        return {dependent,std::move(coefficients)};
    }
    void append(const SCRow& row,const std::string& label) {
        require(source.size()<256&&!representation(row).first,"independent append within existing cap");
        source.push_back(row);labels.push_back(label);rebuild();
    }
};

#include "weighted-conic-arithmetic.hpp"

int main(int argc,char** argv) {
try {
    require(argc==9,"usage: high-contact-controls QUARTIC BACKMAP BASELINE TANGENTS OLD_SCAN EXPANDED BALANCED X_FIBRE_RESULT");
    const auto previous=read_json(argv[7]),next_previous=read_json(argv[8]);
    require(previous.at("controls_pass").scalar=="true"&&previous.at("balanced_conic_pattern_excluded").scalar=="true"&&
            previous.at("closed_rank").integer()==6,"frozen balanced-image exclusion");
    require(next_previous.at("controls_pass").scalar=="true"&&next_previous.at("supported_conics").integer()==0&&
            next_previous.at("closed_rank").integer()==6&&next_previous.at("all_x_fibre_pencils").integer()==625,
            "frozen completed x-fibre family");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),
               tangents=read_json(argv[4]),scan=read_json(argv[5]),expanded=read_json(argv[6]);
    ss_private(baseline,tangents);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("expanded_curve_pool").integer()==1434&&expanded.at("selected_sections").integer()==128,
            "successful frozen expansion dependency");
    const auto curve=read_bp(model.at("terms"),true);
    std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"actual expanded-conic map identity");
    std::vector<F> leading(5);
    for(const auto& [e,a]:curve.t)if(e[0]+e[1]==4)leading[size_t(e[1])]=a;
    const Poly infinity(std::move(leading)),z{F(0),F(1)};
    require(gcd(infinity,wide_power(z,prime(),infinity)-z).degree()==0,"no omitted rational source infinity");
    std::vector<::Point> base;
    std::vector<audit::Point> gmp_base,original_base;
    std::set<std::string> column_x;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"original429canonical image columns");
        original_base.push_back(ec_gmp(p));
        if(base.size()<225){base.push_back(p);gmp_base.push_back(ec_gmp(p));}
    }
    require(original_base.size()==429,"original full image width");
    for(size_t i=0;i<225;++i)require(base[i]==point(baseline.at("projected_base").arr.at(i)),"old225prefix unchanged");
    require(expanded.at("selected_images").arr.size()==256&&expanded.at("fibres").arr.size()==256,
            "all256new image/fibre records");
    for(const auto& q:expanded.at("selected_images").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"new canonical auxiliary image column");
        base.push_back(p);gmp_base.push_back(ec_gmp(p));
    }
    require(base.size()==481,"expanded geometric image width");
    std::vector<CPoint> points;
    std::map<std::string,int> coordinates;
    auto append=[&](const Json& q,int index,int sign) {
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        const auto actual=fs_image(curve,map,x,y).value;
        require(actual==point(q.at("image"))&&coordinates.emplace(sp_key(x,y),int(points.size())).second,
                "fresh distinct expanded conic-pool image");
        if(index<0)require(index==-1&&sign==0&&actual.zero,"identity pool label");
        else {
            require(size_t(index)<base.size()&&(sign==1||sign==-1),"signed pool label bounds");
            auto expected=base.at(size_t(index));if(sign<0)expected.y=-expected.y;
            require(actual==expected,"pool exact canonical image/sign");
        }
        const auto fx=fs_eval(curve.partial(0),x,y),fy=fs_eval(curve.partial(1),x,y);
        require(fx!=F(0)||fy!=F(0),"smooth expanded pool point");
        points.push_back({{x,y,actual,index,sign},{F(1),x,y,x*x,x*y,y*y},
                          {F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
    };
    for(const auto& q:scan.at("points").arr) {
        require(q.at("index").integer()<225,"old scan uses only original image domain");
        append(q,q.at("index").integer(),q.at("sign").integer());
    }
    require(points.size()==652,"unchanged old pool prefix");
    for(size_t i=0;i<256;++i) {
        const auto& fibre=expanded.at("fibres").arr.at(i);
        require(fibre.at("target").integer()==int(i),"ordered frozen fibre inventory");
        for(const auto& q:fibre.at("points").arr) {
            const auto claimed=point(q.at("image"));
            require(!claimed.zero&&claimed.x==base[225+i].x,"exact new fibre image x");
            append(q,int(225+i),claimed.y==base[225+i].y?1:-1);
        }
    }
    require(points.size()==1434&&points.size()<=3000,"complete bounded actual expanded pool");
    for(size_t i=0;i<226;++i) {
        const auto& q=baseline.at("pool").at("points").arr.at(i);
        require(decimal(points[i].value.x)==q.at("x").scalar&&decimal(points[i].value.y)==q.at("y").scalar&&
                points[i].value.image==point(baseline.at("images").arr.at(i).at("point")),"original226bindings");
    }
    std::vector<Anchors> known;
    std::vector<BP> lines;
    for(const auto& fibre:baseline.at("pool").at("split_fibers").arr) {
        Anchors ids{};
        require(fibre.at("point_indices").arr.size()==4,"original split degree4");
        for(size_t i=0;i<4;++i)ids[i]=fibre.at("point_indices").arr[i].integer();
        const auto& a=points.at(size_t(ids[0])).value;
        const auto& b=points.at(size_t(ids[1])).value;
        BP line;line.term({0,0},b.y*a.x-b.x*a.y);line.term({1,0},a.y-b.y);line.term({0,1},b.x-a.x);
        for(int id:ids)require(fs_eval(line,points.at(size_t(id)).value.x,points.at(size_t(id)).value.y)==F(0),
                               "original split line incidence");
        known.push_back(ids);lines.push_back(std::move(line));
    }
    require(known.size()==7,"original seven complete sections");
    SCRow reference(481);::Point kappa;
    for(int id:known[0]) {
        const auto& p=points.at(size_t(id)).value;kappa=add(kappa,p.image);
        if(p.index>=0)reference[size_t(p.index)]+=p.sign;
    }
    audit::SSL ssl;
    auto check_row=[&](const SCRow& row) {
        require(norm_test::row_sum(row,gmp_base).zero&&ec_ssl_zero(ec_sparse(row),gmp_base,ssl),
                "independent GMP/named conic or seed row sum");
    };
    std::vector<SCRow> line_rows;
    for(const auto& ids:known) {
        SCRow row(481);
        for(int id:ids){const auto& p=points.at(size_t(id)).value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t i=0;i<row.size();++i)row[i]-=reference[i];
        check_row(row);line_rows.push_back(std::move(row));
    }
    auto matrix=sc_read_matrix(tangents.at("matrix"));
    for(auto& row:matrix) {
        require(norm_test::row_sum(row,original_base).zero&&ec_ssl_zero(ec_sparse(row),original_base,ssl),
                "original108row sums in both implementations");
        row.resize(685);
    }
    std::set<ECSparse> seed_rows,allowed_rows;
    allowed_rows.insert(ECSparse{});
    for(const auto& row:baseline.at("rows").arr) {
        const auto sparse=ec_read_sparse(row,225);
        allowed_rows.insert(sparse);allowed_rows.insert(norm_test::opposite(sparse));
    }
    for(const auto& saved:expanded.at("seed_sections").arr) {
        const int i=saved.at("i").integer(),j=saved.at("j").integer();
        require(i>=0&&j>=0&&i<652&&j<652,"saved seed origins in old pool");
        const bool tangent=saved.at("tangent").scalar=="true";
        require(tangent?i==j:i<j,"saved seed secant/tangent order");
        const auto& a=points[size_t(i)].value;const auto& b=points[size_t(j)].value;
        const auto dx=coefficient(saved.at("direction").arr[0]),dy=coefficient(saved.at("direction").arr[1]);
        require(tangent?(dx==fs_eval(curve.partial(1),a.x,a.y)&&dy==-fs_eval(curve.partial(0),a.x,a.y)):
                        (dx==b.x-a.x&&dy==b.y-a.y),"saved seed direction binding");
        const auto restriction=ss_restrict(curve,a.x,a.y,dx,dy);
        require(restriction==read_poly(saved.at("restriction")),"actual saved seed restriction");
        const auto roots=ss_quadratic(restriction.exact(tangent?Poly{F(0),F(0),F(1)}:Poly{F(0),F(-1),F(1)}));
        require(roots.size()==2,"saved split residual factor product");
        std::array<std::string,2> computed{sp_key(a.x+dx*roots[0],a.y+dy*roots[0]),sp_key(a.x+dx*roots[1],a.y+dy*roots[1])},recorded;
        std::array<int,4> ids{i,j,0,0};
        for(size_t k=0;k<2;++k) {
            const auto& q=saved.at("residual_points").arr.at(k);
            recorded[k]=q.at("x").scalar+":"+q.at("y").scalar;
            ids[2+k]=coordinates.at(recorded[k]);
            require(points[size_t(ids[2+k])].value.image==point(q.at("image")),"saved residual exact image");
        }
        std::sort(computed.begin(),computed.end());std::sort(recorded.begin(),recorded.end());
        require(computed==recorded,"saved residual geometry and factors agree");
        SCRow row(481);
        for(int id:ids){const auto& p=points[size_t(id)].value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t k=0;k<row.size();++k)row[k]-=reference[k];
        const auto sparse=ec_sparse(row);
        require(sparse==ec_read_sparse(saved.at("row"),481)&&seed_rows.insert(sparse).second,"saved seed row exact binding");
        check_row(row);matrix.push_back(ec_embed(row));allowed_rows.insert(sparse);
        require(!tangent,"declared mixed-image family uses split secant seeds");
        std::set<int> distinct(ids.begin(),ids.end());require(distinct.size()==4,"four distinct known seed support points");
        BP line;line.term({0,0},dy*a.x-dx*a.y);line.term({1,0},-dy);line.term({0,1},dx);
        for(int id:ids)require(fs_eval(line,points[size_t(id)].value.x,points[size_t(id)].value.y)==F(0),"known seed line incidence");
        known.push_back(ids);lines.push_back(std::move(line));line_rows.push_back(row);
    }
    require(seed_rows.size()==128&&matrix.size()==236,"all128actual seed rows");
    std::set<ECSparse> norm_rows;
    for(const auto& row:expanded.at("norm_rows").arr) {
        const auto sparse=ec_read_sparse(row.at("row"),481);
        require(allowed_rows.count(sparse)&&norm_rows.insert(sparse).second,"saved norm row belongs to seed/baseline span");
    }
    require(norm_rows.size()==135,"all saved necessary norm rows bound");
    for(const auto& row:seed_rows)require(norm_rows.count(row),"all seeds in saved necessary norm inventory");
    require(sc_rank(matrix)==236&&ec_aux_rank(matrix)==230,"starting full236aux230closed6");
    auto changed_seed=norm_test::dense(*seed_rows.begin(),481);++changed_seed[0];
    require(!norm_test::row_sum(changed_seed,gmp_base).zero&&!ec_ssl_zero(ec_sparse(changed_seed),gmp_base,ssl),
            "changed seed coefficient rejects in both implementations");
    auto uncontaminated=[&](const SCMat& full) {
        for(size_t i=108;i<full.size();++i)for(size_t j=225;j<429;++j)
            require(sc_reduce(full[i][j])==0,"new row cannot touch peeled original private columns");
    };
    uncontaminated(matrix);
    auto contaminated=matrix;contaminated[108][225]=1;
    fs_reject([&]{uncontaminated(contaminated);});
    SCMat core;
    for(size_t i=0;i<matrix.size();++i) {
        if(i>=6&&i<108)continue;
        SCRow row;
        row.insert(row.end(),matrix[i].begin(),matrix[i].begin()+225);
        row.insert(row.end(),matrix[i].begin()+429,matrix[i].end());
        core.push_back(std::move(row));
    }
    require(core.size()==134&&core[0].size()==481&&sc_rank(core)==134&&ec_aux_rank(core)==128,
            "exact private-row elimination gives134full128auxiliary6closed");
    matrix=std::move(core);
    require(known.size()==135&&lines.size()==135&&line_rows.size()==135,"all135known split lines");
    MIBasis initial_basis(matrix);
    std::vector<SCRow> line_representations;
    std::set<std::string> distinct_lines;
    for(size_t i=0;i<lines.size();++i) {
        require(distinct_lines.insert(ckey(mi_coefficients(lines[i]))).second,"distinct known split lines");
        const auto representation=initial_basis.representation(line_rows[i]);
        require(representation.first,"every known line row belongs to initial basis");
        line_representations.push_back(representation.second);
    }
    const auto represented=initial_basis.representation(line_rows[1]);
    require(represented.first&&!ec_sparse(line_rows[1]).empty(),"nonzero known row representation control");
    auto bad_coefficients=represented.second;bad_coefficients[0]=sc_reduce(bad_coefficients[0]+1);
    require(!initial_basis.verifies(line_rows[1],bad_coefficients),"changed representation coefficient rejects");
    size_t free_old=0;
    while(free_old<225&&std::find(initial_basis.pivots.begin(),initial_basis.pivots.end(),free_old)!=initial_basis.pivots.end())++free_old;
    require(free_old<225,"synthetic old-column direction outside current span");
    auto numerical_fixture=initial_basis;SCRow unit(481);unit[free_old]=1;
    numerical_fixture.append(unit,"synthetic-old-unit");
    require(sc_rank(numerical_fixture.source)==135&&ec_aux_rank(numerical_fixture.source)==128,
            "separate numerical positive control has closed rank7");
    std::map<std::string,std::array<int,2>> known_products;
    for(size_t i=0;i<lines.size();++i)for(size_t j=i;j<lines.size();++j) {
        const auto key=ckey(mi_coefficients(lines[i]*lines[j]));
        require(known_products.emplace(key,std::array<int,2>{int(i),int(j)}).second,"unique normalized line-product reference");
    }
    require(known_products.size()==lines.size()*(lines.size()+1)/2,"complete known product dictionary");
    ECDigest product_digest;size_t product_number=0;
    for(const auto& [key,pair]:known_products)product_digest.add(product_number++,key,{pair[0],pair[1]});
    const auto product_sha=product_digest.finish();
    const auto saved_points=points;
    const auto saved_base=base;
    const auto saved_gmp=gmp_base;
    const auto saved_reference=reference;
    const auto saved_matrix=matrix;
    auto subject_digest=[&]() {
        ECDigest digest;size_t n=0;
        for(const auto& p:points) {
            std::string value="C:"+decimal(p.value.x)+":"+decimal(p.value.y)+":";
            value+=p.value.image.zero?"O":decimal(p.value.image.x.p.at(0))+":"+decimal(p.value.image.y.p.at(0));
            for(const auto& v:p.monomial)value+=":"+decimal(v);
            for(const auto& v:p.derivative)value+=":"+decimal(v);
            digest.add(n++,value,{p.value.index,p.value.sign});
        }
        for(const auto& p:base)digest.add(n++,"B:"+decimal(p.x.p.at(0))+":"+decimal(p.y.p.at(0)),{});
        for(const auto& p:gmp_base)digest.add(n++,"G:"+p.x.get_str()+":"+p.y.get_str(),{});
        for(const auto& row:matrix) {
            std::string value="M:";for(const auto& v:row)value+=v.get_str()+",";digest.add(n++,value,{});
        }
        std::string value="R:";for(const auto& v:reference)value+=v.get_str()+",";digest.add(n++,value,{});
        return digest.finish();
    };
    const auto before_digest=subject_digest();
    auto restored=[&]() {
        require(points.size()==saved_points.size()&&base==saved_base&&gmp_base==saved_gmp&&
                reference==saved_reference&&matrix==saved_matrix,"exact actual subject sizes/base/matrix restored");
        for(size_t i=0;i<points.size();++i) {
            const auto& p=points[i];const auto& q=saved_points[i];
            require(p.value.x==q.value.x&&p.value.y==q.value.y&&p.value.image==q.value.image&&
                    p.value.index==q.value.index&&p.value.sign==q.value.sign&&p.monomial==q.monomial&&
                    p.derivative==q.derivative,"exact actual point/sign/monomial restoration");
        }
        require(subject_digest()==before_digest,"actual subject digest restored");
    };
    int selected_seed=-1,anchor=-1,other=-1;
    F direction_x,direction_y;Poly tangent_restriction,residual_quadratic;
    std::vector<F> residual_parameters;
    BP tangent_line;
    for(size_t seed=0;seed<128&&selected_seed<0;++seed) {
        const int id=known[7+seed][0];const auto p=points[size_t(id)].value;
        const auto dx=fs_eval(curve.partial(1),p.x,p.y),dy=-fs_eval(curve.partial(0),p.x,p.y);
        const auto restriction=ss_restrict(curve,p.x,p.y,dx,dy);
        require(restriction.degree()==4&&restriction.at(0)==F(0)&&restriction.at(1)==F(0),"exact tangent restriction");
        if(restriction.at(2)==F(0))continue;
        const auto residual=restriction.exact(Poly{F(0),F(0),F(1)});
        const auto roots=ss_quadratic(residual);
        if(roots.size()!=2||roots[0]==roots[1]||roots[0]==F(0)||roots[1]==F(0))continue;
        selected_seed=int(seed);anchor=id;other=known[7+seed][2];direction_x=dx;direction_y=dy;
        tangent_restriction=restriction;residual_quadratic=residual;residual_parameters=roots;
        tangent_line.term({0,0},dy*p.x-dx*p.y);tangent_line.term({1,0},-dy);tangent_line.term({0,1},dx);
    }
    require(selected_seed>=0,"bounded suitable ordinary split tangent fixture exists");
    auto wrong_factor=residual_parameters;wrong_factor[0]=wrong_factor[0]+F(1);
    fs_reject([&]{require((Poly{-wrong_factor[0],F(1)}*Poly{-wrong_factor[1],F(1)}).scale(residual_quadratic.at(2))==
                         residual_quadratic,"changed tangent residual factor");});
    std::map<std::string,int> fixture_coordinates=coordinates;
    std::vector<int> appended_ids;
    auto fixture_point=[&](const F& x,const F& y) {
        require(fs_eval(curve,x,y)==F(0),"fixture point on actual quartic");
        const int old_precision=precision;const auto mapped=fs_image(curve,map,x,y);precision=old_precision;
        const auto actual=mapped.value;valid(actual);
        require(actual.zero||(actual.x.p.degree()<=0&&actual.y.p.degree()<=0),"rational fixture elliptic image");
        const auto present=fixture_coordinates.find(sp_key(x,y));
        if(present!=fixture_coordinates.end()) {
            require(points[size_t(present->second)].value.image==actual,"existing fixture point image binding");
            return present->second;
        }
        int column=-1,sign=0;
        if(!actual.zero) {
            auto canonical=actual;
            if(BN_cmp(canonical.y.p.at(0).n.get(),(-canonical.y.p.at(0)).n.get())>0)canonical.y=-canonical.y;
            for(size_t i=0;i<base.size();++i)if(base[i].x==canonical.x) {
                require(base[i]==canonical,"existing canonical fixture image binding");column=int(i);break;
            }
            if(column<0) {
                column=int(base.size());base.push_back(canonical);gmp_base.push_back(ec_gmp(canonical));
                require(audit::on_curve(gmp_base.back()),"new fixture image independent curve equation");
            }
            sign=actual.y==canonical.y?1:-1;
            auto expected=base[size_t(column)];if(sign<0)expected.y=-expected.y;
            require(expected==actual,"fixture canonical image/sign binding");
        }
        const auto fx=fs_eval(curve.partial(0),x,y),fy=fs_eval(curve.partial(1),x,y);
        require(fx!=F(0)||fy!=F(0),"smooth fixture point");
        const int id=int(points.size());
        points.push_back({{x,y,actual,column,sign},{F(1),x,y,x*x,x*y,y*y},
                          {F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
        require(fixture_coordinates.emplace(sp_key(x,y),id).second,"unique control-only point");
        appended_ids.push_back(id);
        require(points.size()<=3000&&base.size()<=1024&&base.size()==gmp_base.size(),"unchanged fixture pool/column caps");
        return id;
    };
    const auto a=points[size_t(anchor)].value;
    std::array<int,2> residual_ids{};
    for(size_t i=0;i<2;++i)residual_ids[i]=fixture_point(a.x+direction_x*residual_parameters[i],a.y+direction_y*residual_parameters[i]);
    const size_t fixture_points=points.size(),fixture_columns=base.size();
    SCRow tangent_row(base.size());::Point tangent_sum;
    for(auto [id,m]:std::array<std::array<int,2>,3>{{{anchor,2},{residual_ids[0],1},{residual_ids[1],1}}}) {
        const auto& p=points[size_t(id)].value;
        require(fs_eval(tangent_line,p.x,p.y)==F(0),"complete tangent line incidence");
        for(int n=0;n<m;++n)tangent_sum=add(tangent_sum,p.image);
        if(p.index>=0)tangent_row[size_t(p.index)]+=m*p.sign;
    }
    require(tangent_sum==kappa,"actual full tangent image sum reference");
    for(size_t i=0;i<reference.size();++i)tangent_row[i]-=reference[i];
    require(norm_test::row_sum(tangent_row,gmp_base).zero&&ec_ssl_zero(ec_sparse(tangent_row),gmp_base,ssl),
            "tangent fixture row independent group checks");
    WEngine engine(curve,points,gmp_base,reference,kappa,ssl);
    WPencil profile31{{anchor,3},{other,1}};
    std::sort(profile31.begin(),profile31.end(),[](const WAnchor& p,const WAnchor& q){return p.id<q.id;});
    const WPencil profile4{{anchor,4}};
    const auto result31=engine.collect(profile31,true),result4=engine.collect(profile4,true);
    const auto q31=mi_coefficients(tangent_line*lines[size_t(7+selected_seed)]),q4=mi_coefficients(tangent_line*tangent_line);
    require(result31.sections.count(ckey(q31))==1&&result4.sections.count(ckey(q4))==1,"both nonempty higher-contact fixtures recovered");
    const auto& section31=result31.sections.at(ckey(q31));const auto& section4=result4.sections.at(ckey(q4));
    require(section31.support.at(anchor)==3&&section4.support.at(anchor)==4,"actual prescribed triple/quadruple contact");
    auto seed_row=line_rows[size_t(7+selected_seed)];seed_row.resize(base.size());
    auto expected31=tangent_row,expected4=tangent_row;
    for(size_t i=0;i<base.size();++i){expected31[i]+=seed_row[i];expected4[i]*=2;}
    require(section31.row==expected31&&section4.row==expected4&&!ec_sparse(section31.row).empty()&&!ec_sparse(section4.row).empty(),
            "nonzero fixtures match exact parent-row decompositions");
    (void)engine.replay(section31.coefficients,section31.support,section31.row,profile31);
    (void)engine.replay(section4.coefficients,section4.support,section4.row,profile4);
    auto corrupt_support=section31.support;++corrupt_support.begin()->second;
    fs_reject([&]{(void)engine.replay(section31.coefficients,corrupt_support,section31.row,profile31);});
    auto missing_support=section4.support;missing_support.erase(missing_support.begin());
    fs_reject([&]{(void)engine.replay(section4.coefficients,missing_support,section4.row,profile4);});
    auto corrupt_row=section31.row;++corrupt_row[0];
    fs_reject([&]{(void)engine.replay(section31.coefficients,section31.support,corrupt_row,profile31);});
    auto corrupt_parent=expected4;++corrupt_parent[0];
    fs_reject([&]{require(section4.row==corrupt_parent,"changed parent-row decomposition");});
    const size_t fixture_row_rank=sc_rank(SCMat{section31.row,section4.row});
    const auto fixture_point_records=std::vector<CPoint>(points.begin()+long(saved_points.size()),points.end());
    const auto fixture_image_records=std::vector<::Point>(base.begin()+long(saved_base.size()),base.end());
    points.resize(saved_points.size());base.resize(saved_base.size());gmp_base.resize(saved_gmp.size());
    engine.clear_appended_cache(saved_points.size());restored();
    const auto original_sign=points[0].value.sign;++points[0].value.sign;
    fs_reject([&]{restored();});points[0].value.sign=original_sign;restored();
    const size_t full=sc_rank(matrix),auxiliary=ec_aux_rank(matrix);
    require(full==134&&auxiliary==128&&matrix.size()==134&&matrix[0].size()==481,"actual core unchanged after controls");
    auto emit_cv=[&](const CV& q) {
        std::cout<<'[';for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<'"'<<decimal(q[i])<<'"';std::cout<<']';
    };
    auto emit_fixture=[&](const std::string& name,const WPencil& pencil,const WCollection& result,const CV& expected) {
        std::cout<<"{\"profile\":\""<<name<<"\",\"anchors\":[";
        for(size_t i=0;i<pencil.size();++i)std::cout<<(i?",":"")<<'['<<pencil[i].id<<','<<pencil[i].contact<<']';
        std::cout<<"],\"expected_conic\":";emit_cv(expected);
        std::cout<<",\"candidate_occurrences\":"<<result.candidate_occurrences<<",\"supported_control_conics\":"<<result.sections.size()
                 <<",\"higher_contact_orders\":[";
        for(size_t i=0;i<result.higher_orders.size();++i)std::cout<<(i?",":"")<<result.higher_orders[i];
        std::cout<<"],\"higher_contact_parameters\":[";
        for(size_t i=0;i<result.higher_parameters.size();++i){if(i)std::cout<<',';emit_cv(result.higher_parameters[i]);}
        std::cout<<"],\"sections\":[";bool comma=false;
        for(const auto& [key,section]:result.sections) {
            (void)key;if(comma)std::cout<<',';comma=true;
            std::cout<<"{\"coefficients\":";emit_cv(section.coefficients);std::cout<<",\"support\":[";bool sep=false;
            for(auto [id,m]:section.support){std::cout<<(sep?",":"")<<'['<<id<<','<<m<<']';sep=true;}
            std::cout<<"],\"row\":";ec_emit_sparse(section.row);std::cout<<'}';
        }
        std::cout<<"]}";
    };
    std::cout<<"{\"scope\":\"higher-contact-control-only-preflight\",\"actual_rank_search_run\":false,\"actual_pencils_scanned\":0,"
             <<"\"fixture_rows_entering_actual_matrix\":0,\"old_columns\":225,\"actual_pool_size\":1434,\"actual_working_columns\":481,"
             <<"\"control_pool_size\":"<<fixture_points<<",\"control_columns\":"<<fixture_columns
             <<",\"control_points_added\":"<<fixture_points-saved_points.size()<<",\"control_columns_added\":"<<fixture_columns-saved_base.size()
             <<",\"control_row_rank\":"<<fixture_row_rank<<",\"selected_seed\":"<<selected_seed<<",\"anchor\":"<<anchor
             <<",\"other_anchor\":"<<other<<",\"tangent_direction\":[\""<<decimal(direction_x)<<"\",\""<<decimal(direction_y)
             <<"\"],\"tangent_restriction\":[";
    for(int i=0;i<=tangent_restriction.degree();++i)std::cout<<(i?",":"")<<'"'<<decimal(tangent_restriction.at(i))<<'"';
    std::cout<<"],\"tangent_residual_parameters\":[\""<<decimal(residual_parameters[0])<<"\",\""<<decimal(residual_parameters[1])
             <<"\"],\"tangent_residual_point_ids\":["<<residual_ids[0]<<','<<residual_ids[1]<<"],\"tangent_line\":";
    emit_cv(mi_coefficients(tangent_line));std::cout<<",\"tangent_row\":";ec_emit_sparse(tangent_row);
    std::cout<<",\"control_points\":[";
    for(size_t i=0;i<fixture_point_records.size();++i) {
        const auto& p=fixture_point_records[i].value;
        std::cout<<(i?",":"")<<"{\"id\":"<<saved_points.size()+i<<",\"x\":\""<<decimal(p.x)<<"\",\"y\":\""<<decimal(p.y)
                 <<"\",\"index\":"<<p.index<<",\"sign\":"<<p.sign<<",\"image\":";
        if(p.image.zero)std::cout<<"{\"infinity\":true}";
        else std::cout<<"{\"infinity\":false,\"x\":\""<<decimal(p.image.x.p.at(0))<<"\",\"y\":\""<<decimal(p.image.y.p.at(0))<<"\"}";
        std::cout<<'}';
    }
    std::cout<<"],\"control_images\":[";
    for(size_t i=0;i<fixture_image_records.size();++i) {
        const auto& p=fixture_image_records[i];
        std::cout<<(i?",":"")<<"{\"index\":"<<saved_base.size()+i<<",\"infinity\":false,\"x\":\""<<decimal(p.x.p.at(0))
                 <<"\",\"y\":\""<<decimal(p.y.p.at(0))<<"\"}";
    }
    std::cout<<"],\"fixtures\":[";emit_fixture("3+1",profile31,result31,q31);std::cout<<',';emit_fixture("4",profile4,result4,q4);
    std::cout<<"],\"actual_subject_digest_before\":\""<<before_digest<<"\",\"actual_subject_digest_after\":\""<<subject_digest()
             <<"\",\"actual_state_restored\":true,\"core_full_rank\":"<<full<<",\"core_auxiliary_rank\":"<<auxiliary
             <<",\"restored_ranks_are_derived\":true,\"full_rank\":"<<full+102<<",\"auxiliary_rank\":"<<auxiliary+102
             <<",\"closed_rank\":"<<full-auxiliary<<",\"controls_pass\":true,\"known_product_reference_sha256\":\""<<product_sha
             <<"\",\"numerical_positive_control_closed_rank\":7,\"synthetic_rows_in_actual_matrix\":0,\"positive_gain_established\":false,"
             <<"\"standing\":\"producer-unverified;control-only preflight;actual3+1/4searchnotrun;positive needs actual KM and separate replay\"}\n";
    return 0;
}catch(const std::exception& error){std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}
}
