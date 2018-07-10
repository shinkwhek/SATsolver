type Literal = isize;
type Clause = Vec<isize>;
type Cnf = Vec<Vec<Literal>>;
type Assignment = Vec<Literal>;

pub struct DPLLClause;
impl DPLLClause {
    pub fn is_empty(clause: &Clause) -> bool {
        clause.is_empty()
    }

    pub fn is_unit(clause: &Clause) -> bool {
        clause.len() == 1
    }

    pub fn assign_clause(clause: &Clause, lit: isize) -> Clause {
        let mut c = clause.clone();
        c.retain(|&l| l != lit);
        c
    }
}

pub struct DPLL;
impl DPLL {
    pub fn solver(cnf: &mut Cnf, assignment: &mut Assignment) -> Option<Assignment> {
        let (mut c, mut a) = DPLL::unit_propagation(cnf, assignment);
        if c.is_empty() {
            Some(a)
        } else if DPLL::exists_empty_clause(&c) {
            None
        } else {
            let lit = DPLL::select(&c);
            if let Some(mut a_r) = DPLL::solver(&mut DPLL::assign(&mut c, lit), &mut a) {
                a_r.push(lit);
                Some(a_r)
            } else if let Some(mut a_r) = DPLL::solver(&mut DPLL::assign(&mut c, -lit), &mut a) {
                a_r.push(-lit);
                Some(a_r)
            } else {
                None
            }
        }
    }

    fn assign(cnf: &Cnf, lit: Literal) -> Cnf {
        cnf.clone()
            .into_iter()
            .filter(|cl| !cl.into_iter().any(|l| *l == lit))
            .map(|cl| DPLLClause::assign_clause(&cl, -lit))
            .collect()
    }

    fn unit_propagation(cnf: &mut Cnf, assignment: &mut Assignment) -> (Cnf, Assignment) {
        if let Some(lit) = DPLL::get_unit_literal(cnf) {
            assignment.push(lit);
            return DPLL::unit_propagation(&mut DPLL::assign(cnf, lit), assignment);
        } else {
            (cnf.clone(), assignment.clone())
        }
    }

    fn exists_empty_clause(cnf: &Cnf) -> bool {
        cnf.into_iter().any(|cl| cl.is_empty())
    }

    fn get_unit_literal(cnf: &mut Cnf) -> Option<Literal> {
        for clause in cnf {
            if DPLLClause::is_unit(clause) {
                let i = clause[0];
                return Some(i);
            }
        }
        None
    }

    fn select(cnf: &Cnf) -> Literal {
        cnf[0][0]
    }
}
