pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn twice<T>(function: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |a| function(function(a))
}

