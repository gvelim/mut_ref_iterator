
fn main() {
    let slice = &mut [1usize,2,3,4,5,6,7];

    let mut ms = MySlice::new(slice);
    println!("MySlice: {:?}",ms);
    ms.iter_mut()
        .map(|x| *x as &mut usize )
        .enumerate()
        .for_each(|(i, x)| { *x *= i+1 });

    println!("MySlice: {:?}",ms);
    ms[0] *= 5;
    ms[1] *= 5;

    println!("Slice: {:?}",slice);
}

#[derive(Debug)]
struct MySlice<'a, T> where T: Ord {
    v : Vec<&'a mut T>,
}

impl<'a, T,>  MySlice<'a, T>
    where T: Ord {

    fn new(slice: &mut [T]) -> MySlice<'_, T> {
        let mut ms = MySlice { v: Vec::new() };
        slice.iter_mut()
            .for_each( |x| ms.v.push(x));
        ms
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&'_ mut &'a mut T> {
        MySliceIterMut::new(self.v.iter_mut() )
    }
}

struct MySliceIterMut<I>
    where I: Iterator, I::Item: Ord {
    iter : I,
}

impl<'a,T,I> MySliceIterMut<I>
    where T: Ord + 'a,
          I: Iterator<Item=&'a mut T>,
          I::Item: Ord {

    fn new(iter: I) -> impl Iterator<Item=&'a mut T> {
        MySliceIterMut {
            iter: iter
        }
    }
}

impl<'a, I, T> Iterator for MySliceIterMut<I>
    where T: 'a,
          I: Iterator<Item=&'a mut T>,
          I::Item: Ord {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(&mut *val ),
            None => None,
        }
    }
}

impl<'a, T> std::ops::Index<usize> for MySlice<'a, T>
    where T: Ord {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        & *self.v[index]
    }
}

impl<'a, T> std::ops::IndexMut<usize> for MySlice<'a, T>
    where T: Ord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut *self.v[index]
    }
}
