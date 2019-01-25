use solving::heuristics::RestartHeuristic2;

/// This structure encapsulates the restart strategy of the solver.
/// It is implemented using D.Knuth's 'reluctant doubling' algorithm
/// to generate luby sequence in $$\theta(1)$$ [time and space]
#[derive(Debug)]
pub struct Luby {
    /// the tuple from the reluctant doubling algorithm
    u : isize,
    v : isize,

    /// the length of an unit run
    unit  : usize,
    /// conflict limit = 2^shift * unit
    shift : usize
}

impl RestartHeuristic2 for Luby {
    /// Tells whether the solver should restart given it has already encountered `nb_conflicts`
    #[inline]
    fn should_restart(&self, nb_conflict: usize, _queue: &Vec<u32>) -> bool {
        nb_conflict > (self.unit << self.shift)
    }

    /// Sets the next conflict limit before the next restart
    #[inline]
    fn set_next_limit(&mut self) {
        self.shift = self.luby();
    }
}

impl Luby {
    /// Creates a new instance having the given unit run
    pub fn new(unit: usize) -> Luby {
        Luby {
            u: 1,
            v: 1,
            shift: 0,
            unit
        }
    }

    /// This is the core of the strategy where D. Knuth's reluctant doubling algorithm
    /// is implemented to generate a luby sequence.
    #[inline]
    fn luby(&mut self) -> usize {
        let res = self.v;

        if self.u & -self.u == self.v {
            self.u += 1;
            self.v  = 1;
        } else {
            self.v *= 2;
        }

        return res as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luby_generates_luby_sequence() {
        let mut tested = Luby::new(100);

        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 2);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 2);
        assert_eq!(tested.luby(), 4);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 2);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 1);
        assert_eq!(tested.luby(), 2);
        assert_eq!(tested.luby(), 4);
        assert_eq!(tested.luby(), 8);
    }
    /*
    #[test]
    fn should_restart_follows_luby_sequence(){
        let mut tested = Luby::new(100);

        // 0
        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(101), true);

        // 1
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(201), true);

        // 1
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(201), true);

        // 2
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(300), false);
        assert_eq!(tested.should_restart(400), false);
        assert_eq!(tested.should_restart(401), true);

        // 1
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(201), true);

        // 1
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(201), true);

        // 2
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(300), false);
        assert_eq!(tested.should_restart(400), false);
        assert_eq!(tested.should_restart(401), true);

        // 4
        tested.set_next_limit();

        assert_eq!(tested.should_restart(  1), false);
        assert_eq!(tested.should_restart( 10), false);
        assert_eq!(tested.should_restart( 99), false);
        assert_eq!(tested.should_restart(100), false);
        assert_eq!(tested.should_restart(200), false);
        assert_eq!(tested.should_restart(300), false);
        assert_eq!(tested.should_restart(400), false);
        assert_eq!(tested.should_restart(500), false);
        assert_eq!(tested.should_restart(600), false);
        assert_eq!(tested.should_restart(700), false);
        assert_eq!(tested.should_restart(800), false);
        assert_eq!(tested.should_restart(801), false);
    }*/
}