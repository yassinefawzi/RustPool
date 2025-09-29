pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    return move |y| x + y;
}