pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums2.into_iter()
        .fold((vec![],
            nums1.into_iter()
            .fold(HashMap::new(), |mut m, x|{
                m.entry(x).and_modify(|v| *v += 1).or_insert(1); m
            })
        ),
        |(mut r, mut m), x| {
            if m.get(&x).unwrap_or(&0) > &0 {
                r.push(x);
                m.entry(x).and_modify(|v| *v -= 1);
            }
            (r, m)
        }).0
    }
}

fn main(){
    let num1 = vec![2,4,6,8,10];
    let num2 =vec![3,6,7,9,12,13];

    let my_sol = Solution::intersect(num1, num2);


    for value in my_sol{
        println!("{value}");

    }

}