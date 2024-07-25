/*

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    //Iterator trait comes standard with next method
}

#[test]
fn iterator_demonstartion() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), None);
}


#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 15);
}

fn main() {

    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    for value in v1_iter{
        println!("Got: {}", value);
    }
    
}  
*/


fn main() {
    let v1 = vec![1, 2, 3];

    //map takes a closure and creates an iterator which
    //calls the closure on each element

    //the map method won't do anything unless put in a collection

    //.iter() does not take ownership while into_iter() does

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v2, vec![2, 3, 4])

}