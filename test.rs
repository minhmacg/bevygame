fn main() {
    let a = vec!("12","123","41");
    let c: Vec<&&str> = a.iter().filter(|x| **x == "12").collect();
    println!("{:?}",c)
}
