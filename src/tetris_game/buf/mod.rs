#[derive(Debug, PartialEq)]
pub enum RingBufferError{
    AlreadyEmpty,
    AlreadyFull,
}

/// ### Ring Buffer
/// Implementation of a RingBuffer:
/// - pop() from front, 
/// - push() to end, 
/// - of static length
#[derive(Debug, PartialEq)]
pub struct RingBuffer<T>{
    all:    Vec<T>,
    first:   usize,
    size:   usize,
}
impl <T: Clone> RingBuffer <T>{
    pub fn new(starting_vec:Vec<T>) -> Self{
        Self { 
            first: 0,
            size: starting_vec.len() as usize, 
            all: starting_vec, 
        }
    }

    // pops FIRST element in the Ring and appends new Element to END after.
    pub fn pop_and_push(&mut self, t: T)-> T{
        let out = self.pop().unwrap();
        self.push(t).unwrap();
        out
    }
    fn pop(&mut self)-> Result<T, RingBufferError>{
        if self.size == 0 {
            return Err(RingBufferError::AlreadyEmpty);
        }
        let out = &self.all[self.first];
        self.first = self.add_one(self.first);
        self.size -= 1;
        Ok(out.clone())
    }

    fn push(&mut self, val:T)-> Result<(),RingBufferError>{
        if self.size >= self.all.len(){
            return Err(RingBufferError::AlreadyFull);
        }
        self.size -= 1;
        let new_last_idx = self.add_one(self.first+self.size);
        self.size += 2;
        self.all[new_last_idx] = val;
        dbg!(new_last_idx);
        Ok(())
    }

    // helper functions:
    /// adds one and wraps to beginning if it would be out of bounds of array length
    fn add_one(&self, idx: usize)-> usize{
        let idx = idx + 1;
        if idx > self.all.len()-1{
            return idx  % self.all.len();
        }
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RingBufferError::{AlreadyEmpty, AlreadyFull};

    // #[test]
    // fn test(){
    //     let mut ring = RingBuffer::new(vec![1,2,3]);
    //     dbg!(&ring);
    //     ring.pop_and_push(99);
    //     dbg!(&ring);
    //     ring.pop_and_push(88);
    //     dbg!(&ring);

    // }

    #[test]
    fn pop_to_much(){
        // trying to pop while its empty must fail. (for every possible first element) for size 4
        let mut ring = RingBuffer::new(vec!(0,1,2,3));
        ring.pop().unwrap();
        ring.pop().unwrap();
        ring.pop().unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
        ring.push(9).unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
        ring.push(8).unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
        ring.push(7).unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
        ring.push(6).unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
        ring.push(5).unwrap();
        ring.pop().unwrap();
        assert_eq!(ring.pop().unwrap_err(),AlreadyEmpty);
    }

    #[test]
    fn push_to_much(){
        // pushing into already full must fail. Testing for every possible "first-pointer" for size 3
        let mut ring = RingBuffer::new(vec!(0,1,2));
        assert_eq!(ring.push(9).unwrap_err(),AlreadyFull);
        ring.pop().unwrap();
        ring.push(11).unwrap();
        assert_eq!(ring.push(9).unwrap_err(),AlreadyFull);
        ring.pop().unwrap();
        println!("{:?}", ring);
        ring.push(22).unwrap();
        assert_eq!(ring.push(9).unwrap_err(),AlreadyFull);
        ring.pop().unwrap();
        ring.push(33).unwrap();
        assert_eq!(ring.push(9).unwrap_err(),AlreadyFull); 
        ring.pop().unwrap();
        ring.push(44).unwrap();
        assert_eq!(ring.push(9).unwrap_err(),AlreadyFull); 
    }

    #[test]
    fn add_one_overwrap(){
        // if we add_one( over-array-length) it should wrap arround to 0, 1, 2... again.
        let ring = RingBuffer::new(vec![1,2,3,4]);
        assert_eq!(ring.add_one(0),1);
        assert_eq!(ring.add_one(1),2);
        assert_eq!(ring.add_one(2),3);
        assert_eq!(ring.add_one(3),0);
        assert_eq!(ring.add_one(4),1);
        assert_eq!(ring.add_one(5),2);

        let ring2 = RingBuffer::new(vec![1,2,3]);
        assert_eq!(ring2.add_one(0),1);
        assert_eq!(ring2.add_one(1),2);
        assert_eq!(ring2.add_one(2),0);
        assert_eq!(ring2.add_one(3),1);
        assert_eq!(ring2.add_one(4),2);
        assert_eq!(ring2.add_one(5),0);
    
        let ring3 = RingBuffer::new(vec![1,2]);
        assert_eq!(ring3.add_one(0),1);
        assert_eq!(ring3.add_one(1),0);
        assert_eq!(ring3.add_one(2),1);
        assert_eq!(ring3.add_one(3),0);
        assert_eq!(ring3.add_one(4),1);
        assert_eq!(ring3.add_one(5),0);
    }

    #[test]
    fn push_into_pop_out(){
        let mut ring1 = RingBuffer::new(vec![0,1,2]);

        // first pop push
        let out = ring1.pop().unwrap();
        assert_eq!(out, 0);
        let ring2 = RingBuffer{all: vec![0,1,2],first: 1,size: 2,};
        assert_eq!(ring1, ring2);

        ring1.push(99).unwrap();
        let ring2 = RingBuffer{all: vec![99,1,2],first: 1,size: 3,};
        assert_eq!(ring1, ring2);

        // second pop push
        let out = ring1.pop().unwrap();
        assert_eq!(out, 1);
        let ring2 = RingBuffer{all: vec![99,1,2],first: 2,size: 2,};

        assert_eq!(ring1, ring2);
        ring1.push(88).unwrap();
        let ring2 = RingBuffer{all: vec![99,88,2],first: 2,size: 3,};
        assert_eq!(ring1, ring2);
        



    }
}
