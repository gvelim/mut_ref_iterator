
fn main() {
    let slice = &mut [1usize,2,3,4,5,6,7];

    let mut ms = MySlice::new(slice);
    println!("MySlice: {:?}",ms);
    ms.iter_mut()
        .enumerate()
        .for_each(|(i, x)| { *x *= i+1 });

    println!("MySlice: {:?}",ms);
    ms[0] *= 5;
    ms[1] *= 5;

    println!("Slice: {:?}",slice);
}

#[derive(Debug)]
struct MySlice<T> where T: Ord {
    v : Vec<*mut T>,
}

impl<T>  MySlice<T>
    where T: Ord {

    fn new(s: &mut [T]) -> MySlice<T> {
        let mut ms = MySlice { v: Vec::new() };
        s.iter_mut().for_each( |x| ms.v.push(x as *mut T));
        ms
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&'_ mut T> {
        MySliceIterMut::new(self.v.iter_mut() )
    }
}

struct MySliceIterMut<I>
    where I: Iterator {
    iter_mut: I,
}

impl<'a, I, T> Iterator for MySliceIterMut<I>
    where I: Iterator<Item=&'a mut *mut T>,
          T: 'a {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_mut.next() {
            Some(val) => {
                unsafe {
                    Some(&mut **val )
                }
            },
            None => None,
        }
    }
}

impl<'a, T, I> MySliceIterMut<I>
    where I: Iterator<Item=&'a mut *mut T>,
          T: 'a {

    fn new(iter_mut: I) -> impl Iterator<Item=&'a mut T> {
        MySliceIterMut {
            iter_mut
        }
    }
}

impl<'a, T> std::ops::Index<usize> for MySlice<T>
    where T: Ord {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            & *self.v[index]
        }
    }
}

impl<'a, T> std::ops::IndexMut<usize> for MySlice<T>
    where T: Ord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *self.v[index]
        }
    }
}
