use std::ffi::OsStr;
use std::path::Path;
use std::collections::BTreeSet;
use std::fmt::Debug;

pub fn chapter15() {
    println!("hello, world!");
    println!("{}", triangle(10));

    println!("There's");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    // for element in &v {
    //     println!("{}", element);
    // }
    
    // two ways
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    let v = vec![1, 2, 3, 4];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&3));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    // path
    let path = Path::new("C:/Users/Jimb/Downlands/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("Jimb")));
    assert_eq!(iterator.next(), Some(OsStr::new("Downlands")));
    assert_eq!(iterator.next(), Some(OsStr::new("Fedora.iso")));
    assert_eq!(iterator.next(), None);

    // btreeset
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebestraume No.3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebestraume No.3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);

    let mut outer = "Earth".to_string();
    let _inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    
}

pub fn triangle(n: i32) -> i32 {
    (1..n+1).fold(0, |sum, item| sum + item)
}


pub fn dump<T, U>(t: T) 
    where T: IntoIterator<Item=U>,
        U: Debug
{
    for u in t {
        println!("{:?}", u);
    }
}
