#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate derive_more;

use std::collections::{BTreeMap, HashMap};

#[derive(Index, IndexMut)]
struct MyVec(Vec<i32>);

#[derive(Index, IndexMut)]
struct Numbers {
    #[index]
    numbers: Vec<i32>,
}

#[derive(Index, IndexMut)]
enum MyVecs {
    MyVecVariant(MyVec),
    NumbersVariant(Numbers),
}

#[test]
fn enum_index() {
    let v = MyVecs::MyVecVariant(MyVec(vec![10, 20, 30]));
    assert_eq!(10, v[0]);
    let mut nums = MyVecs::NumbersVariant(Numbers {
        numbers: vec![100, 200, 300],
    });
    assert_eq!(200, nums[1]);
    nums[2] = 400;
    assert_eq!(400, nums[2]);
}

#[derive(Index)]
enum SomeMapNames {
    Hash {
        h: HashMap<i32, u64>,
        #[index(ignore)]
        useless: bool,
    },
    BTree {
        b: BTreeMap<i32, u64>,
    },
}
