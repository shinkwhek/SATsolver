extern crate failure;
use self::failure::err_msg;
use self::failure::Error;

type Literal = isize;
type Clause = Vec<Literal>;
type Cnf = Vec<Clause>;
type Assignment = Vec<Literal>;

#[derive(Debug, PartialEq, Clone)]
pub struct Dpll {
    cnf: Cnf,
    pub ass: Assignment,
}

impl Dpll {
    pub fn new(cnf: Cnf) -> Dpll {
        Dpll {
            cnf: cnf,
            ass: Vec::new(),
        }
    }

    pub fn solver(self) -> Option<Self> {
        let dpll = self.unit_propagation();

        if dpll.is_empty() {
            return Some(dpll);
        }
        if dpll.exists_empty_clause() {
            return None;
        }

        let heuristic_lit = dpll.heuristic_select();
        if let Some(dpll_r) = dpll.assign(heuristic_lit).solver() {
            return Some(dpll_r);
        }
        if let Some(dpll_r) = dpll.assign(-heuristic_lit).solver() {
            return Some(dpll_r);
        }

        None
    }
}

impl Dpll {
    fn unit_propagation(self) -> Self {
        if let Some(lit) = self.get_unit_literal() {
            return self.assign(lit).unit_propagation();
        }
        self
    }

    fn assign(&self, lit: Literal) -> Self {
        let mut dpll = self.clone();
        dpll.ass.push(lit);
        let c = dpll
            .cnf
            .into_iter()
            .filter(|cl| !cl.iter().any(|l| *l == lit)) // delete clause whose have the lit
            .map(|cl| retain_lit(cl, -lit))             // delete literal equaled -lit
            .collect();
        dpll.cnf = c;
        dpll
    }
}

fn retain_lit(mut cl: Clause, lit: Literal) -> Clause {
    cl.retain(|&l| l != lit);
    cl
}

impl Dpll {
    fn exists_empty_clause(&self) -> bool {
        self.cnf.iter().any(|cl| cl.is_empty())
    }

    fn get_unit_literal(&self) -> Option<Literal> {
        if let Some(a) = self.cnf.iter().filter(|&cl| cl.len() == 1).next() {
            return Some(a[0]);
        }
        None
    }

    fn heuristic_select(&self) -> Literal {
        self.cnf[0][0]
    }

    fn is_empty(&self) -> bool {
        self.cnf.is_empty()
    }
}

#[test]
fn solver_test_1() {
    let c = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let d = Dpll::new(c);
    assert_eq!(
        d.solver(),
        Some(Dpll {
            cnf: vec![],
            ass: vec![1, 4, 7],
        })
    );
}

#[test]
fn solver_test_2() {
    let c = vec![vec![1, 2], vec![1, -2], vec![-1, 2], vec![-1, -2]];
    let d = Dpll::new(c);
    assert_eq!(d.solver(), None);
}

#[test]
fn unit_propagation_test_1() {
    let c = vec![vec![1], vec![1, 2, 3], vec![-1, 2, 3], vec![1, -1, 2]];
    let d = Dpll::new(c);
    assert_eq!(
        d.unit_propagation(),
        Dpll {
            cnf: vec![vec![2, 3]],
            ass: vec![1],
        }
    );
}

#[test]
fn unit_propagation_test_2() {
    let c = vec![
        vec![1],
        vec![2],
        vec![1, 2, 3],
        vec![-1, 2, 3],
        vec![-1, -2, 3, 4],
        vec![1, -1, 2],
    ];
    let d = Dpll::new(c);
    assert_eq!(
        d.unit_propagation(),
        Dpll {
            cnf: vec![vec![3, 4]],
            ass: vec![1, 2],
        }
    );
}

#[test]
fn assign_test() {
    let c = vec![vec![1, 2, 3], vec![-1, 2, 3], vec![1, -1, 2]];
    let d = Dpll::new(c);
    assert_eq!(d.assign(1).cnf, vec![vec![2, 3]]);
}

#[test]
fn retain_lit_test() {
    let cl = vec![1, 2, 3];
    assert_eq!(retain_lit(cl, 1), vec![2, 3]);
}

#[test]
fn exists_empty_clause_test() {
    let c = vec![vec![1, 2], vec![], vec![3, 4]];
    let d = Dpll::new(c);
    assert_eq!(d.exists_empty_clause(), true);
    let c2 = vec![vec![1, 2], vec![3, 4]];
    let d2 = Dpll::new(c2);
    assert_eq!(d2.exists_empty_clause(), false);
}

#[test]
fn get_unit_literal_test() {
    let c = vec![vec![1, 2, 3], vec![9], vec![1, 2]];
    let d = Dpll::new(c);
    assert_eq!(d.get_unit_literal(), Some(9));
    let c2 = vec![vec![1, 2], vec![3, 4]];
    let d2 = Dpll::new(c2);
    assert_eq!(d2.get_unit_literal(), None);
}

#[test]
fn heuristic_select_test() {
    let c = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let d = Dpll::new(c);
    assert_eq!(d.heuristic_select(), 1);
}

#[test]
fn is_empty_test() {
    let c = vec![];
    let d = Dpll::new(c);
    assert_eq!(d.is_empty(), true);
    let c = vec![vec![]];
    let d = Dpll::new(c);
    assert_eq!(d.is_empty(), false);
}
