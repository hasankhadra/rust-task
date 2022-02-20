use std::thread;
use std::cmp;

static THRESHOLD: usize = 4;

pub fn split_work<T, R>(vector: Vec<T>, func: fn(t: T) -> R) -> Vec<R>
where T: Copy + std::fmt::Display + std::marker::Send + 'static + std::marker::Sync, 
R: std::marker::Send + 'static + Copy, {
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
    
    
    let mut return_vector: Vec<R> = Vec::new();
    let divisions: usize = (vector.len() + THRESHOLD - 1) / THRESHOLD;

    // loop over each division and handle each of them in a separate thread
    for division in 0..divisions {
        let right_boundary: usize = cmp::min(vector.len(), (division + 1) * THRESHOLD);
    
        // slice the vector to get a chunck of the original input vector
        let division_vec: Vec<T> = vector[division*THRESHOLD..right_boundary].to_vec();

        // apply the work on this chunck 
        let mut computed_division_vec: Vec<R> = apply_work(division_vec, func);

        // append all the results to the returned vector
        return_vector.append(&mut computed_division_vec);
    }

    return return_vector;
}

pub fn apply_work<T, R>(vector: Vec<T>, func: fn(t: T) -> R) -> Vec<R>
where T: Copy + std::fmt::Display + std::marker::Send + 'static , 
R: std::marker::Send + 'static, {
    /// Returns a vector after applying the computational work on the input
    /// vector using the given function
    /// 
    /// For each call, the function instantiate a new thread to do the 
    /// computational work
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

    
    let handle = thread::spawn(move || {
        vector.iter().map(|item| func(*item)).collect()
    });
    handle.join().unwrap()
}