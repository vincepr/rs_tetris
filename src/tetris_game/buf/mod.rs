

/// ### Ring Buffer
/// Implementation of a RingBuffer:
/// - pop() from front, 
/// - push() to end, 
/// - of static length
#[derive(Debug)]
pub struct RingBuffer<T>{
    all:    Vec<T>,
    first:   usize,
    size:   usize,
}
impl <T> RingBuffer <T>{
    pub fn new(starting_vec:Vec<T>) -> Self{
        Self { 
            first: 0,
            size: starting_vec.len() as usize, 
            all: starting_vec, 
        }
    }

    pub fn pop(&mut self)->&T{
        if self.size == 0 {
            panic!("trying to pop() from empty Ring")
        }
        let out = &self.all[self.first];
        self.first = self.add_one(self.first);
        self.size -= 1;
        out
    }

    pub fn push(&mut self, val:T){
        println!("{},{}", self.size >= self.all.len(),1);
        if self.size >= self.all.len(){
            panic!("trying to push() to an already full Ring")
        }
        self.size += 1;
        let new_last_idx = self.add_one(self.first+self.size);
        self.all[new_last_idx] = val;
    }

    // helper functions:
    fn add_one(&self, idx: usize)-> usize{
        if idx + 2  > self.all.len(){
            return idx + 1 - self.all.len();
        }
        idx + 1
    }

    // // usize does not allow abs() so we just do this:
    // fn size(&self)-> usize{
    //     if self.first > self.last{
    //         return self.first-self.last;
    //     }
    //     self.last-self.first
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut ring = RingBuffer::new(vec![0,1,2]);
        println!("{:?}",ring);
        ring.pop();
        println!("{:?}",ring);
        ring.push(8);
        println!("{:?}",ring);
        // ring.push(8);   // should fail!
        // println!("{:?}",ring);


    }
}
