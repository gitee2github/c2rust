/// Set-based constraint solver for analyses in different stages.
use crate::util::{HashMap, HashSet};
use crate::{solver::*, util::profile, Name};
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
/// A newtype for SCC IDs so that we don't confuse them with variables
/// during the rewrite.
struct SCCId(u32);

impl SCCId {
    /// Increment this ID and return the previous value. This is
    /// equivalent to `self++` in C++.
    pub fn next(&mut self) -> SCCId {
        let prev = *self;
        self.0 += 1;
        prev
    }
}

/// A set-based constraint system where `Var` is the type of
/// variables. It supports constraints of the form `t1 ⊆ t2` where
/// `t1` and `t2` are terms, that are either a variable `X`, or a
/// complex term of the form `c(X1, ..., Xn)`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConstraintSystem<Var: Eq + Hash> {
    /// A mapping from variables to SCC IDs so that we don't need to use
    /// hash maps in the constraint graph, and we can have more
    /// complex variables without much cost for hashing or comparing
    /// them.
    var_to_num: HashMap<Var, u32>,
    /// A reverse-lookup table to find the variables for a given SCC ID
    num_to_var: Vec<HashSet<Var>>,
    /// Inter-variable superset constraints (y ∈ supers[x] iff x ⊆ y)
    supers: Vec<HashSet<u32>>,
    /// Constructor lower bounds for each variable
    lower: Vec<HashSet<Ctor<u32>>>,
    /// Constructor upper bounds for each variable
    upper: Vec<HashSet<Ctor<u32>>>,
    /// Variables that should be empty
    empty: Vec<bool>,
    /// Variables that should contain everything
    all: Vec<bool>,
    /// Dirty bit to denote if the constraint system has changed in an
    /// iteration
    dirty: bool,
}

impl<V: Eq + Clone + Hash + Debug> ConstraintSystem<V> {
    pub fn new() -> Self {
        ConstraintSystem {
            var_to_num: HashMap::default(),
            num_to_var: Vec::new(),
            supers: Vec::default(),
            lower: Vec::default(),
            upper: Vec::default(),
            all: Vec::default(),
            empty: Vec::default(),
            dirty: false,
        }
    }

    pub fn lower(&self) -> &Vec<HashSet<Ctor<u32>>> {
        &self.lower
    }

    pub fn upper(&self) -> &Vec<HashSet<Ctor<u32>>> {
        &self.upper
    }

    #[must_use]
    pub fn add_goal(&mut self, goal: Constraint<V>) -> Result<(), ConstraintError<V>> {
        let num_goal = goal.repack(&mut |x| self.to_num(&x));
        self.add_num_goal(num_goal)
    }

    #[must_use]
    #[inline(always)]
    fn add_num_goal(&mut self, goal: Constraint<u32>) -> Result<(), ConstraintError<V>> {
        use SimpleTerm::*;
        use Term::*;

        // Add the goal to the constraint graph on-the-go
        match goal {
            Constraint(S(LV(x)), S(LV(y))) => {
                // Skip self-cycles (TODO: apply partial cycle detection here)
                if x == y {
                    return Ok(());
                }
                self.dirty = self.supers[x as usize].insert(y) || self.dirty;
            },
            Constraint(C(c), S(LV(x))) => {
                self.dirty = self.lower[x as usize].insert(c) || self.dirty;
            },
            Constraint(S(LV(x)), C(c)) => {
                self.dirty = self.upper[x as usize].insert(c) || self.dirty;
            },
            Constraint(C(c), C(d))
                if c.0 != d.0 || c.1.len() != d.1.len() || c.2.len() != d.2.len() =>
            {
                // Mismatches happen because of weak typing of the
                // original C program, ignore them (effectively doing
                // TBAA).
            }
            Constraint(C(Ctor(_, lower, lower_contra)), C(Ctor(_, upper, upper_contra))) => {
                for (x1, x2) in lower.into_iter().zip(upper.into_iter()) {
                    let c = Constraint(S(x1), S(x2));
                    self.add_num_goal(c)?;
                }
                for (x1, x2) in lower_contra.into_iter().zip(upper_contra.into_iter()) {
                    let c = Constraint(S(x2), S(x1));
                    self.add_num_goal(c)?;
                }
            },
            Constraint(S(LV(x)), S(EmptySet)) => {
                self.empty[x as usize] = true;
            },
            Constraint(S(All), S(LV(x))) => {
                self.all[x as usize] = true;
            },
            // These cases are identities
            Constraint(S(EmptySet), _) | Constraint(_, S(All)) => {},
            // These cases are unsatisfiable
            Constraint(S(All), S(EmptySet))
            | Constraint(C(_), S(EmptySet))
            | Constraint(S(All), C(_)) => {
                // check for these only if we are not adding back edges
                if !cfg!(feature = "add_back_edges") {
                    return Err(ConstraintError::EmptynessViolation(goal));
                }
            },
        }

        Ok(())
    }

    /// Get the number for `x`, assign it a number if one is not
    /// assigned yet. This will clone `x` if it is not assigned a
    /// number.
    pub fn to_num(&mut self, x: &V) -> u32 {
        if let Some(n) = self.var_to_num.get(x) {
            *n
        } else {
            // this is a fresh variable, so it will have its own SCC
            let next_num = self.supers.len() as u32;
            let mut h = HashSet::default();
            h.insert(x.clone());
            self.num_to_var.push(h);
            self.var_to_num.insert(x.clone(), next_num);
            self.lower.push(HashSet::default());
            self.upper.push(HashSet::default());
            self.supers.push(HashSet::default());
            self.empty.push(false);
            self.all.push(false);
            next_num
        }
    }

    fn collect_all_possible_vars(
        &self,
        mut prefixes: HashSet<Vec<SimpleTerm<V>>>,
        args: &[SimpleTerm<u32>],
    ) -> HashSet<Vec<SimpleTerm<V>>> {
        use SimpleTerm::*;

        for arg in args {
            prefixes = prefixes
                .into_iter()
                .flat_map(|v| match arg {
                    LV(arg) => self.num_to_var[*arg as usize]
                        .iter()
                        .map(|arg_var| {
                            let mut v1 = v.clone();
                            v1.push(LV(arg_var.clone()));
                            v1
                        })
                        .collect::<Vec<Vec<SimpleTerm<V>>>>(),
                    EmptySet => {
                        let mut v1 = v.clone();
                        v1.push(EmptySet);
                        vec![v1]
                    },
                    All => {
                        let mut v1 = v.clone();
                        v1.push(EmptySet);
                        vec![v1]
                    },
                })
                .collect::<HashSet<Vec<SimpleTerm<V>>>>();
        }
        prefixes
    }

    /// Convert a constructor over internally assigned numbers to a
    /// constructor over `V`
    fn repack_ctor_over_var(&self, c: Ctor<u32>) -> HashSet<Ctor<V>> {
        let all_args = self.collect_all_possible_vars(vec![vec![]].into_iter().collect(), &c.1);
        let all_contra_args =
            self.collect_all_possible_vars(vec![vec![]].into_iter().collect(), &c.2);
        assert!(
            !all_args.is_empty(),
            "arguments of constructor {:?} ended up not getting matched. num->var mapping: {:#?}",
            c,
            self.num_to_var
        );
        all_args
            .iter()
            .flat_map(|args| {
                all_contra_args
                    .clone()
                    .into_iter()
                    .map(|contra| Ctor(c.0.clone(), args.clone(), contra.clone()))
                    .collect::<Vec<Ctor<V>>>()
            })
            .collect::<HashSet<Ctor<V>>>()
    }

    #[cfg(test)]
    /// Copies and returns the set of constructors in x
    fn lower_varified(&mut self, x: &V) -> HashSet<Ctor<V>> {
        let num_x = self.to_num(x) as usize;
        self.lower[num_x]
            .iter()
            .flat_map(|c| self.repack_ctor_over_var(c.clone()))
            .collect()
    }

    #[cfg(test)]
    /// Copies and returns the set of constructors larger than x
    fn upper_varified(&mut self, x: &V) -> HashSet<Ctor<V>> {
        let num_x = self.to_num(x) as usize;
        self.upper[num_x]
            .iter()
            .flat_map(|c| self.repack_ctor_over_var(c.clone()))
            .collect()
    }

    /// Copies and returns supersets of x. This is an expensive operation
    pub fn compute_supersets(&mut self, x: &V) -> HashSet<V> {
        let num_x = self.to_num(x) as usize;
        self.supers[num_x]
            .iter()
            .flat_map(|y| self.num_to_var[*y as usize].clone())
            .collect()
    }

    /// Computes the subsets of all SCCs. This expression needs to
    /// traverse the whole graph, and it is expensive.
    pub fn compute_subsets(&self) -> Vec<HashSet<u32>> {
        let mut subsets = vec![HashSet::default(); self.supers.len()];
        for (x, ys) in self.supers.iter().enumerate() {
            ys.iter().for_each(|y| {
                subsets[*y as usize].insert(x as u32);
            });
        }
        subsets
    }

    /// Get the numerical representation of supersets of the SCC
    pub fn supersets(&self, scc: u32) -> &HashSet<u32> {
        &self.supers[scc as usize]
    }

    /// Check if x is a subset of y
    pub fn is_subset(&self, x: &V, y: &V) -> bool {
        if let (Some(n_x), Some(n_y)) = (self.var_to_num.get(x), self.var_to_num.get(y)) {
            self.supers[*n_x as usize].contains(n_y)
        } else {
            // x and y are not in the constraint system
            false
        }
    }

    pub fn num_to_var(&self) -> &Vec<HashSet<V>> {
        &self.num_to_var
    }

    pub fn vars(&self) -> Vec<V> {
        self.num_to_var.iter().flat_map(|s| s.clone()).collect()
    }

    /// Reduce the constraint graph by computing the strongly
    /// connected components and replacing the nodes with SCCs
    pub fn compute_sccs(&mut self) {
        let mut sccs: Vec<Option<SCCId>> = vec![None; self.supers.len()];
        let mut scc_contents: Vec<HashSet<u32>> = Vec::new();
        let mut stack = Vec::new();
        let mut on_stack = HashSet::default();
        let mut last_scc = SCCId(0);
        let mut lowlink = vec![None; self.supers.len()];
        for v in 0..self.supers.len() {
            if sccs[v].is_none() {
                Self::compute_scc(
                    &mut sccs,
                    &mut scc_contents,
                    &self.supers,
                    v as u32,
                    &mut stack,
                    &mut on_stack,
                    &mut last_scc,
                    &mut lowlink,
                );
            }
        }

        // Update the SCC IDs in `sccs` with a contiguous ID block
        // (`compute_scc` generates non-contiguous SCC IDs).
        for (i, vs) in scc_contents.iter().enumerate() {
            let scc_id = SCCId(i as u32);
            for v in vs {
                // update the old value left from `compute_scc`
                sccs[*v as usize] = Some(scc_id);
            }
        }

        // Build the SCC-based graph and lower/upper sets by merging
        // the entries for all variables in the SCC.
        self.supers = scc_contents
            .iter()
            .map(|vs| {
                vs.iter()
                    .flat_map(|v| {
                        self.supers[*v as usize]
                            .iter()
                            .map(|u| sccs[*u as usize].unwrap().0)
                    })
                    .collect::<HashSet<u32>>()
            })
            .collect();
        self.lower = scc_contents
            .iter()
            .map(|vs| {
                vs.iter()
                    .flat_map(|v| {
                        self.lower[*v as usize]
                            .iter()
                            .map(|ctor| ctor.clone().repack(&mut |x| sccs[x as usize].unwrap().0))
                    })
                    .collect::<HashSet<Ctor<u32>>>()
            })
            .collect();
        self.upper = scc_contents
            .iter()
            .map(|vs| {
                vs.iter()
                    .flat_map(|v| {
                        self.upper[*v as usize]
                            .iter()
                            .map(|ctor| ctor.clone().repack(&mut |x| sccs[x as usize].unwrap().0))
                    })
                    .collect::<HashSet<Ctor<u32>>>()
            })
            .collect();
        self.empty = scc_contents
            .iter()
            .map(|vs| vs.iter().any(|v| self.empty[*v as usize]))
            .collect();
        self.all = scc_contents
            .iter()
            .map(|vs| vs.iter().any(|v| self.all[*v as usize]))
            .collect();
        for (_, v) in self.var_to_num.iter_mut() {
            *v = sccs[*v as usize].unwrap().0;
        }
        self.num_to_var = scc_contents
            .iter()
            .map(|vs| {
                vs.iter()
                    .flat_map(|v| self.num_to_var[*v as usize].clone())
                    .collect()
            })
            .collect();
    }

    /// Compute an SCC that the argument belongs to using Tarjan's
    /// algorithm, this is to be called from `compute_sccs`
    ///
    /// `scc_contents` contains the actual SCCs that the parent
    /// algorithm should use. `sccs` is used for building up the IDs.
    fn compute_scc(
        sccs: &mut Vec<Option<SCCId>>,
        scc_contents: &mut Vec<HashSet<u32>>,
        graph: &Vec<HashSet<u32>>,
        v: u32,
        stack: &mut Vec<u32>,
        on_stack: &mut HashSet<u32>,
        last_scc: &mut SCCId,
        lowlink: &mut Vec<Option<SCCId>>,
    ) {
        let scc = last_scc.next();
        sccs[v as usize] = Some(scc);
        lowlink[v as usize] = Some(scc);
        stack.push(v);
        on_stack.insert(v);

        // visit successors of v and compute their SCCs
        for w in &graph[v as usize] {
            if sccs[*w as usize].is_none() {
                // recursively compute the scc for w
                Self::compute_scc(
                    sccs,
                    scc_contents,
                    graph,
                    w.clone(),
                    stack,
                    on_stack,
                    last_scc,
                    lowlink,
                );
                // update the lowlink of v, in case we ended up on a cycle through w
                lowlink[v as usize] = Some(
                    lowlink[v as usize]
                        .unwrap()
                        .min(lowlink[*w as usize].unwrap()),
                );
            } else if on_stack.contains(w) {
                // w is on the stack so it is in the current SCC (we visited it earlier!)
                lowlink[v as usize] =
                    Some(lowlink[v as usize].unwrap().min(sccs[*w as usize].unwrap()));
            }
        }

        // if v is a root node (the lowlink didn't get lowered), then
        // generate an SCC
        if lowlink[v as usize] == Some(scc) {
            let mut nodes_in_scc = HashSet::default();
            while let Some(w) = stack.pop() {
                on_stack.remove(&w);
                sccs[w as usize] = Some(scc);
                if w == v {
                    break;
                }
                nodes_in_scc.insert(w);
            }
            nodes_in_scc.insert(v);
            scc_contents.push(nodes_in_scc);
        }
    }

    /// Print statistics for debugging the solver
    fn print_stats(&self) {
        println!("# vars: {}", self.var_to_num.len());
        println!("# SCCs: {}", self.num_to_var.len());
        let super_constraints: usize = self.supers.iter().map(|s| s.len()).sum();
        let lower_constraints: usize = self.lower.iter().map(|s| s.len()).sum();
        let upper_constraints: usize = self.upper.iter().map(|s| s.len()).sum();
        let total_constraints: usize = super_constraints + lower_constraints + upper_constraints;
        println!("# constraints: {}", total_constraints);
        println!("# - var < var: {}", super_constraints);
        println!("# - ctor< var: {}", lower_constraints);
        println!("# - var <ctor: {}", upper_constraints);
    }

    /// Solve the constraint system for goals, starting from sets as
    /// the initial sets.
    pub fn solve(&mut self) -> Result<(), ConstraintError<V>> {
        use SimpleTerm::*;
        use Term::*;

        let mut iteration_count = 0;
        let mut old_scc_count = self.var_to_num.len();

        while self.dirty {
            self.dirty = false;
            if cfg!(feature = "debug_set_solver") {
                println!("iteration {}", iteration_count);
                self.print_stats();
            }

            profile("compute SCCs", || self.compute_sccs());

            if cfg!(feature = "debug_set_solver") {
                self.print_stats();
            }

            // Fine-grained dirty bits for supersets. See the comment
            // below about dirty_ctor_sets for more detail (the same
            // optimization for lower/upper). Reset these if the SCC
            // computation has collapsed any cycles.
            let mut dirty_supers = vec![
                iteration_count == 0
                    || old_scc_count != self.var_to_num.len();
                self.supers.len()
            ];

            profile("transitive closure", || {
                // Compute transitive closure of subsets
                for x in 0..self.supers.len() {
                    self.transitive_closure_from(x as u32, &mut dirty_supers);
                }
            });

            if cfg!(feature = "debug_set_solver") {
                let super_constraints: usize = self.supers.iter().map(|s| s.len()).sum();
                println!(
                    "# - var < var: {} (after transitive closure)",
                    super_constraints
                );
                println!("propagating upper and lower sets");
            }

            profile("propagating upper and lower sets", || {
                // Fine-grained dirty bits so we can skip propagation
                // along constructors, if there aren't any new ones
                // for a variable. An even more fine-grained approach
                // would be to keep track of newly-added constructors,
                // or to use an online algorithm. We treat everything
                // as dirty in the first iteration.
                let mut dirty_ctor_sets = vec![
                    iteration_count == 0
                        || old_scc_count != self.var_to_num.len();
                    self.upper.len()
                ];

                // Update upper & lower sets
                for (x, ys) in self.supers.iter().enumerate().filter(|p| dirty_supers[p.0]) {
                    for y in ys.iter() {
                        for upper in self.upper[*y as usize].clone() {
                            dirty_ctor_sets[x] = self.upper[x].insert(upper) || dirty_ctor_sets[x];
                        }
                        for lower in self.lower[x].clone() {
                            dirty_ctor_sets[*y as usize] = self.lower[*y as usize].insert(lower)
                                || dirty_ctor_sets[*y as usize];
                        }
                    }
                }
                self.dirty = self.dirty || dirty_ctor_sets.iter().any(|x| *x);

                if cfg!(feature = "debug_set_solver") {
                    println!("adding new goals from constructors");
                    self.print_stats();
                    self.constraint_breakdown();
                }

                // solve the whole system
                profile("constructor constraints", || {
                    self.add_constructor_constraints(&dirty_ctor_sets)
                })?;
                Ok(())
            })?;

            if cfg!(feature = "debug_set_solver") {
                println!("dirty after: {}", self.dirty);
            }
            iteration_count += 1;
            old_scc_count = self.var_to_num.len();
        }

        if !cfg!(feature = "add_back_edges") {
            // verify `empty set` constraints
            for (x, is_empty) in self.empty.iter().enumerate() {
                if !is_empty {
                    continue;
                }
                if self.all[x] {
                    panic!("Constraint violation: 1 ⊆ {} ⊆ 0", x);
                }
                if !self.lower[x].is_empty() {
                    return Err(ConstraintError::EmptynessViolation(Constraint(
                        C(self.lower[x].iter().next().unwrap().clone()),
                        S(EmptySet),
                    )));
                }
            }

            // verify `all` constraints
            for (x, is_empty) in self.all.iter().enumerate() {
                if !is_empty {
                    continue;
                }
                if !self.upper[x].is_empty() {
                    return Err(ConstraintError::EmptynessViolation(Constraint(
                        C(self.upper[x].iter().next().unwrap().clone()),
                        S(EmptySet),
                    )));
                }
            }
        }

        Ok(())
    }

    /// Adds goals from constructors by matching lower and upper sets
    ///
    /// ```
    ///
    /// x ⊆ c(a..)
    /// c(b..) ⊆ x
    /// ---------------
    /// c(b..) ⊆ c(a..)
    ///
    /// ```
    ///
    /// The parameter `dirty_ctor_sets` encodes whether the lower or
    /// upper sets of an SCC has changed.
    fn add_constructor_constraints(
        &mut self,
        dirty_ctor_sets: &Vec<bool>,
    ) -> Result<(), ConstraintError<V>> {
        use Term::*;
        // Add new goals from constructors
        //
        // copy the upper set so we don't cumulatively add to it in the loop
        let upper = self.upper.clone();
        for (x, lowers) in self
            .lower
            .iter()
            .enumerate()
            .filter_map(|(i, lowers)| {
                if dirty_ctor_sets[i] && !lowers.is_empty() {
                    Some((i, lowers.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, HashSet<Ctor<u32>>)>>()
        {
            for upper in &upper[x] {
                for lower in &lowers {
                    self.add_num_goal(Constraint(C(lower.clone()), C(upper.clone())))?;
                }
            }
        }
        Ok(())
    }

    /// Print a breakdown of constraints
    fn constraint_breakdown(&self) {
        let mut c2n: HashMap<Name, (usize, usize)> = HashMap::default();
        for cs in &self.upper {
            for c in cs {
                let tag = c.0.clone();
                c2n.entry(tag).or_default().1 += 1;
            }
        }
        for cs in &self.lower {
            for c in cs {
                let tag = c.0.clone();
                c2n.entry(tag).or_default().0 += 1;
            }
        }
        for (tag, (bot, top)) in &c2n {
            println!("{}\tbot: {}\ttop: {}", tag, bot, top);
        }
        let cart = c2n.values().map(|t| t.0 * t.1).sum::<usize>();
        println!("total cartesian product: {}", cart);
    }

    /// Computes the transitive closure of given variable, using the
    /// subset constraint graph represented with `self.supers`
    fn transitive_closure_from(&mut self, x: u32, dirty_supers: &mut Vec<bool>) {
        // Compute the transitive closure by DFS
        let mut visited = HashSet::default();
        visited.insert(x.clone());
        let mut worklist: Vec<u32> = self.supers[x as usize].clone().into_iter().collect();

        while let Some(y) = worklist.pop() {
            if visited.contains(&y) {
                continue;
            }

            // self.dirty = add_to_multimap(&mut self.supers, x.clone(), y.clone()) || self.dirty;
            if self.supers[x as usize].insert(y) {
                self.dirty = true;
                dirty_supers[x as usize] = true;
            }

            for z in &self.supers[y as usize] {
                if !visited.contains(z) {
                    worklist.push(z.clone());
                }
            }

            visited.insert(y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_hash_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
        v.into_iter().collect()
    }

    #[test]
    fn term_repack() {
        use SimpleTerm::*;
        use Term::*;

        let mut f = |n: u32| n.to_string();

        assert_eq!(S(LV("3".to_owned())), S(LV(3).repack(&mut f)));
        assert_eq!(
            C(Ctor::simple(
                Name::from("c"),
                vec!["3".to_owned(), "4".to_owned()],
                vec!["5".to_owned(), "6".to_owned()]
            )),
            C(Ctor::simple(Name::from("c"), vec![3, 4], vec![5, 6])).repack(&mut f)
        );
    }

    #[test]
    fn solve_idempotency() {
        use SimpleTerm::*;
        use Term::*;

        let mut sys = ConstraintSystem::new();
        sys.add_goal(Constraint(S(LV(3)), S(LV(4)))).unwrap();

        sys.solve()
            .expect("The constraint system should not have errors");

        let mut hash_set_4 = HashSet::default();
        hash_set_4.insert(4);

        assert!(sys.upper_varified(&3).is_empty());
        assert!(sys.upper_varified(&4).is_empty());
        assert!(sys.lower_varified(&3).is_empty());
        assert!(sys.lower_varified(&4).is_empty());
        assert_eq!(&sys.compute_supersets(&3), &hash_set_4);
        assert!(sys.compute_supersets(&4).is_empty());
    }

    #[test]
    fn solve_transitivity() {
        use SimpleTerm::*;
        use Term::*;

        // Testing the following set of constraints:
        //
        // 1 ⊆ 2
        // 2 ⊆ 3
        // 3 ⊆ 4
        //
        // The solution has the following sets:
        // supersets:
        // 1 -> {2, 3, 4}
        // 2 -> {3, 4}
        // 3 -> {4}
        // 4 -> {}
        //
        // lower: everything: {}
        //
        // upper: everything: {}

        let mut sys = ConstraintSystem::new();
        sys.add_goal(Constraint(S(LV(1)), S(LV(2)))).unwrap();
        sys.add_goal(Constraint(S(LV(2)), S(LV(3)))).unwrap();
        sys.add_goal(Constraint(S(LV(3)), S(LV(4)))).unwrap();

        sys.solve().unwrap();

        let supersets_1 = mk_hash_set(vec![2, 3, 4]);
        let supersets_2 = mk_hash_set(vec![3, 4]);
        let supersets_3 = mk_hash_set(vec![4]);

        assert_eq!(&sys.compute_supersets(&1), &supersets_1);
        assert_eq!(&sys.compute_supersets(&2), &supersets_2);
        assert_eq!(&sys.compute_supersets(&3), &supersets_3);
        assert!(sys.compute_supersets(&4).is_empty());

        assert!(sys.upper_varified(&1).is_empty());
        assert!(sys.upper_varified(&2).is_empty());
        assert!(sys.upper_varified(&3).is_empty());
        assert!(sys.upper_varified(&4).is_empty());

        assert!(sys.lower_varified(&1).is_empty());
        assert!(sys.lower_varified(&2).is_empty());
        assert!(sys.lower_varified(&3).is_empty());
        assert!(sys.lower_varified(&4).is_empty());
    }

    #[test]
    fn solve_constructor() {
        use SimpleTerm::*;
        use Term::*;

        // Testing the following set of constraints:
        //
        // 1 ⊆ 2
        // c(1, 2) ⊆ 1
        // 2 ⊆ c(3, 4)
        //
        // The solution has the following sets:
        // supersets:
        // 1 -> {2, 3, 4}
        // 2 -> {4}
        // 3 -> {}
        // 4 -> {}
        //
        // lower:
        // 1, 2, 3, 4 -> c(1, 2)
        //
        // upper:
        // 1, 2 -> c(3, 4)
        // everything else -> {}

        let mut sys = ConstraintSystem::new();

        let goal_set = mk_hash_set(vec![
            Constraint(S(LV(1)), S(LV(2))),
            Constraint(
                C(Ctor::simple(Name::from("c"), vec![1, 2], vec![])),
                S(LV(1)),
            ),
            Constraint(
                S(LV(2)),
                C(Ctor::simple(Name::from("c"), vec![3, 4], vec![])),
            ),
        ]);

        for goal in goal_set {
            sys.add_goal(goal).unwrap();
        }

        sys.solve().unwrap();

        let upper = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![3, 4], vec![])]);

        let lower = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![1, 2], vec![])]);

        let supersets_1 = mk_hash_set(vec![2, 3, 4]);
        let set_4 = mk_hash_set(vec![4]);

        println!("The solved system is: {:#?}", sys);

        assert_eq!(&sys.compute_supersets(&1), &supersets_1);
        assert_eq!(&sys.compute_supersets(&2), &set_4);
        assert_eq!(&sys.compute_supersets(&3), &HashSet::default());
        assert_eq!(&sys.compute_supersets(&4), &HashSet::default());

        assert_eq!(&sys.lower_varified(&1), &lower);
        assert_eq!(&sys.lower_varified(&2), &lower);
        assert_eq!(&sys.lower_varified(&3), &lower);
        assert_eq!(&sys.lower_varified(&4), &lower);

        assert_eq!(&sys.upper_varified(&1), &upper);
        assert_eq!(&sys.upper_varified(&2), &upper);
        assert!(sys.upper_varified(&3).is_empty());
        assert!(sys.upper_varified(&4).is_empty());
    }

    #[test]
    fn solve_contravariant() {
        use SimpleTerm::*;
        use Term::*;

        // Testing the following set of constraints:
        //
        // Using the notation `c(a..; b..)` to denote covariant and
        // contravariant arguments
        //
        // 2 ⊆ 1
        // 1 ⊆ c(; 1, 2)
        // c(; 3, 4) ⊆ 2
        //
        // The solution has the following sets:
        // supersets:
        // 1 -> {3}
        // 2 -> {1, 3, 4}
        // 3 -> {}
        // 4 -> {}
        //
        // lower:
        // 1, 2, 3, 4 -> c(; 3, 4)
        //
        // upper:
        // 1, 2 -> c(; 1, 2)
        // everything else -> {}

        let mut sys = ConstraintSystem::new();

        let goal_set = mk_hash_set(vec![
            Constraint(S(LV(2)), S(LV(1))),
            Constraint(
                C(Ctor::simple(Name::from("c"), vec![], vec![3, 4])),
                S(LV(2)),
            ),
            Constraint(
                S(LV(1)),
                C(Ctor::simple(Name::from("c"), vec![], vec![1, 2])),
            ),
        ]);

        for goal in goal_set {
            sys.add_goal(goal).unwrap();
        }

        sys.solve().unwrap();

        let upper = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![], vec![1, 2])]);

        let lower = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![], vec![3, 4])]);

        let supersets_2 = mk_hash_set(vec![1, 3, 4]);
        let set_3 = mk_hash_set(vec![3]);

        println!("The solved system is: {:#?}", sys);

        assert_eq!(&sys.compute_supersets(&1), &set_3);
        assert_eq!(&sys.compute_supersets(&2), &supersets_2);
        assert_eq!(&sys.compute_supersets(&3), &HashSet::default());
        assert_eq!(&sys.compute_supersets(&4), &HashSet::default());

        assert_eq!(&sys.lower_varified(&1), &lower);
        assert_eq!(&sys.lower_varified(&2), &lower);
        assert_eq!(&sys.lower_varified(&3), &lower);
        assert_eq!(&sys.lower_varified(&4), &lower);

        assert_eq!(&sys.upper_varified(&1), &upper);
        assert_eq!(&sys.upper_varified(&2), &upper);
        assert!(sys.upper_varified(&3).is_empty());
        assert!(sys.upper_varified(&4).is_empty());
    }

    #[test]
    fn solve_complex() {
        use SimpleTerm::*;
        use Term::*;

        // Testing the following set of constraints:
        //
        // 1 ⊆ 2
        // 1 ⊆ 3
        // 3 ⊆ 4
        // 1 ⊆ c(1,5)
        // c(3,5) ⊆ 2
        // c(1,4) ⊆ 1
        //
        // The solution has the following sets:
        // supersets:
        // 1 -> {2, 3, 4, 5}
        // 2 -> {}
        // 3 -> {4, 5}
        // 4 -> {5}
        // 5 -> {}
        //
        // lower:
        // 2 -> {c(1, 4), c(3, 5)}
        // everything else -> {c(1, 4)}
        //
        // upper:
        // 1 -> c(1, 5)
        // everything else -> {}

        let mut sys = ConstraintSystem::new();

        let goal_set = mk_hash_set(vec![
            Constraint(S(LV(1)), S(LV(2))),
            Constraint(S(LV(1)), S(LV(3))),
            Constraint(S(LV(3)), S(LV(4))),
            Constraint(
                S(LV(1)),
                C(Ctor::simple(Name::from("c"), vec![1, 5], vec![])),
            ),
            Constraint(
                C(Ctor::simple(Name::from("c"), vec![3, 5], vec![])),
                S(LV(2)),
            ),
            Constraint(
                C(Ctor::simple(Name::from("c"), vec![1, 4], vec![])),
                S(LV(1)),
            ),
        ]);

        for goal in goal_set {
            sys.add_goal(goal).unwrap();
        }

        sys.solve().unwrap();
        println!("The solved system is: {:#?}", sys);

        let upper_1 = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![1, 5], vec![])]);

        let lower_1 = mk_hash_set(vec![Ctor::simple(Name::from("c"), vec![1, 4], vec![])]);

        let lower_2 = mk_hash_set(vec![
            Ctor::simple(Name::from("c"), vec![1, 4], vec![]),
            Ctor::simple(Name::from("c"), vec![3, 5], vec![]),
        ]);

        let supersets_1 = mk_hash_set(vec![2, 3, 4, 5]);
        let set_45 = mk_hash_set(vec![4, 5]);
        let set_5 = mk_hash_set(vec![5]);

        assert_eq!(&sys.compute_supersets(&1), &supersets_1);
        assert!(sys.compute_supersets(&2).is_empty());
        assert_eq!(&sys.compute_supersets(&3), &set_45);
        assert_eq!(&sys.compute_supersets(&4), &set_5);
        assert!(sys.compute_supersets(&5).is_empty());

        assert_eq!(&sys.upper_varified(&1), &upper_1);
        assert!(sys.upper_varified(&2).is_empty());
        assert!(sys.upper_varified(&3).is_empty());
        assert!(sys.upper_varified(&4).is_empty());
        assert!(sys.upper_varified(&5).is_empty());
        assert_eq!(&sys.lower_varified(&1), &lower_1);
        assert_eq!(&sys.lower_varified(&2), &lower_2);
        assert_eq!(&sys.lower_varified(&3), &lower_1);
        assert_eq!(&sys.lower_varified(&4), &lower_1);
        assert_eq!(&sys.lower_varified(&5), &lower_1);
    }

    #[test]
    fn compute_sccs() {
        use std::collections::BTreeSet;

        // the constraint graph
        let supers: Vec<HashSet<u32>> = vec![
            vec![],
            vec![3],
            vec![1],
            vec![2, 4],
            vec![5],
            vec![6],
            vec![7],
            vec![8],
            vec![5, 9],
            vec![6],
        ]
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();

        let mut sccs: Vec<Option<SCCId>> = vec![None; supers.len()];
        let mut scc_contents: Vec<HashSet<u32>> = Vec::new();
        let mut stack: Vec<u32> = Vec::new();
        let mut on_stack: HashSet<u32> = HashSet::default();
        let mut last_scc = SCCId(0);
        let mut lowlink: Vec<Option<SCCId>> = vec![None; supers.len()];

        for v in 0..supers.len() {
            if sccs[v].is_none() {
                ConstraintSystem::<u32>::compute_scc(
                    &mut sccs,
                    &mut scc_contents,
                    &supers,
                    v as u32,
                    &mut stack,
                    &mut on_stack,
                    &mut last_scc,
                    &mut lowlink,
                );
            }
        }

        let expected_sccs: BTreeSet<Vec<u32>> =
            vec![vec![0], vec![1, 2, 3], vec![4], vec![5, 6, 7, 8, 9]]
                .into_iter()
                .map(|v| v.into_iter().collect())
                .collect();

        assert_eq!(
            scc_contents
                .iter()
                .map(|s| {
                    // convert to a vector then sort so we can compare the two sets of sets
                    let mut v = s.clone().into_iter().collect::<Vec<u32>>();
                    v.sort();
                    v
                })
                .collect::<BTreeSet<Vec<u32>>>(),
            expected_sccs
        );
    }
}
