use sorting::selection_sort;

fn main() {
    let mut arr = [1,3,2,4,5,9,6];
    println!("arr = {:?}", arr);
    selection_sort(&mut arr);
    println!("sorted arr = {:?}", arr);
}
