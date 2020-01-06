use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct ApproximateCounterBuilder<F> {
    counter: usize,
    rng: Option<ThreadRng>,
    seq: F,
}

impl<'a, F> ApproximateCounterBuilder<F>
where
    F: FnMut(usize) -> usize + 'a,
{
    pub fn new(seq: F) -> Self {
        Self {
            counter: 0,
            rng: None,
            seq,
        }
    }

    pub fn rng(&mut self, rng: ThreadRng) -> &mut Self {
        self.rng = Some(rng);
        self
    }

    pub fn build(self) -> ApproximateCounter<F> {
        ApproximateCounter {
            counter: self.counter,
            rng: self.rng.unwrap_or(thread_rng()),
            seq: self.seq,
        }
    }
}

pub struct ApproximateCounter<F> {
    counter: usize,
    rng: ThreadRng,
    seq: F,
}

impl<'a, F> ApproximateCounter<F>
where
    F: FnMut(usize) -> usize + 'a,
{
    pub fn increase(&mut self) {
        let ni = (self.seq)(self.counter);
        let ni_next = (self.seq)(self.counter + 1);
        let val = self.rng.gen_range(0, ni_next - ni);
        if val == 0 {
            self.counter += 1;
        }
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn reset(&mut self) {
        self.counter = 0
    }
}
