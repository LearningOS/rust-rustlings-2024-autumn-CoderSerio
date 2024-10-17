// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // clippy 是一个 lint 工具
    // 原本的代码：
    // 空的不能 unwrap, 这里手动解包
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    // 第一次改动：
    // match my_option {
    //     Some(_) => (),
    //     None => (),
    // }

    // 第二次改动：
    // if let Some(_) = my_option {
    //     ()
    // }

    let _ = my_option.is_some();

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // resize 返回值为 ()
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // 这里竟然会直接报错：
    // error: this looks like you are trying to swap `value_a` and `value_b`
    // 按理说这里的行为是符合语法的，只是显得没什么意义，为什么会直接报错呢？
    // 这里就是rust的一个lint工具的作用了，也就是本章节的标题，clippy, 会自动判别推导一些常见的行为
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
