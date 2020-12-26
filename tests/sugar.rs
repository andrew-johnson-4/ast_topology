extern crate ast_topology;
use ast_topology::autograd;

#[test]
fn xy() {
   autograd!{
      let x; let y;
      let z = 2.*x*x + 3.*y + 1.;

      assert_eq!(grad (dz/dy), 3.0);
      assert_eq!(grad [x=2.] (dz/dx), 8.0);
      assert_eq!(grad (ddz/dx), 4.0);
   };
}
