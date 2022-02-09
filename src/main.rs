
fn main() {
    let slice = &mut [1,2,3,4,5,6,7];
    {
        let mut ms = MySlice::new(slice);
        println!("MySlice: {:?}",ms);
        ms.iter_mut()
            .enumerate()
            .for_each(|(i, x)| { *x *= i+1 });
    }
    {
        let mut ms = MySlice::new(slice);
        println!("MySlice: {:?}",ms);
        ms[0] *= 5;
        ms[1] *= 5;
    }
    println!("Slice: {:?}",slice);
}

#[derive(Debug)]
struct MySlice<'a, T> {
    v : Vec<&'a mut T>,
}

impl<'a, T: 'a>  MySlice<'a, T> {

    fn new(slice: &'a mut [T]) -> MySlice<'a, T> {
        let mut ms = MySlice { v: Vec::new() };
        slice.iter_mut()
            .for_each( |x| ms.v.push(x));
        ms
    }

    fn iter_mut(&'a mut self) -> MySliceIterMut<T> {
        MySliceIterMut::new(self.v.iter_mut() )
    }
}

struct MySliceIterMut<'a, T> {
    iter : std::slice::IterMut<'a, &'a mut T>,
}

impl<'a, T> MySliceIterMut<'a, T>  {
    fn new(iter: std::slice::IterMut<'a, &'a mut T>) -> MySliceIterMut<'a, T> {
        MySliceIterMut {
            iter
        }
    }
}

impl<'a, T> Iterator for MySliceIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(&mut *val),
            None => None,
        }
    }
}

impl<'a, T> std::ops::Index<usize> for MySlice<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        & *self.v[index]
    }
}

impl<'a, T> std::ops::IndexMut<usize> for MySlice<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut *self.v[index]
    }
}

use std::ops::Range;

impl<'a, T> std::ops::Index<Range<usize>> for MySlice<'a, T> {
    type Output = [&'a mut T];

    fn index(&self, rng: Range<usize>) -> &Self::Output {
        &self.v[rng]
    }
}

impl<'a, T> std::ops::IndexMut<Range<usize>> for MySlice<'a, T> {
    fn index_mut(&mut self, rng: Range<usize>) -> &mut Self::Output {
        &mut self.v[rng]
    }
}