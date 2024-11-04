fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 take owneship; s1 becomes invalid
    let _s2 = &s2; // immutable reference to the string
    let mut s2 = s2.clone(); // shadowing
    s2.push_str(",abc");
    println!("_s2 = {_s2}, s2 = {s2}"); // _s2 = hello, s2 = hello,abc

    borrow(_s2);
    println!("_s2 = {_s2}");
    takeownship(s2); // s2 go out of scope

    let mut s3 = String::new();
    s3.push_str("ha");
    let x = String::from("lloween");
    concat(&mut s3, &x); // regarding the &mut s3 and the "only 1 mutable ref" rule, it seems within the parentesis it count as another scope?
                         // Anyway the compiler consider this legal
    println!("s3 = {s3}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
fn takeownship(s: String) {
    let _s = s;
}

fn borrow(s: &String) {
    let _s2 = s;
    println!("borrow: _s2 = {_s2}");
}
fn concat(s: &mut String, x: &String) {
    s.push_str(x);
}
