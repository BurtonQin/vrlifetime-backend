fn main() {
    let mut v1 = vec![1, 2, 3];
    let r1 = &mut v1;
    let v2 = vec![4, 5, 6];
    *r1 = v2;
//    println!("{:#?}", r1);
}
