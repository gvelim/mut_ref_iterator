use std::fmt::{Debug, Formatter};

fn main() {
    let s1 = &mut [1usize,2,3,4,5,6,7];
    let s2 = &mut [8usize,9,10,11,12,13,14];
    let s3 = &mut [15usize,16,17,18,19,20,21];

    println!("Slice 1: {:?}",s1);
    println!("Slice 2: {:?}",s2);
    println!("Slice 3: {:?}",s3);

    let mut ms = VirtualSlice::new();

    ms.attach(s1);
    ms.attach(s2);
    ms.attach(s3);

    println!("VirtualSlice Created:\n\t {:?}",ms);

    ms.iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x *= i+1 );

    println!("VirtualSlice processed:\n\t {:?}",ms);
    ms[0] *= 5;
    ms[1] *= 5;

    println!("Slice 1: {:?}",s1);
    println!("Slice 2: {:?}",s2);
    println!("Slice 3: {:?}",s3);

    ms[0] = 10;
}

struct VirtualSlice<T> where T: Ord {
    v : Vec<*mut T>,
}

impl<T>  VirtualSlice<T>
    where T: Ord {

    fn attach(&mut self, s: &mut [T]) {
        s.iter_mut().for_each( |x| self.v.push(x as *mut T));
    }
    fn new() -> VirtualSlice<T> {
       VirtualSlice { v: Vec::new() }
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&'_ mut T> {
        MySliceIterMut::new(self.v.iter_mut() )
    }
}

struct MySliceIterMut<I>
    where I: Iterator {
    iter_mut: I,
}

impl<'a,I,T> Iterator for MySliceIterMut<I>
    where I: Iterator<Item=&'a mut T>,          // This is the iter signature from the impl of MySliceIter<I>
          T: 'a {                               //   new() -> Iterator<Item=&'a mut T>
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {  // behind this next there is a .map() iter adaptor,
        match self.iter_mut.next() {            //   which mean the call is `iter_mut().map( &&x -> &x ).next()`
            Some(val) => Some(val ),
            None => None,
        }
    }
}

impl<'a, T, I> MySliceIterMut<I>
    where I: Iterator<Item=&'a mut *mut T>,     // We grab input IterMut<> here. The trick here is that *mut T eliminates the
          T: 'a {                               //   need to lifetime binding hence we break the lifetime chain which is unsafe

    fn new(iter_mut: I) -> impl Iterator<Item=&'a mut T> {  // Define the output from &mut*mut T -> &mut T
        MySliceIterMut {                                    //   reference lifetime binds to the iterator's lifetime
            iter_mut : iter_mut
                .map(|x| unsafe { &mut **x })    // doing the type &mut *mut conversion to &mut here
        }
    }
}

impl<T> std::ops::Index<usize> for VirtualSlice<T>
    where T: Ord {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            & *self.v[index]
        }
    }
}

impl<T> std::ops::IndexMut<usize> for VirtualSlice<T>
    where T: Ord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *self.v[index]
        }
    }
}

impl<T> Debug for VirtualSlice<T>
    where T: Ord + Debug {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries( self.v.iter().map(|x| unsafe { & **x } ) )
            .finish()
    }
}
