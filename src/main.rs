fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
  let mut i: usize = 0;
  let mut j: usize = numbers.len() - 1;
  let mut sorted_numbers: Vec<i32> = numbers.to_vec();
  sorted_numbers.sort();
  while (sorted_numbers[i] + sorted_numbers[j]) != target {
    if sorted_numbers[i] + sorted_numbers[j] < target {
      i += 1;
    }
    else {
      j -= 1;
    }
  }
  let ans1 = numbers.iter().position(|&x| x == sorted_numbers[i] as i32).unwrap_or_default();
  let mut ans2 = numbers.iter().position(|&x| x == sorted_numbers[j]).unwrap_or_default();
  if ans1 == ans2 {
    ans2 += 1;
  }
  return (ans1, ans2);
}

fn main() {
    let basic: [i32; 4] = [10, 20, 100, -70];
    let target: i32 = 30;
    println!("{:?}", two_sum(&basic, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum ()  {
      let basic: [i32; 4] = [1,2,3,4];
      let target: i32 = 3;
      assert_eq!((0,1), two_sum(&basic, target))
    }
}
