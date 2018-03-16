#[derive(PartialEq, Clone, Debug)]
pub enum LiteralStatus {
    Pure,
    UnPure,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    id: usize,
    status: LiteralStatus,
}

impl Literal {
    pub fn new(n: isize) -> Literal {
        if n > 0 {
            Literal {
                id: n as usize,
                status: LiteralStatus::Pure,
            }
        } else if 0 > n {
            Literal {
                id: -n as usize,
                status: LiteralStatus::UnPure,
            }
        } else {
            println!("error: {}", "don't use '0'");
            ::std::process::exit(1);
        }
    }

    pub fn is_pure(&self) -> bool {
        self.status == LiteralStatus::Pure
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Clause {
    body: Vec<Literal>,
}

impl Clause {
    pub fn new(ns: &Vec<isize>) -> Clause {
        Clause {
            body: ns.iter()
                .map(|n| Literal::new(*n))
                .collect::<Vec<Literal>>(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub fn is_unit(&self) -> bool {
        self.body.len() == 1
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cnf {
    body: Vec<Clause>,
}

impl Cnf {
    pub fn new(nss: Vec<Vec<isize>>) -> Cnf {
        Cnf {
            body: nss.iter()
                .map(|ns| Clause::new(ns))
                .collect::<Vec<Clause>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_literal() {
        let pure_literal = Literal::new(1);
        assert!(pure_literal.is_pure());
    }

    #[test]
    #[should_panic]
    fn unpure_literal() {
        let unpure_literal = Literal::new(-1);
        assert!(unpure_literal.is_pure());
    }
    #[test]
    fn empty_clause() {
        let a = Clause::new(&vec![]);
        assert!(a.is_empty());
    }

    #[test]
    fn unit_clause() {
        let a = Clause::new(&vec![1]);
        assert!(a.is_unit());
    }

    #[test]
    fn show_cnf_example() {
        let a = vec![vec![1, -2, 4], vec![-1, -3], vec![3], vec![2, 3, 4]];
        println!("{:?}", Cnf::new(a));
    }
}
