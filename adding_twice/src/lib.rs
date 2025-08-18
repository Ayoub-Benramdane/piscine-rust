pub fn add_curry<>(a: i32) -> impl Fn(i32) -> i32 {
    move |b: i32| a + b
}

pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x: T| f(f(x))
}