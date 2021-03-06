# rust-task
Splitting computational work between threads for a certain hob

## Table of contents
1. [ Logic of the code ](#logic)
2. [ Installation and running ](#install)

<a name="logic"></a>
### Logic of the code
We have two files `main.rs` and `function.rs`. In `main.rs`, we specify the input parameters for the implemented function. As you can see there is already a simple example of a vector of $10$ elements and we're passing the function that squares each number in that vector. Feel free to edit the function and the vector to test the code.

In `function.rs`, we implemented the required function `split_work(vector, func)` that takes a vector with generic typed values, and a function that can be applied to each element in the given vector. In this function, we check the count of the work needed and make a decision if threads need to be injected here. It depends on the `THRESHOLD` declared in the start of the code. Feel free to edit it. If the work was bigger than the threshold and needs threads, we use `par_iter()` from `rayon` which provide us parallel execution for iterations (threading).


<a name="install"></a>
### Installation and running
First, clone this repo on your local machine. Open the terminal inside the directory of the repo (inside the folder `rust-task`). 
To run the code, run the following command in the terminal:
```
cargo run
```
