// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    // collect 会把 迭代器 转为 Vec<_> 的 vec（是的，根据类型推导
    let numbers: Vec<_> = (0..100u32).collect();
    // arc 和 rc 都是支持计数的，但是 arc 是原子的、线程安全的，而 rc 不是。
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // 这里为啥不直接用 shared_numbers 呢？
        // 是因为借用检查：Rust 的借用检查器（borrow checker）会阻止你直接将 shared_numbers 传入多个线程中的闭包，
        // 因为这会导致多个线程同时访问同一个引用——这在 Rust 中是不允许的
        let child_numbers = Arc::clone(&shared_numbers);
        //在 Rust 中，闭包默认捕获外部作用域中的变量的方式有两种： 借用（默认情况，只读这些变量） 和 移动（可以改这些变量）
        // Rust 不允许多个线程同时访问同一个引用
        // 这里由于是使用基本数据类型——天然支持 copy trait，所以可以直接通过移动获取一份副本
        joinhandles.push(thread::spawn(move || {
            // iter() 返回的是 &u32，filter 一般是都是 &xxx 引用，所以二者叠加就是 &&n
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
