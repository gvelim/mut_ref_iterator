# Iterating over mutable references

## Need
* A simple way to operate sequencially over multiple array segments in memory at once
We should be able to
* perform direct operations like `a[12] = 10` or `a.swap(10,20)`
* iterate using a single iterator like `a.iter_mut().enumerate().for_each( |i, x| *x *= i )`


## An approach
* Constract an array of references to all array segments using something like 
```
  let s1 = &mut [1,2,3,4,5,6,7];
  let s2 = &mut [8,9,10,11,12,13,14];
  let s3 = &mut [15,16,17,18,19,20,21];

  let mut ms = VirtualSlice::new();

  ms.attach(s1);
  ms.attach(s2);
  ms.attach(s3);
  
//  VirtualSlice holding *mutable references* {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21}  
```
* Iterate over the mutable references using `&mut T` rather `&mut &mut T` and without attaching to `ms` lifetime
```
ms.iter_mut()
  .enumerate()
  .for_each(|(i, x)| *x *= i+1 );
  
// VirtualSlice : {1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441}
// s1: [5, 20, 9, 16, 25, 36, 49]
// s2: [64, 81, 100, 121, 144, 169, 196]
// s3: [225, 256, 289, 324, 361, 400, 441]
```
