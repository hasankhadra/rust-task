mod function;

fn main() {
    let vector: Vec<i32> =  (0..10).collect();
    let output = function::split_work(vector, f);
    for x in output.iter() {
        println!("> {}", x);
    }
}

fn f(t: i32) -> i32{
    return t * t;
}
