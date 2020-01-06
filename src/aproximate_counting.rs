use rand::{
    rngs::ThreadRng,
    thread_rng, 
    Rng
};

pub struct AproximateCounter<'a> {
    counter: usize,
    rng: ThreadRng,
    seq: &'a dyn Fn(usize) -> usize,
} 

impl<'a> AproximateCounter<'a> {
    pub fn new() -> Self {
        //TODO: Instead of this should be used builder pattern
        AproximateCounter {
            counter: 0,
            rng: thread_rng(),
            seq: &|x| {x*x}
        }
    }

    pub fn set_rand_gen(&mut self) {
        unimplemented!();
    }

    pub fn set_seq(&mut self, seq: &'a dyn Fn(usize) -> usize) {
        self.seq = seq
    }

    pub fn increase(&mut self) {
        let ni = (self.seq)(self.counter);
        let ni_next = (self.seq)(self.counter + 1);
        let val = self.rng.gen_range(0, ni_next - ni);
        if val == 0 {
            self.counter+=1;
        }
    }

    pub fn count(&self) -> usize {
        self.counter
    }

    pub fn reset(&mut self) {
        self.counter = 0
    }
}
