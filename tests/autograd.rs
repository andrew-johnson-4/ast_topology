extern crate autograd as ag;

#[test]
fn xy() {
   ag::with(|g: &mut ag::Graph<_>| {
      let x = g.placeholder(&[]);
      let y = g.placeholder(&[]);
      let z = 2.*x*x + 3.*y + 1.;

      // dz/dy
      let gy = &g.grad(&[z], &[y])[0];
      assert_eq!(gy.eval(&[]).unwrap().first().unwrap(), &3.0);

      // dz/dx (requires to fill the placeholder `x`)
      let gx = &g.grad(&[z], &[x])[0];
      let feed = ag::ndarray::arr0(2.);
      assert_eq!(gx.eval(&[x.given(feed.view())]).unwrap().first().unwrap(), &8.0);

      // ddz/dx (differentiates `z` again)
      let ggx = &g.grad(&[gx], &[x])[0];
      assert_eq!(ggx.eval(&[]).unwrap().first().unwrap(), &4.0);
   });
}
