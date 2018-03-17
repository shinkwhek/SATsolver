pub struct Clause;
impl Clause {
    pub fn is_empty(clause: &Vec<isize>) -> bool {
        clause.is_empty()
    }

    pub fn is_unit(clause: &Vec<isize>) -> bool {
        clause.len() == 1
    }

    pub fn assign_2(clause: &Vec<isize>, lit: isize) -> Vec<isize> {
        let mut c = clause.clone();
        c.retain(|&l| l != lit);
        c
    }
}
pub struct DPLL;

impl DPLL {
    pub fn solver(cnf: &mut Vec<Vec<isize>>, assignment: &mut Vec<isize>) -> Option<Vec<isize>> {
        let (mut c, mut a) = DPLL::unit_propagation(cnf, assignment);
        if c.is_empty() {
            Some(a)
        } else if DPLL::exists_empty_clause(&c) {
            None
        } else {
            let lit = DPLL::select(&c);
            if let Some(mut a_r) = DPLL::solver(&mut DPLL::assign(&mut c, lit), &mut a) {
                a_r.push(lit);
                return Some(a_r);
            }
            if let Some(mut a_r) = DPLL::solver(&mut DPLL::assign(&mut c, -lit), &mut a) {
                a_r.push(-lit);
                return Some(a_r);
            } else {
                return None;
            }
        }
    }

    fn assign(cnf: &Vec<Vec<isize>>, lit: isize) -> Vec<Vec<isize>> {
        let c = cnf.clone()
            .into_iter()
            .filter(|cl| !cl.into_iter().any(|l| *l == lit))
            .map(|cl| Clause::assign_2(&cl, -lit))
            .collect();
        c
    }

    fn unit_propagation(
        cnf: &mut Vec<Vec<isize>>,
        assignment: &mut Vec<isize>,
    ) -> (Vec<Vec<isize>>, Vec<isize>) {
        if let Some(lit) = DPLL::get_unit_literal(cnf) {
            assignment.push(lit);

            return DPLL::unit_propagation(&mut DPLL::assign(cnf, lit), assignment);
        } else {
            (cnf.clone(), assignment.clone())
        }
    }

    fn exists_empty_clause(cnf: &Vec<Vec<isize>>) -> bool {
        cnf.into_iter().any(|cl| cl.is_empty())
    }

    fn get_unit_literal(cnf: &mut Vec<Vec<isize>>) -> Option<isize> {
        for clause in cnf {
            if Clause::is_unit(clause) {
                let i = clause[0];
                return Some(i);
            }
        }
        return None;
    }

    fn select(cnf: &Vec<Vec<isize>>) -> isize {
        cnf[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let ns: Vec<isize> = vec![];
        assert!(Clause::is_empty(&ns));
    }
}
