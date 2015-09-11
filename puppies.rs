// This program is written as a solution to an r/dailyprogrammer challenge:
// https://www.reddit.com/r/dailyprogrammer/comments/33ow0c/20150424_challenge_211_hard_hungry_puppies/

// Notes:
// Be sure to compile with -O, both for running the program and running the tests.
// This program can solve the challenge inputs almost instantly (see the unit tests), but can't solve the bonus in a reasonable amount of time.
// I started with a brute force solution and then kept adding optimizations that help significantly for some inputs but don't help at all in the worst case
// As a result, the run time for a given input length can vary by several orders of magnitude
// See my main() function below for some examples

fn main() {
  // solution: 6
  // guess:    5
  // time to solve: ~300 ms
  let treats = vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 5, 5, 6, 7];
  
  
  
  // the next two are good examples of where this program reaches its limits for a reasonable input
   
  // this one struggles due to a somewhat poor initial guess and several unique treat sizes (6, 7, 8)
  // solution: 6
  // guess:    4
  // time to solve: ~15 seconds
  //let treats = vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8]; 

  // this one is the challenge input with four of the trailing 9's removed
  // it struggles due to sheer size of the input
  // solution: 9
  // guess:    8
  // time to solve: ~40 seconds
  //let treats = vec![1, 1, 2, 2, 2, 2, 2, 2, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 9, 9, 9, 9];
  
  
  
  // the absolute worst case is where none of the treat sizes repeat
  // in this case, none of my optimizations work:
  //  - no repeats in treat size means there are no repeated permutations I can eliminate
  //  - the max happiness is +1, so the "max happiness added in the next n treats" is far too generous; as a result, pruning doesn't help
  //  - the initial guess (0) is close to the solution but that doesn't help because the solution is so low
  
  // these examples show where the solution time becomes impractical for worst-case input
  //let treats = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];         // time to solve: ~168 ms
  //let treats = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];     //time to solve: ~1.1s
  //let treats = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // time to solve: long
  
  let p = solve(&treats);
  
  println!("{}", p.happiness);
  println!("{:?}", p.treats);
}

struct Puppies {
  happiness: i32,
  treats: Vec<i32>,
}

// used in add_treat
const TABLE: [i32; 9] = [-1, -1, 0, -1, 0, 1, 0, 1, 1];

impl Puppies {
  
  fn new() -> Puppies {
    Puppies{
      happiness: 0,
      treats: vec![],
    }
  }
  
  fn add_treat(&mut self, to_add: i32) {
    self.treats.push(to_add);
    if self.length() == 1 {
      self.happiness = 1;
    }
    else if self.length() == 2 {
      self.happiness = 0; // either one happy and one unhappy, or two neutral
    }
    else {
      let last: i32 = self.treats[self.length() - 2];
      let second_last: i32 = self.treats[self.length() - 3];
      
      let mut index = if last > second_last { 0 }
                      else if last == second_last { 3 }
                      else { 6 };
      if last > to_add {
        index += 0;
      }
      else if last == to_add {
        index += 1;
      }
      else {
        index += 2;
      }
      
      self.happiness += TABLE[index];
    }
  }
  
  fn length(&self) -> usize {
    self.treats.len()
  }
}

impl Clone for Puppies {
  fn clone(&self) -> Puppies {
    Puppies {happiness: self.happiness, treats: self.treats.clone()}
  }
}

fn solve(treats: &Vec<i32>) -> Puppies {
  let v = treat_size_counts(treats);
  
  // check for edge case: only one number
  let mut non_zeros = 0;
  for i in 0..v.len() {
    if v[i] != 0 {
      non_zeros += 1;
    }
  }
  if non_zeros == 1 {
    let mut index = 0;
    while v[index] == 0 {
      index += 1;
    }
    
    let mut result = Puppies::new();
    for i in 0..v[index] {
      result.add_treat(index as i32);
    }
    return result;
  }
  
  // no edge case, so solve normally
  let first_guess = guess(treats);
  
  let p = Puppies::new();
  
  permutation_helper(&v, &p, &first_guess)
}

fn permutation_helper(take_from: &Vec<i32>, result: &Puppies, guess: &Puppies) -> Puppies {
  let mut zero_count = 0;
  let mut best_lineup = guess.clone();
  
  let numbers_left: i32 = take_from.iter().fold(0, |sum, x| sum + x) - 1; //  -1 because we're about to use one
  let max_add = numbers_left / 3 + 1;
  
  for i in 0..take_from.len() {
    if take_from[i] != 0 {
      let mut take_from_copy = take_from.clone();
      let mut result_copy = result.clone();
      result_copy.add_treat(i as i32);
      take_from_copy[i] -= 1;
      
      let max_possible_score = result_copy.happiness + max_add;
      if max_possible_score <= best_lineup.happiness {
        continue; // skip this branch since it can't beat the current best score
      }
      
      let p = permutation_helper(&take_from_copy, &result_copy, guess);
      if p.happiness > best_lineup.happiness {
        best_lineup = p;
      }
    }
    else {
      zero_count += 1;
    }
  }
  if zero_count == take_from.len() {
    return result.clone();
  }
  else {
    return best_lineup;
  }
}

// convert treats vector into a vector where index i stores the number of treats of size i
fn treat_size_counts(treats: &Vec<i32>) -> Vec<i32> {
  let treats_copy = treats.clone();
  let max = treats_copy.iter().max().unwrap();
  
  let mut result = vec![];
  for i in 0..*max+1 {
    result.push(0);
  }
  for n in treats.clone() {
    result[n as usize] += 1;
  }
  
  result
}

// create a decent initial guess so that the pruning can be more aggressive
fn guess(treats: &Vec<i32>) -> Puppies {
  let v = treat_size_counts(treats);

  let (mut singles, mut pairs) = singles_and_pairs(&v);
  while pairs >= singles { // make sure there are more singles than pairs
    let x = pairs.pop().unwrap(); // take the biggest first
    singles.push(x);
    singles.push(x);
  }
  
  let mut result = Puppies::new();
  while pairs.len() != 0 {
    let next_pair = pairs[0];
    let mut i = 0;
    while i < singles.len() && singles[i] <= next_pair {
      i += 1;
    }
    if i < singles.len() {
      result.add_treat(singles.remove(i));
      pairs.remove(0);
      result.add_treat(next_pair);
      result.add_treat(next_pair); // can optimize
    }
    else {
      // didn't find one, pull from pairs
      let mut j = 0;
      let largest_pair = pairs[pairs.len() - 1];
      
      // find out where to insert pair
      while j < singles.len() && singles[j] >= largest_pair {
        j += 1;
      }
      pairs.pop();
      singles.insert(j, largest_pair);
      singles.insert(j+1, largest_pair);
    }
  }
  
  // pairs is empty now, add singles in order
  for i in singles {
    result.add_treat(i);
  }
  
  result
}

// sort the treat sizes into pairs and those left over (singles)
fn singles_and_pairs(input: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
  let mut singles = vec![];
  let mut pairs = vec![];
  let mut input_copy = input.clone();
  
  for i in 0..input.len() {
    while input_copy[i] > 1 {
      input_copy[i] -= 2;
      pairs.push(i as i32);
    }
    if input_copy[i] == 1 {
      singles.push(i as i32);
    }
  }
  
  (singles, pairs)
}

#[test]
fn test_solve() {
  // sample input from the reddit post
  let best_score = solve(&vec![1, 1, 1, 1, 1, 2, 2, 3]).happiness;
  assert_eq!(best_score, 3);
  
  // example input 1
  let best_score = solve(&vec![1, 2, 2, 3, 3, 3, 4]).happiness;
  assert_eq!(best_score, 2);
  
  // example input 2 (also challenge input 1)
  let best_score = solve(&vec![1, 1, 2, 3, 3, 3, 3, 4, 5, 5]).happiness;
  assert_eq!(best_score, 4);
  
  // challenge input 2
  let best_score = solve(&vec![1, 1, 2, 2, 3, 4, 4, 5, 5, 5, 6, 6]).happiness;
  assert_eq!(best_score, 4);
  
  // these are test cases that tripped me up while trying to develop a non-brute-force solution
  let best_score = solve(&vec![1, 2, 3, 4, 5]).happiness;
  assert_eq!(best_score, 1);
  let best_score = solve(&vec![1, 1, 1, 1]).happiness;
  assert_eq!(best_score, 0);
  let best_score = solve(&vec![1, 1, 2, 3, 4]).happiness;
  assert_eq!(best_score, 2);
  let best_score = solve(&vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 4]).happiness;
  assert_eq!(best_score, 3);
  let best_score = solve(&vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 5]).happiness;
  assert_eq!(best_score, 5);
  let best_score = solve(&vec![1, 1, 2, 3, 4, 4]).happiness;
  assert_eq!(best_score, 2);
}

#[test]
fn test_score() {
  // note: only used for testing
  fn score(v: &Vec<i32>) -> i32 {
    let mut p = Puppies::new();
    for n in v {
      p.add_treat(*n);
    }
    
    p.happiness
  }

  let mut v = vec![1, 2, 1, 2, 1, 3, 1, 1];
  assert_eq!(score(&v), 0);
  v = vec![1, 2, 2, 3, 3, 3, 4];
  assert_eq!(score(&v), 0);
  v = vec![1, 1, 1, 1, 1, 2, 2, 3];
  assert_eq!(score(&v), 1);
  v = vec![3, 2, 2, 3, 1, 3, 4];
  assert_eq!(score(&v), 2);
  v = vec![2, 1, 1, 2, 1, 1, 1, 3];
  assert_eq!(score(&v), 3);
  v = vec![1, 2, 3, 4, 5];
  assert_eq!(score(&v), 0);
  v = vec![1, 1, 1, 1];
  assert_eq!(score(&v), 0);
  v = vec![5, 4, 3, 2, 1];
  assert_eq!(score(&v), 0);
  v = vec![1];
  assert_eq!(score(&v), 1);
  v = vec![1, 2];
  assert_eq!(score(&v), 0);
  v = vec![2, 2];
  assert_eq!(score(&v), 0);
}