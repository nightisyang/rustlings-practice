// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

<<<<<<< HEAD
// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

// I AM NOT DONE
=======
>>>>>>> my-practice

fn main() {
    let vec0 = Vec::new();

<<<<<<< HEAD
    // Do not move the following line!
    let mut vec1 = fill_vec(vec0);
=======
    let mut vec1 = fill_vec(vec0.clone());
>>>>>>> my-practice

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
