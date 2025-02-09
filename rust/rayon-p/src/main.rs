use rayon::slice::ParallelSliceMut;

fn main() {
    let mut v = vec![5, 4, 3, 2, 1];
    v.par_sort();
    dbg!(v);
    let filter1 = |x: i32| (x >= 1);
    let filter2 = |x: i32| (x & 2) == 0;
}
