
fn array_test() {
    let vec1 = vec![1, 2, 3, 4];
    println!("{:#?}", vec1);
    let vec2 = &mut vec1[..3].to_vec()[..];
    println!("{:?}", vec2);
    let vec3 = &mut vec1[3..].to_vec()[..];
    println!("{:?}", vec3);
}

fn main() {
    array_test();
}
