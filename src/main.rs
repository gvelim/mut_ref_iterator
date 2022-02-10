
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

type MyType<'m,T> = &'m mut T;

#[derive(Debug)]
struct MySlice<'a, T> {
    v : Vec<MyType<'a,T>>,
}

impl<'a, T> MySlice<'a, T> {

    fn new(slice: &mut[T]) -> MySlice<'_, T> {
        let mut ms = MySlice { v: Vec::new() };
        slice.iter_mut()
            .for_each( |x| ms.v.push(x));
        ms
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut MyType<'a,T>> {
        IterMut::new(self.v.iter_mut() )
    }
}

struct IterMut<I: Iterator> {
    iter : I,
}

impl<'a, I, T> IterMut<I>
    where I: Iterator<Item=MyType<'a,T>>, T: 'a {

    fn new(iter: I) -> impl Iterator<Item=MyType<'a,T>> {
        IterMut {
            iter
        }
    }
}

impl<'a, I, T> Iterator for IterMut<I>
    where I: Iterator<Item=MyType<'a,T>>, T: 'a {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(val),
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
