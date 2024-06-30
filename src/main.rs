use vector::Vector;

mod ex00;
mod matrix;
mod vector;

fn main() {
    let u = Vector([2.2, 3.]);
    let v = Vector([5., 7.1]);
    println!("{}", u + v);
}
