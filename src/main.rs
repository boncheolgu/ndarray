
extern crate ndarray;
extern crate debug;

use ndarray::{Array, Si, S};

use ndarray::Dimension;

fn main()
{

    let mut xs = Array::<f32, _>::zeros((2u, 4u, 2u));
    //*xs.at_mut((0, 0, 0)).unwrap() = 2.2f32;
    for (i, elt) in xs.iter_mut().enumerate() {
        *elt = i as f32;
    }

    println!("{}", xs);
    println!("{}", xs.slice([S, Si(0,None,2), Si(0,None,-1)]));

    let mut rm = Array::zeros((2u,3u));
    rm[(0, 0)] = 2.2f32;
    for (i, elt) in rm.iter_mut().enumerate() {
        *elt = i as f32 / 10.0;
    }
    let mm = rm.clone();
    rm[(0, 1)] = -1.0;

    println!("{}", rm);
    println!("{}", rm.slice([Si(0, None, -1), Si(0, None, 2)]));
    println!("{}", mm);
    println!("Diagonal={}", rm.diag());
    rm.iadd(&mm);
    println!("Added=\n{}", rm);
    let snd = rm.slice([S, Si(1, None, 1)]);
    println!("Snd=\n{}", snd);
    println!("Snd Reshape={}", snd.reshape(4u));

    let sl = rm.slice([Si(0, None, -1), Si(0, None, -1)]);
    //println!("{}", rm);
    let mut tm = sl.reshape((3u, 2u));
    println!("sl={}, tm={}", sl, tm);
    //println!("{}, {}", sl.data, tm.data);


    for elt in tm.slice_iter_mut([S, Si(0, None, 2)]) {
        *elt = -3.0;
    }
    println!("{}", tm);
    println!("{}", tm + tm);
    tm.imul(&sl.reshape((3u, 2u)));
    println!("{}", tm);

    println!("{}", tm.reshape(6u).slice([Si(0,None,-3)]));
    let mut x = tm.reshape(6u).slice([Si(0,None,-3)]);
    println!("{}", x);
    x.at_mut(0);

    //x[0] = 1.0;
    println!("{}", x);

    let mut m = Array::<f32,_>::zeros(());
    m[()] = 1729.0;
    println!("{}", m);
    println!("{}", m.reshape(1u));
    println!("{}", m + m);

    let ar = Array::from_iter(range(0.0f32, 32.)).reshape((2u,4u,4u));
    println!("{},\n{}", ar, ar * ar);
    println!("{}", ar.subview(0, 1));
    let sub = ar.subview(2, 1);
    println!("sub shape={}, sub={}", sub.shape(), sub);
    let mut mat = Array::from_iter(range(0.0f32, 16.0)).reshape((2u, 4u, 2u));
    println!("{}", mat);
    //println!("{}", mat.subview(0,0));
    //println!("{}", mat.subview(0,1));
    println!("{} times \n{},=\n{}", mat.subview(2, 1), mat.subview(0, 1), mat.subview(2,1).mat_mul(&mat.subview(0,1)));
    println!("{}", mat.subview(1,1).subview(0,1));

    let u = Array::from_iter(range(0,10i)).slice([Si(0,None,-2)]);
    println!("{}", u);

    type D3 = (uint, uint, uint);
    let a_dim = (2u, 4u, 2u, 2u);
    let b_dim = (2u, 1u, 2u, 1u);
    let mut a = Array::from_iter(range(0.0, a_dim.size() as f32)).reshape(a_dim);
    let b = Array::from_iter(range(0.0, b_dim.size() as f32)).reshape(b_dim);

    let ad = a.dim();
    for (x, y) in a.iter_mut().zip(b.broadcast_iter(ad).unwrap()) {
        *x *= *y;
    }
    println!("{}\n{}", a, b);
}
