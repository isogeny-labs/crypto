#include "separate_arithmetic.hpp"
#include <fstream>

using CheckRow=std::vector<mpz_class>;
using CheckMatrix=std::vector<CheckRow>;
CheckMatrix cc_matrix(const Json& json) {
    CheckMatrix matrix;
    for(const auto& row:json.arr) {
        CheckRow values;
        for(const auto& value:row.arr) values.emplace_back(value.scalar);
        matrix.push_back(values);
    }
    return matrix;
}
void cc_closure(const CheckMatrix& matrix,size_t old,const Json& proof) {
    require(!matrix.empty()&&old<=matrix[0].size(),"separate closure shape");
    const size_t count=matrix.size(),width=matrix[0].size();
    CheckMatrix fresh;
    for(const auto& row:matrix) {
        require(row.size()==width,"separate rectangle");
        fresh.emplace_back(row.begin()+static_cast<std::ptrdiff_t>(old),row.end());
    }
    const size_t total=size_t(separate_rank(matrix)), nrank=size_t(separate_rank(fresh));
    const auto combinations=cc_matrix(proof.at("combinations")), rows=cc_matrix(proof.at("old_rows"));
    require(combinations.size()==count-nrank&&rows.size()==combinations.size(),"complete left-kernel dimension");
    require(size_t(separate_rank(combinations))==count-nrank,"independent complete kernel witnesses");
    for(size_t i=0;i<combinations.size();++i) {
        require(combinations[i].size()==count&&rows[i].size()==old,"witness shape");
        CheckRow product(width);
        for(size_t j=0;j<count;++j) for(size_t k=0;k<width;++k)
            product[k]=reduce(product[k]+combinations[i][j]*matrix[j][k],order_r);
        for(size_t k=0;k<width;++k)
            require(product[k]==(k<old?reduce(rows[i][k],order_r):mpz_class(0)),"direct full witness equation");
    }
    const size_t closed=size_t(separate_rank(rows));
    const int baseline=proof.at("baseline_rank").integer();
    require(baseline>=0&&size_t(baseline)<=closed,"separate baseline rank");
    require(total==nrank+closed&&proof.at("full_rank").integer()==int(total)&&
            proof.at("new_rank").integer()==int(nrank)&&proof.at("closed_rank").integer()==int(closed)&&
            proof.at("gain").integer()==int(closed)-baseline,"separate closed-rank identity");
}
void cc_quadratic(const Json& item) {
    const auto& coeff=item.at("coefficients").arr;
    require(coeff.size()==3,"checker quadratic shape");
    const mpz_class c(coeff[0].scalar),b(coeff[1].scalar),a(coeff[2].scalar);
    require(reduce(a)!=0,"checker quadratic leading term");
    const mpz_class disc=reduce(b*b-4*a*c), exponent=(modulus_p-1)/2;
    mpz_class euler;
    mpz_powm(euler.get_mpz_t(),disc.get_mpz_t(),exponent.get_mpz_t(),modulus_p.get_mpz_t());
    const std::string expected=disc==0?"repeated":euler==1?"distinct":"irreducible";
    require(item.at("category").scalar==expected,"separate quadratic category");
    const auto& roots=item.at("roots").arr;
    if(expected=="irreducible") { require(roots.empty(),"no invented irreducible roots"); return; }
    require(roots.size()==2,"both scalar roots");
    const mpz_class x(roots[0].scalar),y(roots[1].scalar);
    require(x>=0&&x<modulus_p&&y>=0&&y<modulus_p&&x<=y,"full-field canonical roots");
    require((x==y)==(expected=="repeated"),"root multiplicity");
    require(reduce(a*(x+y)+b)==0&&reduce(a*x*y-c)==0,"separate root-product equality");
}
void cc_self_test(const std::string& fixtures,const std::string& producer,const std::string& destination) {
    const auto source=read_json(fixtures),data=read_json(producer);
    require(data.at("actual_collection_executed").scalar=="false"&&data.at("new_curve_points").integer()==0,"synthetic scope");
    const auto& quadratics=data.at("quadratics").arr;
    require(quadratics.size()==5,"all declared quadratics");
    for(const auto& q:quadratics) cc_quadratic(q);
    auto bad_root=quadratics.at(0);
    bad_root.obj["roots"].arr[0]=bad_root.obj["roots"].arr[1];
    must_reject([&]{cc_quadratic(bad_root);});
    const auto& cases=data.at("cases").arr;
    const std::vector<int> gains{0,1,1,0,1,1};
    require(cases.size()==6&&source.at("cases").arr.size()==6,"all declared coefficient cases");
    for(size_t i=0;i<cases.size();++i) {
        const auto matrix=cc_matrix(cases[i].at("matrix"));
        require(matrix==cc_matrix(source.at("cases").arr[i].at("matrix"))&&
            cases[i].at("name").scalar==source.at("cases").arr[i].at("name").scalar,"immutable matrix fixture binding");
        cc_closure(matrix,3,cases[i].at("proof"));
        require(cases[i].at("proof").at("gain").integer()==gains[i],"known-answer closure gain");
    }
    const auto matrix=cc_matrix(cases[1].at("matrix"));
    const auto proof=cases[1].at("proof");
    auto changed=proof;
    auto& coefficient=changed.obj["combinations"].arr[0].arr[0].scalar;
    coefficient=mpz_class(mpz_class(coefficient)+1).get_str();
    must_reject([&]{cc_closure(matrix,3,changed);});
    changed=proof;
    auto& projected=changed.obj["old_rows"].arr[0].arr[0].scalar;
    projected=mpz_class(mpz_class(projected)+1).get_str();
    must_reject([&]{cc_closure(matrix,3,changed);});
    changed=proof;
    changed.obj["closed_rank"].scalar=std::to_string(proof.at("closed_rank").integer()+1);
    must_reject([&]{cc_closure(matrix,3,changed);});
    std::ofstream out(destination);
    require(bool(out),"new synthetic checker result");
    out << "{\"standing\":\"producer-unverified\",\"quadratics_checked\":5,\"closure_cases_checked\":6,\"corruptions_rejected\":4,\"complete_left_kernel_witnesses_checked\":true,\"new_curve_points\":0,\"actual_collection_executed\":false}\n";
    require(bool(out),"complete synthetic checker result");
}
using CPoly=std::vector<mpz_class>;
CPoly cc_product(const CPoly& a,const CPoly& b) {
    CPoly out(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i) for(size_t j=0;j<b.size();++j) out[i+j]=reduce(out[i+j]+a[i]*b[j]);
    return out;
}
CPoly cc_power(CPoly a,int exponent) {
    CPoly out{1};
    while(exponent--) out=cc_product(out,a);
    return out;
}
void cc_geometry(const Json& baseline,const Json& tangents,const Json& data,const SeparateMap& map) {
    const auto& points=data.at("points").arr;
    const auto& sections=data.at("sections").arr;
    const auto& records=tangents.at("tangents").arr;
    require(records.size()==226&&sections.size()==102,"full declared selection domain");
    size_t selected=0;
    std::vector<bool> used(points.size());
    for(size_t i=0;i<226;++i) used.at(i)=true;
    for(size_t i=0;i<records.size();++i) {
        const auto& point=baseline.at("pool").at("points").arr.at(i);
        const mpz_class ox(point.at("x").scalar),oy(point.at("y").scalar);
        const auto& record=records[i];
        require(record.at("point_index").integer()==int(i),"separate tangent ordering");
        const mpz_class a(record.at("line").arr[0].scalar),b(record.at("line").arr[1].scalar),c(record.at("line").arr[2].scalar);
        require((a!=0||b!=0)&&reduce(a*ox+b*oy+c)==0,"checker tangent incidence");
        const mpz_class fx=evaluate(derivative(map.curve,0),ox,oy),fy=evaluate(derivative(map.curve,1),ox,oy);
        require((fx!=0||fy!=0)&&reduce(a*fy-b*fx)==0,"checker gradient tangent");
        CPoly restriction(5);
        for(const auto& term:map.curve) {
            const auto value=cc_product(cc_power({ox,b},term.x),cc_power({oy,reduce(-a)},term.y));
            for(size_t j=0;j<value.size();++j) restriction[j]=reduce(restriction[j]+term.c*value[j]);
        }
        require(restriction[0]==0&&restriction[1]==0&&restriction[4]!=0,"full checker tangent degree4");
        require(record.at("restriction").arr.size()==5,"recorded restriction shape");
        for(size_t j=0;j<5;++j) require(restriction[j]==mpz_class(record.at("restriction").arr[j].scalar),"independent restriction coefficients");
        const mpz_class disc=reduce(restriction[3]*restriction[3]-4*restriction[2]*restriction[4]),exponent=(modulus_p-1)/2;
        mpz_class value;
        mpz_powm(value.get_mpz_t(),disc.get_mpz_t(),exponent.get_mpz_t(),modulus_p.get_mpz_t());
        const bool split=disc!=0&&value==1;
        const std::string category=disc==0?"repeated-contact":split?"split-distinct-rational":"irreducible-quadratic";
        require(category==record.at("category").scalar,"independent every selection label");
        if(!split) continue;
        require(selected<sections.size()&&sections[selected].at("point_index").integer()==int(i),"all selected sections without omissions");
        const auto& ids=sections[selected++].at("residual_indices").arr;
        require(ids.size()==2&&ids[0].integer()!=ids[1].integer(),"distinct residual supports");
        std::array<mpz_class,2> roots;
        for(size_t j=0;j<2;++j) {
            const int index=ids[j].integer();
            require(index>=0&&size_t(index)<points.size(),"residual point index");
            used.at(size_t(index))=true;
            const auto& p=points[size_t(index)];
            const mpz_class x(p.at("x").scalar),y(p.at("y").scalar);
            require(reduce(a*x+b*y+c)==0&&evaluate(map.curve,x,y)==0,"separate residual incidence");
            roots[j]=b!=0?reduce((x-ox)*inverse(b)):reduce((y-oy)*inverse(-a));
            require(reduce(ox+b*roots[j])==x&&reduce(oy-a*roots[j])==y,"complete residual parameter");
        }
        const auto replay=cc_product({0,0,restriction[4]},cc_product({reduce(-roots[0]),1},{reduce(-roots[1]),1}));
        require(replay==restriction,"independent full tangent product");
    }
    require(selected==sections.size(),"no spurious selected section");
    require(std::all_of(used.begin(),used.end(),[](bool value){return value;}),"no unwitnessed new point");
}
void cc_check(const std::string& model_path,const std::string& map_path,const std::string& baseline_path,
              const std::string& tangents_path,const std::string& producer,const std::string& destination) {
    const auto model=read_json(model_path),map_input=read_json(map_path),baseline=read_json(baseline_path),tangents=read_json(tangents_path),data=read_json(producer);
    SeparateMap map(model,map_input);
    OpenSSLGroup ec;
    const auto& points=data.at("points").arr;
    const auto& images=data.at("images").arr;
    const auto& bases=data.at("projected_base").arr;
    require(points.size()>=226&&points.size()<=430&&images.size()==points.size()&&bases.size()>=225&&bases.size()<=429,"checker full domain sizes");
    std::vector<EP> base;
    std::set<std::pair<mpz_class,mpz_class>> unique;
    for(size_t i=0;i<bases.size();++i) {
        const auto q=read_point(bases[i]);
        require(!q.infinity&&q.y<=modulus_p-q.y&&unique.emplace(q.x,q.y).second,"canonical distinct base");
        if(i<225) require(same(q,read_point(baseline.at("projected_base").arr.at(i))),"original base prefix");
        base.push_back(ec.point(q));
    }
    std::vector<std::array<int,2>> projection;
    std::vector<bool> used_base(base.size());
    std::set<std::pair<mpz_class,mpz_class>> unique_points;
    for(size_t i=0;i<points.size();++i) {
        const mpz_class x(points[i].at("x").scalar),y(points[i].at("y").scalar);
        require(x>=0&&x<modulus_p&&y>=0&&y<modulus_p,"full-field quartic coordinates");
        require(unique_points.emplace(x,y).second,"unique full-coordinate quartic points");
        if(i<226) {
            const auto& old=baseline.at("pool").at("points").arr.at(i);
            require(x==mpz_class(old.at("x").scalar)&&y==mpz_class(old.at("y").scalar),"unchanged original point prefix");
        }
        const auto actual=map.image(x,y),recorded=read_point(images[i].at("point"));
        require(same(actual,recorded),"independent individual rational-map image");
        auto p=ec.point(recorded),order=ec.empty();
        ec.add(order,p,order_r);
        require(ec.zero(order),"OpenSSL point subgroup");
        const int index=images[i].at("projected_index").integer(),sign=images[i].at("sign").integer();
        if(recorded.infinity) require(index==-1&&sign==0,"identity binding");
        else {
            require(index>=0&&size_t(index)<base.size()&&(sign==1||sign==-1),"projection binding range");
            auto expected=read_point(bases[size_t(index)]);
            if(sign<0) expected.y=reduce(-expected.y);
            require(same(expected,recorded),"signed canonical binding");
            used_base[size_t(index)]=true;
        }
        projection.push_back({index,sign});
    }
    require(std::all_of(used_base.begin(),used_base.end(),[](bool value){return value;}),"no unused base column");
    cc_geometry(baseline,tangents,data,map);
    const auto matrix=cc_matrix(data.at("matrix"));
    require(matrix.size()==108,"all original and selected rows");
    require(separate_rank(CheckMatrix(matrix.begin(),matrix.begin()+6))==6,"independent original rank6");
    auto check_sum=[&](const CheckRow& row) {
        require(row.size()==base.size(),"elliptic row width");
        auto sum=ec.empty();
        for(size_t i=0;i<row.size();++i) if(row[i]!=0) ec.add(sum,base[i],row[i]);
        require(ec.zero(sum),"separate transported elliptic sum");
    };
    for(size_t i=0;i<matrix.size();++i) {
        CheckRow expected(base.size());
        if(i<6) for(const auto& entry:baseline.at("rows").arr.at(i).arr)
            expected.at(size_t(entry.arr[0].integer()))=mpz_class(entry.arr[1].scalar);
        else {
            const auto& section=data.at("sections").arr.at(i-6);
            auto add=[&](int id,int coefficient) { const auto p=projection.at(size_t(id)); if(p[0]>=0) expected[size_t(p[0])]+=coefficient*p[1]; };
            add(section.at("point_index").integer(),2);
            for(const auto& id:section.at("residual_indices").arr) add(id.integer(),1);
            for(const auto& id:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) add(id.integer(),-1);
        }
        require(expected==matrix[i],"independent complete row coefficients");
        check_sum(matrix[i]);
    }
    cc_closure(matrix,225,data.at("closure"));
    require(data.at("closure").at("baseline_rank").integer()==6,"fixed baseline rank binding");
    for(const auto& row:cc_matrix(data.at("closure").at("old_rows"))) {
        auto sum=ec.empty();
        for(size_t i=0;i<row.size();++i) if(row[i]!=0) ec.add(sum,base[i],row[i]);
        require(ec.zero(sum),"closed old-base elliptic relation");
    }
    auto bad=data;
    bad.obj["sections"].arr[0].obj["residual_indices"].arr[0]=bad.obj["sections"].arr[0].obj["residual_indices"].arr[1];
    must_reject([&]{cc_geometry(baseline,tangents,bad,map);});
    auto bad_labels=tangents;
    const int first=data.at("sections").arr[0].at("point_index").integer();
    bad_labels.obj["tangents"].arr[size_t(first)].obj["category"].scalar="irreducible-quadratic";
    must_reject([&]{cc_geometry(baseline,bad_labels,data,map);});
    auto bad_row=matrix[0];
    auto entry=std::find_if(bad_row.begin(),bad_row.end(),[](const mpz_class& value){return value!=0;});
    require(entry!=bad_row.end(),"nonzero original row");
    ++*entry;
    must_reject([&]{check_sum(bad_row);});
    auto bad_image=read_point(images.at(1).at("point"));
    require(!bad_image.infinity,"finite image control");
    bad_image.y=reduce(bad_image.y+1);
    must_reject([&]{require(same(bad_image,map.image(mpz_class(points[1].at("x").scalar),mpz_class(points[1].at("y").scalar))),"changed individual map image");});
    auto bad_proof=data.at("closure");
    auto& coefficient=bad_proof.obj["combinations"].arr[0].arr[0].scalar;
    coefficient=mpz_class(mpz_class(coefficient)+1).get_str();
    must_reject([&]{cc_closure(matrix,225,bad_proof);});
    std::ofstream out(destination);
    require(bool(out),"new production checker result");
    out << "{\"standing\":\"producer-unverified separate arithmetic\",\"point_image_checks\":" << points.size()
        << ",\"tangent_product_checks\":102,\"selection_checks\":226,\"elliptic_rows_checked\":108,\"closed_rank\":"
        << data.at("closure").at("closed_rank").integer() << ",\"closed_gain\":" << data.at("closure").at("gain").integer()
        << ",\"corruptions_rejected\":5,\"signed_independent_verdict\":false}\n";
    require(bool(out),"complete production checker result");
}
int main(int argc,char** argv) {
    try {
        if(argc==5&&std::string(argv[1])=="--self-test") {
            require(!std::ifstream(argv[4]).good(),"output already exists");
            cc_self_test(argv[2],argv[3],argv[4]);
        }
        else if(argc==8&&std::string(argv[1])=="--check") {
            require(!std::ifstream(argv[7]).good(),"output already exists");
            cc_check(argv[2],argv[3],argv[4],argv[5],argv[6],argv[7]);
        }
        else throw std::runtime_error("usage: checker --self-test FIXTURES.json PRODUCER.json OUTPUT.json | --check MODEL.json MAP.json BASELINE.json TANGENTS.json PRODUCER.json OUTPUT.json");
    } catch(const std::exception& error) { std::cerr << "REJECT " << error.what() << '\n'; return 1; }
}
