//struct StateVec {
//vec: Vec<u32>,
//}

//impl StateVec {
//fn vec(self) -> Vec<u32> {
//return self.vec;
//}
//}

//impl<'a> From<&'a [u32]> for StateVec {
//fn from(slice: &'a [u32]) -> Self {
//StateVec {
//vec: slice.to_vec(),
//}
//}
//}

//impl From<u32> for StateVec {
//fn from(value: u32) -> Self {
//StateVec { vec: vec![value] }
//}
//}
use std::collections::HashMap;

//example https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton

struct NFA {
    final_states: Vec<u32>,
    transitions: HashMap<(u32, String), Vec<u32>>,
    state: Vec<u32>,
}

impl NFA {
    fn new<A>(final_states: &[u32], transitions: &[(u32, A, Vec<u32>)]) -> NFA
    where
        A: Into<String> + Copy,
    {
        let mut map = HashMap::new();
        for &(state, symbol, ref next_states) in transitions {
            map.insert((state, symbol.into()), next_states.clone());
        }

        return NFA {
            final_states: final_states.iter().cloned().collect(),
            transitions: map,
        // TODO closure of this state
            state: vec![0],
        };
    }

    fn reset_State(&mut self) {
        // TODO closure of this state
        self.state = vec![0]
    }

    //TODO this does not contemplate lambda transitions
    //we will need to compute a lambda closure XD
    fn consume(&mut self, input: &String) -> Vec<u32> {
        let mut next_states = vec![];

        self.state
            .iter()
            .map(|state| return self.transitions.get(&(state.clone(), input.clone())))
            .filter(|ns| ns.is_some())
            //TODO maybe use flat_map here
            .map(|ns| ns.unwrap())
            .map(|ns| {
                //TODO closure of each state in ns
                for state in ns {
                    next_states.push(state.clone())
                }
            })
            .collect::<Vec<()>>();

        self.state = next_states.clone();
        return next_states;
    }

    //TODO is accepted is any of the states is a final one
    fn is_accepted(&self) -> bool {
        false
    }


    fn consume_string(&self, input: &String) -> bool {
        self.reset_state();
        for symbol in input.chars() {
            self.consume(symbol)
        }

        return self.is_accepted();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let m = NFA::new(
            &[1, 3],
            &[
                (0, "", vec![0, 1]),
                (1, "0", vec![2]),
                (1, "1", vec![1]),
                (2, "0", vec![1]),
                (2, "1", vec![2]),
                (3, "0", vec![3]),
                (3, "1", vec![4]),
                (4, "0", vec![4]),
                (4, "1", vec![3]),
            ],
        );
        assert_eq!(2 + 2, 4);
    }
}
