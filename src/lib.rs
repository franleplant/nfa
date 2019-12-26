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

//trait States {
    //fn get_states(&self) -> Vec<u32>;
//}

//impl States for u32 {
    //fn get_states(&self) -> Vec<u32> {
        //vec![self.clone()]
    //}
//}

//impl<'a> States for &'a [u32] {
    //fn get_states(&self) -> Vec<u32> {
        //self.to_vec()
    //}
//}

trait Transition {
    fn get_current_state(&self) -> u32;
    fn get_input(&self) -> String;
    fn get_next_states(&self) -> Vec<u32>;
}

impl<'a> Transition for (u32, &'a str, u32) {
    fn get_current_state(&self) -> u32 {
        self.0.clone()
    }
    fn get_input(&self) -> String {
        self.1.to_string()
    }
    fn get_next_states(&self) -> Vec<u32> {
        vec![self.2.clone()]
    }
}

impl<'a, 'b> Transition for (u32, &'a str, &'b[u32]) {
    fn get_current_state(&self) -> u32 {
        self.0.clone()
    }
    fn get_input(&self) -> String {
        self.1.to_string()
    }
    fn get_next_states(&self) -> Vec<u32> {
        self.2.to_vec()
    }
}



//example https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton

struct NFA {
    final_states: Vec<u32>,
    transitions: HashMap<(u32, String), Vec<u32>>,
    state: Vec<u32>,
}

impl NFA {
    fn new(final_states: &[u32], transitions: &[Box<Transition>]) -> NFA
    {
        let mut map = HashMap::new();
        for t in transitions {
            map.insert((t.get_current_state(), t.get_input()), t.get_next_states());
        }

        return NFA {
            final_states: final_states.iter().cloned().collect(),
            transitions: map,
        // TODO closure of this state
            state: vec![0],
        };
    }

    fn reset_state(&mut self) {
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


    //fn consume_string(&mut self, input: &String) -> bool {
        //self.reset_state();
        //for symbol in input.chars() {
            //self.consume(&symbol.to_string());
        //}

        //return self.is_accepted();
    //}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let transitions: &[(u32, &str, &States)] =    &[
                //(0, "", &[0, 1]),
                //(1, "0", &2),
                ////(1, "1", 1),
        //];
        let m = NFA::new(
            &[1, 3],
            &[
                Box::new((0, "", &[0, 1])),
                Box::new((1, "0", &2)),
                //(1, "1", 1),
                //(2, "0", 1),
                //(2, "1", 2),
                //(3, "0", 3),
                //(3, "1", 4),
                //(4, "0", 4),
                //(4, "1", 3),
            ],
        );
        assert_eq!(2 + 2, 4);
    }
}
