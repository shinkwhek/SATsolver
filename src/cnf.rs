#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub id: usize,
    pub status: bool,
}

impl Literal {
    pub fn new(n: isize) -> Literal {
        if n > 0 {
            Literal {
                id: n as usize,
                status: true,
            }
        } else if 0 > n {
            Literal {
                id: -n as usize,
                status: false,
            }
        } else {
            println!("error: {}", "don't use '0'");
            ::std::process::exit(1);
        }
    }

    pub fn is_pure(&self) -> bool {
        self.status
    }

    pub fn get_rev_literal(&self) -> Literal {
        Literal {
            id: self.id,
            status: !self.status,
        }
    }
}

pub struct Clause {}
impl Clause {
    pub fn new(ns: &Vec<isize>) -> Vec<Literal> {
        ns.iter()
            .map(|n| Literal::new(*n))
            .collect::<Vec<Literal>>()
            .clone()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cnf {
    pub body: Vec<Vec<Literal>>,
}

impl Cnf {
    pub fn new(nss: Vec<Vec<isize>>) -> Cnf {
        Cnf {
            body: nss.iter()
                .map(|ns| Clause::new(ns))
                .collect::<Vec<Vec<Literal>>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
