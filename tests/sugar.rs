extern crate ast_topology;
use ast_topology::autograd;

#[test]
fn xy() {
   autograd!{
      let x; let y;
      let z = 2.*x*x + 3.*y + 1.;

      assert_eq!(eval (dz/dy), Ok(3.0));
      assert_eq!(eval [x=2.] (dz/dx), Ok(8.0));
      assert_eq!(eval (ddz/dx), Ok(4.0));
   };
}
