pub fn first_fifty_even_square() -> Vec<i32> {
    let mut a = vec![];
    let mut i = 2 ;
    while a.len() != 50 {
        a.push(i*i) ;
        i+=2 ;
    }
    a
}