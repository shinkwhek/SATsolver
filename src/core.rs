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

    pub fn is_empty(ns: &Vec<Literal>) -> bool {
        ns.is_empty()
    }

    pub fn find(ns: &Vec<Literal>, lit: Literal) -> Option<&Literal> {
        ns.into_iter().find(|l| *l == &lit)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct DPLL {
    pub cnf: Vec<Vec<Literal>>,
}

impl DPLL {
    pub fn new(nss: Vec<Vec<isize>>) -> DPLL {
        DPLL {
            cnf: nss.iter()
                .map(|ns| Clause::new(ns))
                .collect::<Vec<Vec<Literal>>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
