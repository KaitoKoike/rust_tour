fn factorial( n: i32) -> i32{
    let mut out:i32 = 1;
    let mut cnt = n;
    while cnt > 0{
        out = out * cnt;
        cnt = cnt - 1;
    }
    out
}

fn main() {
    let a : i32;
    let b : i32;
    a = 2 ;
    b = 3 ;
    println!("{}", a);
    println!("{}", factorial(b));
}
