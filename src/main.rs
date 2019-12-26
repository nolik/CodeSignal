extern crate core;

mod task;

fn main() {
    assert_eq!(
        task::spiral_matrix::spiralNumbers(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}

/*
  vec![vec![1, 2, 3,   4],
      vec![12, 13, 14, 5] ,
      vec![11,     15, 6]
     vec![10,  9,  8, 7]

      ]

*/
