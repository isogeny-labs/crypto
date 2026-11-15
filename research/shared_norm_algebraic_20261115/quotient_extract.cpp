// Exact rational-point extraction from a bounded grevlex quotient algebra.
// Uses successive coordinate eigenspaces, not a separating linear form.
// The supplied GB's provenance in the original ideal is a backend obligation.
#define main legacy_canonical_cli_main
#include "../prime_one_shot_20261029/continuation/canonical/canonicalize.cpp"
#undef main
#include <flint/nmod_mat.h>
#include <flint/nmod_poly.h>
#include <flint/nmod_poly_factor.h>
#include <memory>

namespace quotient {
using canonical::U;
using canonical::Monomial;
using canonical::Polynomial;
using canonical::System;
using Matrix = std::vector<std::vector<U>>;

struct Mat {
    nmod_mat_t value;
    Mat(std::size_t rows, std::size_t columns, U p) {
        nmod_mat_init(value, static_cast<slong>(rows), static_cast<slong>(columns), p);
    }
    ~Mat() { nmod_mat_clear(value); }
    Mat(const Mat&) = delete;
    Mat& operator=(const Mat&) = delete;
    U& at(std::size_t row, std::size_t column) { return nmod_mat_entry(value,row,column); }
};
U power(U x, U e, U p) {
    U out=1;
    for(;e;e>>=1,x=x*x%p) if(e&1U)out=out*x%p;
    return out;
}
bool greater(const Monomial&a,const Monomial&b) {
    const auto da=canonical::degree(a),db=canonical::degree(b);
    if(da!=db)return da>db;
    for(std::size_t j=a.size();j-->0;)if(a[j]!=b[j])return a[j]<b[j];
    return false;
}
Monomial lead(const Polynomial&f) {
    if(f.terms.empty())throw std::runtime_error("zero leading polynomial");
    auto out=f.terms.begin()->first;
    for(const auto&term:f.terms)if(greater(term.first,out))out=term.first;
    return out;
}
bool divides(const Monomial&a,const Monomial&b) {
    for(std::size_t j=0;j<a.size();++j)if(a[j]>b[j])return false;
    return true;
}
std::vector<Polynomial> read_basis(const std::filesystem::path&file,const System&input) {
    if(std::filesystem::file_size(file)>16*1024*1024)throw std::runtime_error("basis byte cap");
    std::ifstream stream(file);
    std::string line,body;
    bool field=false,order=false,variables=false;
    while(std::getline(stream,line)) {
        if(line.rfind("#field characteristic:",0)==0)
            field=canonical::unsigned_decimal(canonical::trim(line.substr(22)))==input.p;
        else if(line.rfind("#variable order:",0)==0)
            variables=canonical::split_nonempty(canonical::trim(line.substr(16)),',')==input.names;
        else if(line.rfind("#monomial order:",0)==0)
            order=line.find("graded reverse lexicographical")!=std::string::npos;
        else if(!line.empty()&&line[0]!='#')body+=line;
    }
    if(!field||!order||!variables)throw std::runtime_error("basis metadata mismatch");
    const auto left=body.find('['),right=body.rfind(']');
    if(left==std::string::npos||right==std::string::npos||right<=left)
        throw std::runtime_error("invalid basis container");
    auto tail=canonical::trim(body.substr(right+1));
    if(!tail.empty()&&tail!=":")throw std::runtime_error("trailing basis data");
    const auto inside=canonical::trim(body.substr(left+1,right-left-1));
    std::vector<Polynomial> out;
    if(inside.empty())return out;
    for(const auto&expression:canonical::split_nonempty(inside,',')) {
        auto f=canonical::Parser{expression,input.variables,input.p,input.names.size()}.parse();
        if(!f.terms.empty())out.push_back(std::move(f));
    }
    return out;
}

struct Result {
    std::size_t dimension=0,normal_form_reductions=0,eigenspace_splits=0;
    std::vector<std::vector<U>> points;
};
class Extractor {
    const System& input;
    const std::vector<Polynomial>& basis;
    std::size_t limit;
    U p;
    std::size_t n;
    std::vector<Monomial> leaders,stair;
    std::map<Monomial,std::size_t> index;
    std::vector<std::unique_ptr<Mat>> operators;
    Result result;

    Polynomial normal_form(Polynomial f) {
        Polynomial out(n,p);
        while(!f.terms.empty()) {
            if(++result.normal_form_reductions>10000000)throw std::runtime_error("normal-form cap");
            const auto m=lead(f);
            const auto coefficient=f.terms.at(m);
            std::size_t g=0;
            while(g<basis.size()&&!divides(leaders[g],m))++g;
            if(g==basis.size()) { out.insert(m,coefficient);f.terms.erase(m);continue; }
            const auto scalar=coefficient*power(basis[g].terms.at(leaders[g]),p-2,p)%p;
            Monomial difference(n);
            for(std::size_t j=0;j<n;++j)difference[j]=m[j]-leaders[g][j];
            for(const auto&term:basis[g].terms) {
                Monomial exponent=term.first;
                for(std::size_t j=0;j<n;++j)exponent[j]+=difference[j];
                f.insert(exponent,(p-scalar*term.second%p)%p);
            }
        }
        return out;
    }
    Mat& coordinate(std::size_t j) {
        if(operators[j])return *operators[j];
        const auto delta=stair.size();
        auto op=std::make_unique<Mat>(delta,delta,p);
        for(std::size_t s=0;s<delta;++s) {
            Polynomial f(n,p);
            auto exponent=stair[s];++exponent[j];f.insert(exponent,1);
            const auto nf=normal_form(std::move(f));
            for(const auto&term:nf.terms) {
                auto position=index.find(term.first);
                if(position==index.end())throw std::runtime_error("nonstandard normal form");
                // Transpose of multiplication: acts on evaluation functionals.
                op->at(s,position->second)=term.second;
            }
        }
        operators[j]=std::move(op);
        return *operators[j];
    }
    void singleton(Mat&W) {
        if(W.at(0,0)==0)throw std::runtime_error("character vanishes at one");
        const U inverse=power(W.at(0,0),p-2,p);
        std::vector<U> point(n);
        for(std::size_t j=0;j<n;++j) {
            Mat image(stair.size(),1,p);
            nmod_mat_mul(image.value,coordinate(j).value,W.value);
            point[j]=image.at(0,0)*inverse%p;
            for(std::size_t row=0;row<stair.size();++row)
                if(image.at(row,0)!=point[j]*W.at(row,0)%p)
                    throw std::runtime_error("coordinate operators lack a common character");
        }
        for(const auto&f:input.equations)if(f.evaluate(point)!=0)
            throw std::runtime_error("basis point violates original input");
        for(const auto&f:basis)if(f.evaluate(point)!=0)
            throw std::runtime_error("basis point violates basis");
        result.points.push_back(std::move(point));
        if(result.points.size()>limit)throw std::runtime_error("point-output cap");
    }
    void split(std::size_t j,Mat&W) {
        const auto delta=stair.size(),k=static_cast<std::size_t>(W.value->c);
        if(k==1){singleton(W);return;}
        if(j==n)throw std::runtime_error("joint character dimension exceeds one");
        Mat image(delta,k,p);
        nmod_mat_mul(image.value,coordinate(j).value,W.value);
        Mat row_basis(k,delta,p);
        nmod_mat_transpose(row_basis.value,W.value);
        if(nmod_mat_rref(row_basis.value)!=static_cast<slong>(k))
            throw std::runtime_error("dependent eigenspace basis");
        std::vector<std::size_t> rows;
        for(std::size_t row=0;row<k;++row) {
            std::size_t col=0;while(col<delta&&row_basis.at(row,col)==0)++col;
            if(col==delta)throw std::runtime_error("missing row pivot");
            rows.push_back(col);
        }
        Mat restriction(k,k,p),pivot(k,k,p),inverse(k,k,p),values(k,k,p);
        for(std::size_t row=0;row<k;++row)for(std::size_t col=0;col<k;++col) {
            pivot.at(row,col)=W.at(rows[row],col);
            values.at(row,col)=image.at(rows[row],col);
        }
        if(!nmod_mat_inv(inverse.value,pivot.value))throw std::runtime_error("singular pivot minor");
        nmod_mat_mul(restriction.value,inverse.value,values.value);
        Mat replay(delta,k,p);
        nmod_mat_mul(replay.value,W.value,restriction.value);
        if(!nmod_mat_equal(replay.value,image.value))
            throw std::runtime_error("noninvariant coordinate eigenspace");
        nmod_poly_t characteristic;
        nmod_poly_init(characteristic,p);
        nmod_mat_charpoly(characteristic,restriction.value);
        nmod_poly_factor_t roots;
        nmod_poly_factor_init(roots);
        nmod_poly_roots(roots,characteristic,0);
        std::vector<U> eigenvalues;
        for(slong i=0;i<roots->num;++i) {
            const auto&f=roots->p[i];
            if(f.length!=2)throw std::runtime_error("nonlinear root factor");
            eigenvalues.push_back((p-f.coeffs[0])*power(f.coeffs[1],p-2,p)%p);
        }
        nmod_poly_factor_clear(roots);
        nmod_poly_clear(characteristic);
        std::sort(eigenvalues.begin(),eigenvalues.end());
        for(U root:eigenvalues) {
            Mat equation(delta,k,p),kernel(k,k,p);
            for(std::size_t row=0;row<delta;++row)for(std::size_t col=0;col<k;++col)
                equation.at(row,col)=(image.at(row,col)+p-root*W.at(row,col)%p)%p;
            const auto dim=nmod_mat_nullspace(kernel.value,equation.value);
            if(dim<=0)throw std::runtime_error("characteristic root has empty kernel");
            Mat coefficients(k,dim,p),child(delta,dim,p);
            for(std::size_t row=0;row<k;++row)for(slong col=0;col<dim;++col)
                coefficients.at(row,col)=kernel.at(row,col);
            nmod_mat_mul(child.value,W.value,coefficients.value);
            ++result.eigenspace_splits;
            split(j+1,child);
        }
    }
public:
    Extractor(const System&s,const std::vector<Polynomial>&g,std::size_t cap)
        :input(s),basis(g),limit(cap),p(s.p),n(s.names.size()),operators(n) {
        if(cap==0||cap>4096)throw std::runtime_error("invalid staircase cap");
        for(const auto&f:basis) {
            if(f.n!=n||f.p!=p||f.terms.empty())throw std::runtime_error("basis shape mismatch");
            leaders.push_back(lead(f));
        }
    }
    Result run() {
        for(const auto&m:leaders)if(canonical::degree(m)==0)return result;
        stair.push_back(Monomial(n,0));index.emplace(stair[0],0);
        for(std::size_t head=0;head<stair.size();++head)for(std::size_t j=0;j<n;++j) {
            auto exponent=stair[head];++exponent[j];
            if(index.count(exponent))continue;
            bool standard=true;
            for(const auto&m:leaders)if(divides(m,exponent)){standard=false;break;}
            if(!standard)continue;
            if(stair.size()==limit)throw std::runtime_error("staircase cap");
            index.emplace(exponent,stair.size());stair.push_back(std::move(exponent));
        }
        result.dimension=stair.size();
        Mat initial(stair.size(),stair.size(),p);
        nmod_mat_one(initial.value);
        split(0,initial);
        std::sort(result.points.begin(),result.points.end());
        if(std::adjacent_find(result.points.begin(),result.points.end())!=result.points.end())
            throw std::runtime_error("duplicate quotient character");
        return result;
    }
};

System fixture(U p,std::vector<std::string>names,std::vector<std::string>expressions) {
    System s;s.p=p;s.names=std::move(names);
    for(std::size_t j=0;j<s.names.size();++j)s.variables.emplace(s.names[j],j);
    for(const auto&f:expressions)s.equations.push_back(canonical::Parser{f,s.variables,p,s.names.size()}.parse());
    return s;
}
void selftest() {
    auto grid=fixture(5,{"x","y"},{"x^5-x","y^5-y"});
    auto out=Extractor(grid,grid.equations,128).run();
    if(out.dimension!=25||out.points.size()!=25)throw std::runtime_error("dense-grid extraction");
    for(U x=0;x<5;++x)for(U y=0;y<5;++y)
        if(!std::binary_search(out.points.begin(),out.points.end(),std::vector<U>{x,y}))
            throw std::runtime_error("missing dense-grid point");
    auto nonreduced=fixture(5,{"x","y"},{"x^2","y^2"});
    out=Extractor(nonreduced,nonreduced.equations,128).run();
    if(out.dimension!=4||out.points!=std::vector<std::vector<U>>{{0,0}})
        throw std::runtime_error("nonreduced extraction");
    auto extension=fixture(3,{"x"},{"x^2+1"});
    if(!Extractor(extension,extension.equations,128).run().points.empty())
        throw std::runtime_error("extension-only extraction");
    auto unit=fixture(5,{"x"},{"1"});
    if(Extractor(unit,unit.equations,128).run().dimension!=0)
        throw std::runtime_error("unit-ideal extraction");
    bool rejected=false;
    try{(void)Extractor(grid,grid.equations,24).run();}catch(const std::runtime_error&){rejected=true;}
    if(!rejected)throw std::runtime_error("staircase cap accepted");
    auto bad=fixture(5,{"x","y"},{"x^2-y","x*y-1","y^2"});
    rejected=false;
    try{(void)Extractor(bad,bad.equations,128).run();}catch(const std::runtime_error&){rejected=true;}
    if(!rejected)throw std::runtime_error("inconsistent quotient operators accepted");
}
} // namespace quotient

#ifndef QUOTIENT_EXTRACT_NO_MAIN
int main(int argc,char**argv) {
    try {
        if(argc==2&&std::string(argv[1])=="--selftest") {
            quotient::selftest();std::cout<<"PASS quotient extraction: dense grid, nonreduced, extension, unit, cap, bad operators\n";return 0;
        }
        if(argc!=4)throw std::invalid_argument("usage: quotient_extract --extract INPUT.ms BASIS.gb");
        if(std::string(argv[1])!="--extract")throw std::invalid_argument("invalid extraction command");
        const auto s=canonical::read_system(argv[2]);
        const auto basis=quotient::read_basis(argv[3],s);
        const auto out=quotient::Extractor(s,basis,128).run();
        std::cout<<"{\"quotient_dimension\":"<<out.dimension<<",\"points\":[";
        for(std::size_t i=0;i<out.points.size();++i) {
            if(i)std::cout<<',';
            std::cout<<'[';
            for(std::size_t j=0;j<out.points[i].size();++j){if(j)std::cout<<',';std::cout<<out.points[i][j];}
            std::cout<<']';
        }
        std::cout<<"],\"normal_form_reductions\":"<<out.normal_form_reductions<<",\"eigenspace_splits\":"<<out.eigenspace_splits<<"}\n";
        return 0;
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
#endif
