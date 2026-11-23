#include "progression-barrett.hpp"
#include <iostream>
using namespace paper_ec;using namespace labelled_progression;
int main(){try{
 unsigned checks=0;
 for(const Curve c:std::vector<Curve>{{5,3,0},{7,2,3},{17,2,0},{83,31,21},{97,2,3}}){
  Arithmetic32 f(c);std::vector<Point> points{{}};for(Word x=0;x<c.p;++x)for(Word y=0;y<c.p;++y)if(on_curve(c,{x,y,false}))points.push_back({x,y,false});
  for(const auto&p:points)for(const auto&q:points){
   auto jac=f.from(p);if(jac.z){Word z=3%c.p;jac={f.mul(jac.x,f.sq(z)),f.mul(jac.y,f.mul(f.sq(z),z)),z};}
   auto actual=f.affine({f.plus(jac,q),{},f.twice(jac)});require(actual[0]==point_add(c,p,q),"mixed addition mismatch");require(actual[1].infinity,"batch identity mismatch");require(actual[2]==point_add(c,p,p),"doubling mismatch");checks+=3;
  }
 }
 const Curve large{2305843009213693951ULL,0,7};const Point g{1,4294967296ULL,false};Arithmetic field(large);
 require(on_curve(large,g),"large fixture");
 for(Word i=0;i<24;++i)for(Word j=0;j<24;++j){const auto a=point_mul(large,i,g),b=point_mul(large,j,g);auto jac=field.from(a);if(jac.z){const Word z=large.p-2;jac={field.mul(jac.x,field.sq(z)),field.mul(jac.y,field.mul(field.sq(z),z)),z};}require(field.affine({field.plus(jac,b)}).at(0)==point_add(large,a,b),"large mixed addition");++checks;}
 std::cout<<"{\"checks\":"<<checks<<",\"status\":\"passed\",\"scope\":\"independent affine group oracle; scaled Jacobians, identity, doubled and opposite points, zero batch denominators\"}\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
