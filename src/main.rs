mod cnf;

fn main() {
    let a = vec![vec![1, -2, 4], vec![-1, -3], vec![3], vec![2, 3, 4]];
    println!("{:?}", cnf::Cnf::new(a));
}
