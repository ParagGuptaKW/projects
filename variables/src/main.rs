fn main() {
    let mut x = 5;
    println!("value is {}",x);

    x = 6;
    println!("value is {}",x);

    println!("------------------------------------------------------------");

    let k = 5;

    let k = k +1;
    {
        let k = k*2;
        println!("Value inner scope: {k}");
    }
    println!("Val: {k}");
}
