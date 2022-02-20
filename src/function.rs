use rayon::prelude::*;

static THRESHOLD: usize = 4;

pub fn split_work<T, R>(vector: Vec<T>, func: fn(t: T) -> R) -> Vec<R>
where T: Sync + Copy + std::fmt::Display, 
R: Copy + std::marker::Send, {
    /// Returns a vector after applying the computational work on the input
    /// vector using the given function
    /// 
    /// # Arguments
    ///
    /// * `vector` - The input vector of type Vec<T>
    /// * `func` - The function to apply the computational work (t: T) -> R
    /// 
    /// # Return
    ///
    /// A vector of type Vec<R> that contains the resultant input vector after 
    /// the computational work has been applied

    // apply the computational work in the main program
    if vector.len() <= THRESHOLD {
        let return_vector: Vec<R> = vector.iter().map(|item| func(*item)).collect();        
        return return_vector;
    }

    // use parallel execution (threading) to apply the work on the vector
    let return_vector: Vec<R> = vector.par_iter().map(|item| func(*item)).collect();        
    return return_vector;
}
