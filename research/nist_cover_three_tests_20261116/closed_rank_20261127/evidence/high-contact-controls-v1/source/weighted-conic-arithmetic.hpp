#pragma once

struct WAnchor {int id,contact;};
using WPencil=std::vector<WAnchor>;
struct WCollection {
    size_t candidate_occurrences=0;
    std::map<std::string,CSection> sections;
    std::vector<int> higher_orders;
    std::vector<CV> higher_parameters;
};
struct WEngine {
    const BP& curve;
    const std::vector<CPoint>& points;
    const std::vector<audit::Point>& base;
    const SCRow& reference;
    const ::Point& kappa;
    const audit::SSL& ssl;
    std::map<int,Jet> jets;
    std::map<int,std::array<CV,9>> local_cache;
    WEngine(const BP& c,const std::vector<CPoint>& p,const std::vector<audit::Point>& b,
            const SCRow& r,const ::Point& k,const audit::SSL& s):
        curve(c),points(p),base(b),reference(r),kappa(k),ssl(s){precision=9;}
    const std::array<CV,9>& local(int id) {
        precision=9;
        const auto saved=local_cache.find(id);
        if(saved!=local_cache.end())return saved->second;
        const auto& p=points.at(size_t(id));
        const auto jet=lift(homogenize(curve),{R{K(p.value.x)},R{K(p.value.y)},R{K(1)}},2);
        jets.emplace(id,jet);std::array<CV,9> out{};
        for(size_t column=0;column<6;++column) {
            CV unit{};unit[column]=F(1);
            const auto value=evaluate(homogenize(cbp(unit),2),jet.xyz);
            for(int n=0;n<9;++n) {
                const auto v=value.at(n);require(v.p.degree()<=0,"rational weighted local coefficient");
                out[size_t(n)][column]=v.p.at(0);
            }
        }
        require(out[0]==p.monomial,"weighted constant coefficient binding");
        size_t pivot=0;while(pivot<6&&p.derivative[pivot]==F(0))++pivot;
        require(pivot<6&&out[1][pivot]!=F(0),"weighted nonzero local parameter");
        for(size_t column=0;column<6;++column)
            require(out[1][column]*p.derivative[pivot]==p.derivative[column]*out[1][pivot],
                    "weighted local first derivative proportionality");
        return local_cache.emplace(id,std::move(out)).first->second;
    }
    bool general(const WPencil& pencil)const {
        if(pencil.empty()||pencil.size()>4)return false;
        int degree=0,previous=-1;
        for(const auto& a:pencil) {
            if(a.id<=previous||a.id<0||size_t(a.id)>=points.size()||a.contact<1||a.contact>4)return false;
            degree+=a.contact;previous=a.id;
            if(a.contact>=2) {
                const auto& p=points[size_t(a.id)].value;
                const auto fx=fs_eval(curve.partial(0),p.x,p.y),fy=fs_eval(curve.partial(1),p.x,p.y);
                if(a.contact>=3) {
                    const auto restriction=ss_restrict(curve,p.x,p.y,fy,-fx);
                    if(restriction.at(2)==F(0))return false;
                }
                for(const auto& b:pencil)if(b.id!=a.id) {
                    const auto& q=points[size_t(b.id)].value;
                    if(fx*(q.x-p.x)+fy*(q.y-p.y)==F(0))return false;
                }
            }
        }
        return degree==4;
    }
    FM constraints(const WPencil& pencil) {
        require(general(pencil),"proper weighted conic base scheme");FM rows;
        for(const auto& a:pencil)for(int n=0;n<a.contact;++n) {
            const auto& v=local(a.id)[size_t(n)];rows.emplace_back(v.begin(),v.end());
        }
        require(rows.size()==4,"four weighted conic constraints");return rows;
    }
    void verify_basis(const WPencil& pencil,const std::array<CV,2>& basis) {
        for(const auto& row:constraints(pencil)) {
            CV v{};std::copy(row.begin(),row.end(),v.begin());
            for(const auto& q:basis)require(cdot(q,v)==F(0),"weighted basis incidence/contact");
        }
        FM independent;
        for(const auto& q:basis)independent.emplace_back(q.begin(),q.end());
        require(fs_rref(independent).pivots.size()==2,"weighted nullspace independence");
    }
    std::array<CV,2> basis(const WPencil& pencil) {
        const auto reduced=fs_rref(constraints(pencil));
        require(reduced.pivots.size()==4,"weighted constraint rank four");
        std::vector<size_t> free;
        for(size_t column=0;column<6;++column)
            if(std::find(reduced.pivots.begin(),reduced.pivots.end(),column)==reduced.pivots.end())free.push_back(column);
        require(free.size()==2,"weighted nullspace dimension two");std::array<CV,2> result{};
        for(size_t j=0;j<2;++j) {
            result[j][free[j]]=F(1);
            for(size_t i=0;i<4;++i)result[j][reduced.pivots[i]]=-reduced.a[i][free[j]];
        }
        verify_basis(pencil,result);return result;
    }
    int multiplicity(const CV& q,int id) {
        const auto& p=points.at(size_t(id));require(cdot(q,p.monomial)==F(0),"weighted conic incidence");
        if(cdot(q,p.derivative)!=F(0))return 1;
        const auto& coefficients=local(id);
        require(cdot(q,coefficients[1])==F(0),"weighted zero first local coefficient");
        for(size_t n=2;n<9;++n)if(cdot(q,coefficients[n])!=F(0))return int(n);
        throw std::runtime_error("weighted conic precision9exhausted;incomplete");
    }
    std::optional<CSection> section(const CV& q,const std::vector<int>& ids) {
        require(base.size()<=1024&&reference.size()<=base.size(),"weighted row width cap");
        int degree=0;std::map<int,int> support;SCRow row(base.size());::Point sum;
        for(int id:ids) {
            const int m=multiplicity(q,id);require(support.emplace(id,m).second,"distinct weighted support");degree+=m;
            const auto& p=points.at(size_t(id)).value;
            for(int n=0;n<m;++n)sum=add(sum,p.image);
            if(p.index>=0)row.at(size_t(p.index))+=m*p.sign;
        }
        require(degree<=8,"proper weighted conic intersection degree");if(degree!=8)return {};
        for(auto [id,m]:support)if(m>1) {
            precision=9;
            require(order(evaluate(homogenize(cbp(q),2),jets.at(id).xyz))==m,"weighted multiplicity original evaluator");
        }
        auto negative=kappa;if(!negative.zero)negative.y=-negative.y;
        require(add(add(sum,negative),negative).zero,"weighted actual image sum twice reference");
        for(size_t i=0;i<reference.size();++i)row[i]-=2*reference[i];
        require(norm_test::row_sum(row,base).zero&&ec_ssl_zero(ec_sparse(row),base,ssl),"weighted GMP/named image row");
        return CSection{q,std::move(support),std::move(row)};
    }
    CSection replay(const CV& q,const std::map<int,int>& support,const SCRow& row,const WPencil& origin) {
        require(general(origin)&&q==cnormalize(q),"canonical weighted replay origin/conic");
        for(const auto& a:origin)for(int n=0;n<a.contact;++n)
            require(cdot(q,local(a.id)[size_t(n)])==F(0),"weighted replay prescribed contact");
        std::vector<int> incident;
        for(size_t i=0;i<points.size();++i)if(cdot(q,points[i].monomial)==F(0))incident.push_back(int(i));
        const auto actual=section(q,incident);
        require(bool(actual)&&actual->support==support&&actual->row==row,"weighted complete replay support and row");
        return *actual;
    }
    WCollection collect(const WPencil& pencil,bool controls) {
        const auto vectors=basis(pencil);WCollection result;
        std::map<std::string,std::pair<CV,std::vector<int>>> candidates;
        auto anchor=[&](int id){return std::any_of(pencil.begin(),pencil.end(),[&](const WAnchor& a){return a.id==id;});};
        for(size_t i=0;i<points.size();++i)if(!anchor(int(i))) {
            require(cdot(vectors[0],points[i].monomial)!=F(0)||cdot(vectors[1],points[i].monomial)!=F(0),
                    "no additional weighted pencil base point in pool");
            const auto q=cthrough(vectors,points[i].monomial);
            candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{})).first->second.second.push_back(int(i));
        }
        for(const auto& a:pencil) {
            const auto q=cthrough(vectors,local(a.id)[size_t(a.contact)]);const auto key=ckey(q);
            candidates.emplace(key,std::make_pair(q,std::vector<int>{}));
            const int contact=multiplicity(q,a.id);
            require(contact>a.contact&&candidates.count(key),"weighted first unfixed higher-contact parameter");
            result.higher_orders.push_back(contact);result.higher_parameters.push_back(q);
            if(controls) {
                fs_reject([&]{(void)cthrough(vectors,local(a.id)[size_t(a.contact-1)]);});
                auto missing=candidates;missing.erase(key);
                fs_reject([&]{require(missing.count(key)==1,"missing weighted higher-contact parameter");});
            }
        }
        if(controls) {
            auto broken=vectors;broken[0][0]=broken[0][0]+F(1);
            fs_reject([&]{verify_basis(pencil,broken);});
            auto wrong=pencil;++wrong[0].contact;fs_reject([&]{(void)basis(wrong);});
        }
        result.candidate_occurrences=candidates.size();
        for(const auto& [key,value]:candidates) {
            std::vector<int> ids;for(const auto& a:pencil)ids.push_back(a.id);
            ids.insert(ids.end(),value.second.begin(),value.second.end());
            int degree=0;for(int id:ids)degree+=multiplicity(value.first,id);
            require(degree<=8,"weighted candidate intersection bound");if(degree!=8)continue;
            const auto accepted=section(value.first,ids);require(bool(accepted),"weighted full section accepted");
            for(const auto& a:pencil)require(accepted->support.at(a.id)>=a.contact,"weighted full section contact profile");
            result.sections.emplace(key,*accepted);require(result.sections.size()<=4096,"weighted accepted conic cap");
        }
        return result;
    }
    void clear_appended_cache(size_t old_points) {
        for(auto at=jets.begin();at!=jets.end();)if(size_t(at->first)>=old_points)at=jets.erase(at);else ++at;
        for(auto at=local_cache.begin();at!=local_cache.end();)if(size_t(at->first)>=old_points)at=local_cache.erase(at);else ++at;
    }
};
