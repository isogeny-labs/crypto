// Point-only cold control. No outer enumeration/reference routines are called.
#define AUXILIARY_NO_MAIN
#include "auxiliary.cpp"
namespace auxiliary {
struct GraphOracle:algebraic::Oracle {
    const Tower&tower;
    GraphOracle(std::filesystem::path exe,std::filesystem::path root,const Tower&t)
        :algebraic::Oracle(std::move(exe),std::move(root)),tower(t){}
    OracleResult solve(coeff::Config cfg){
        const auto dir=root/("call-"+std::to_string(++calls));check(std::filesystem::create_directory(dir),"reused auxiliary cold call");
        const Eliminated gen(cfg,3,tower);const auto input=dir/"input.ms",gb=dir/"basis.gb";
        canonical::write_system(input,gen.system,false);const int status=backend(exe,input,gb,dir);
        {std::ofstream out(dir/"exit.txt");out<<status<<'\n';}
        check(status==0,"auxiliary cold backend unknown "+std::to_string(status));
        OracleResult out;out.extraction=quotient::Extractor(gen.system,quotient::read_basis(gb,gen.system),128).run();
        for(const auto&point:out.extraction.points)check(out.models.insert(gen.decode(point)).second,"duplicate cold witness");
        std::ofstream models(dir/"models.tsv");for(const auto&model:out.models)models<<model<<'\n';
        return out;
    }
};
ColdResult cold(Point Q,std::uint64_t seed,GraphOracle&oracle,const Tower&tower){
    const reference::Curve E{19,0,2};const Point P{4,3,false};
    check(!Q.infinity&&Q.x>=0&&Q.x<19&&Q.y>=0&&Q.y<19&&E.contains(Q),"canonical auxiliary cold target");
    check(reference::isprime(13)&&E.contains(P)&&!P.infinity&&multiply(E,P,13).infinity&&multiply(E,Q,13).infinity,"order13 admission");
    nmod_poly_t f,x,power,difference,gcd;
    for(auto*poly:{&f,&x,&power,&difference,&gcd})nmod_poly_init(*poly,19);
    nmod_poly_set_coeff_ui(f,0,2);nmod_poly_set_coeff_ui(f,3,1);nmod_poly_set_coeff_ui(x,1,1);
    nmod_poly_powmod_ui_binexp(power,x,19,f);nmod_poly_sub(difference,power,x);nmod_poly_gcd(gcd,difference,f);
    const bool no_two_torsion=nmod_poly_length(gcd)==1&&nmod_poly_get_coeff_ui(gcd,0)==1;
    for(auto*poly:{&f,&x,&power,&difference,&gcd})nmod_poly_clear(*poly);
    check(no_two_torsion&&8*8<=4*19&&4*19<9*9,"odd group/Hasse certificate");
    // Hasse[12,28], thirteen divides the order and no rational2-torsion:
    // the sole odd multiple of13 in that interval is13.
    check(((12+12)/13)==1&&28/13==2&&13%2==1&&26%2==0,"Hasse interval arithmetic");
    const auto abscissae=tower.fibre();std::vector<Point> base;
    for(int x0:abscissae){const int rhs=E.residue(x0*x0*x0+2);
        for(int y:roots({E.residue(-rhs),0,1},19))if(y<=9){const Point point{x0,y,false};
            check(E.contains(point)&&multiply(E,point,13).infinity,"auxiliary base subgroup");base.push_back(point);}}
    check(base.size()==3,"auxiliary cold base count");
    {
        std::ofstream setup(oracle.root/"cold-setup.tsv");setup<<"order_admission\t13P=O_Hasse12_28_gcd_no2torsion\n";
        setup<<"abscissa_roots\t"<<abscissae.size()<<'\n';for(const auto&point:base)setup<<"base\t"<<point.x<<'\t'<<point.y<<'\n';
    }
    return cold_collect(E,P,Q,seed,13,base,3,tower.terminal,oracle);
}
} // namespace auxiliary
#ifndef AUXILIARY_COLD_NO_MAIN
int main(int argc,char**argv){
    try{
        if(argc!=3)throw std::invalid_argument("usage: cold BACKEND NEW_ROOT");
        const auto root=std::filesystem::absolute(argv[2]);auxiliary::check(std::filesystem::create_directory(root),"reused auxiliary cold root");
        const auxiliary::Tower tower;auxiliary::GraphOracle oracle(argv[1],root/"target-x18-y1",tower);
        const auto out=auxiliary::cold({18,1,false},1234,oracle,tower);
        std::ofstream summary(root/"summary.tsv");summary<<"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank\n";
        summary<<"18\t1\t"<<out.scalar<<'\t'<<out.attempts<<'\t'<<out.calls<<'\t'<<out.misses<<'\t'<<out.rank<<'\n';summary.close();
        auxiliary::check(bool(summary),"auxiliary cold summary write");
        std::cout<<"PASS auxiliary point-only cold scalar="<<out.scalar<<" attempts="<<out.attempts<<" calls="<<out.calls<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
#endif
