use std::cmp::Ordering;

pub struct Pair<T, U> {
    first: T,
    second: U,
}

// to doo Ord later
impl <T, U> Pair<T, U> {
  pub fn new(first: T, second: U)-> Self {
    Self { first, second }
  }
    
}

impl<T, U> Pair<T, U>
where
    T: Ord,
    U: Ord,
{
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.first.cmp(&other.first) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.second.cmp(&other.second) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[test]
fn test() {
  let pair_1 = Pair::new(2,3);
  let pair_2 = Pair::new(2,5);
  let _ = pair_1.cmp(&pair_2) == Ordering::Less;

  let _aa = &pair_2;
  assert!(pair_1.cmp(&pair_2) == Ordering::Less, "Should be less than");
}