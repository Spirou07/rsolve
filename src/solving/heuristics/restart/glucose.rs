use solving::heuristics::RestartHeuristic;

/// This structure encapsulates the restart strategy of the solver.
/// It is implemented using Glucose's restart algorithm
#[derive(Debug)]
pub struct Glucose {
    k : f64,
    x : usize
}

impl RestartHeuristic for Glucose {
    #[inline]
    fn should_restart(&self, avg_glob: f64, queue: &Vec<u32>) -> bool {
        if queue.len() < self.x { return false }
        let sum: f64 = queue.iter().map(|&v| v as f64).sum();
        sum/queue.len() as f64 * self.k > avg_glob as f64
    }

    /// Sets the next conflict limit before the next restart
    /// cannot but a limit
    #[inline]
    fn set_next_limit(&mut self) {

    }
}
impl Glucose {
    /// Creates a new instance
    pub fn new() -> Glucose {
        Glucose {
            k: 0.7,
            x: 100
        }
    }
}

// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glucose_restart_fn() {
        let tested = Glucose::new();
        let _ret = tested.should_restart(1.0,&vec![1]);
        assert_eq!(tested.should_restart(1.0,&vec![1]), false);
        assert_eq!(tested.should_restart(1.0,&vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1]), false);
        assert_eq!(tested.should_restart(1.0,&vec![2,2,2]), false);
        assert_eq!(tested.should_restart(1.0,&vec![100]), false);
        assert_eq!(tested.should_restart(1.0,&vec![101]), false);
        let mut vec1 = Vec::with_capacity(100);
        let mut vec2 = Vec::with_capacity(100);
        for i in 1..100 {
            vec1.push(i);
            vec2.push(i);
        }
        vec2.push(100);

        assert_eq!(tested.should_restart(1.0,&vec1), false);
        assert_eq!(tested.should_restart(1.0,&vec2), true);
        assert_eq!(tested.should_restart(1.0,&vec2), true);
        assert_eq!(tested.should_restart(100.0,&vec2), false);
        assert_eq!(tested.should_restart(10.0,&vec2), true);
        assert_eq!(tested.should_restart(50.0,&vec2), false);
        assert_eq!(tested.should_restart(35.3,&vec2), true);
        assert_eq!(tested.should_restart(35.4,&vec2), false);

    }

    #[test]
    fn average() {
        let glucose_size = 100;
        let mut lbd_vec: Vec<u32> = Vec::with_capacity(glucose_size);
        let mut average: f64 = 0.0;

        for i in 1..100 {
            lbd_vec.push(i);
            let inc: f64 = (i as f64 - average) / i as f64;
            println!("inc {}", inc);
            average = average + inc;
            println!("current i : {}", i);
            println!("new avg : {}", average);
        }
        assert_eq!(average, 50 as f64);
    }
    #[test]
    fn remove_fifo() {
        let glucose_size = 100;
        let mut lbd_vec: Vec<u32> = Vec::with_capacity(glucose_size);
        let mut nb_learned = 0;
        for i in 0..100 {
            lbd_vec.push(i);
            nb_learned += 1;
            let mut get = 0;
            match lbd_vec.get(i as usize) {
                None => { /* nothing to do */ },
                Some(r) => {
                    get = *r;
                }
            }
            assert_eq!(lbd_vec.len(), (i+1) as usize);
            assert_eq!(get,i);
        }
        for i in 0..49{
            lbd_vec.push(50+i);
            nb_learned += 1;
            if lbd_vec.len() > glucose_size {
                lbd_vec.swap_remove(nb_learned % (glucose_size+1));
            }
            let mut get = 0;
            match lbd_vec.get(i as usize) {
                None => { /* nothing to do */ },
                Some(r) => {
                    get = *r;
                }
            }
            assert_eq!(lbd_vec.len(),100);
            assert_eq!(get,50+i)
        }
    }
}
