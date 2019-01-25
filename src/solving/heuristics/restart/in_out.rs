use solving::heuristics::RestartHeuristic2;

/// This structure encapsulates the restart strategy of the solver.
/// It is implemented using picoSAT's 'in/out' restart algorithm
#[derive(Debug)]
pub struct InOut {
    inner : usize,
    outer : usize,

    /// conflict number of next restart
    conflicts  : usize
}

impl RestartHeuristic2 for InOut {
    /// Tells whether the solver should restart given it has already encountered `nb_conflicts`
    #[inline]
    fn should_restart(&self, nb_conflict: usize, _queue: &Vec<u32>) -> bool {
        nb_conflict == self.conflicts
    }

    /// Sets the next conflict limit before the next restart
    #[inline]
    fn set_next_limit(&mut self) {
        self.conflicts = self.in_out();
    }
}
impl InOut {
    /// Creates a new instance
    pub fn new() -> InOut {
        InOut {
            inner: 100,
            outer: 100,
            conflicts: 100
        }
    }

    /// This is the core of the in_out strategy
    #[inline]
    fn in_out(&mut self) -> usize {
        if self.inner >= self.outer {
            self.inner = 100;
            self.outer = (self.outer * 11)/10 as usize;
            self.conflicts = 0;
        } else {
            self.inner = (self.inner * 11)/10 as usize;
        }

        return self.inner;
    }
}


// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_out_sequence() {
        let mut tested = InOut::new();

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);
        assert_eq!(tested.in_out(), 146);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);
        assert_eq!(tested.in_out(), 146);
        assert_eq!(tested.in_out(), 160);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);
        assert_eq!(tested.in_out(), 146);
        assert_eq!(tested.in_out(), 160);
        assert_eq!(tested.in_out(), 176);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);
        assert_eq!(tested.in_out(), 146);
        assert_eq!(tested.in_out(), 160);
        assert_eq!(tested.in_out(), 176);
        assert_eq!(tested.in_out(), 193);

        assert_eq!(tested.in_out(), 100);
        assert_eq!(tested.in_out(), 110);
        assert_eq!(tested.in_out(), 121);
        assert_eq!(tested.in_out(), 133);
        assert_eq!(tested.in_out(), 146);
        assert_eq!(tested.in_out(), 160);
        assert_eq!(tested.in_out(), 176);
        assert_eq!(tested.in_out(), 193);
        assert_eq!(tested.in_out(), 212);

    }

    /*
    #[test]
    fn should_restart_follows_in_out_sequence(){
        let mut tested = In_out::new();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), true);
        assert_eq!(tested.should_restart(110), false);

        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), true);
        assert_eq!(tested.should_restart(110), false);

        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(110), true);
        assert_eq!(tested.should_restart(121), false);

        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(110), false);
        assert_eq!(tested.should_restart(121), true);
        assert_eq!(tested.should_restart(133), false);

        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(110), false);
        assert_eq!(tested.should_restart(121), false);
        assert_eq!(tested.should_restart(133), true);
        assert_eq!(tested.should_restart(146), false);

        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(110), false);
        assert_eq!(tested.should_restart(121), false);
        assert_eq!(tested.should_restart(133), false);
        assert_eq!(tested.should_restart(146), true);
        assert_eq!(tested.should_restart(160), false);

        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(110), false);
        assert_eq!(tested.should_restart(121), false);
        assert_eq!(tested.should_restart(133), false);
        assert_eq!(tested.should_restart(146), false);
        assert_eq!(tested.should_restart(160), true);
        assert_eq!(tested.should_restart(176), false);

    }*/
}
