/*
Haskell idea

data Showable = forall s . Show s => Showable {
    unbox :: s }

reverseShow :: Showable -> String
reverseShow (Showable s) = reverse $ show s

Example idea in Rust

struct Bowl<T> {
  T caps,
  id: i64
}

fn createReadableBowl(id: i64) -> Bowl<Box> {
  struct ReadBowlFoo {
  }
  impl Read<Bowl<Box>> for ReadBowlFoo { };
  let cap =  ReadBowlFoo{};
  let bowl = Bowl<Box>(Box::new(cap),id);
  bowl
}

fn readBowlData<T:Read<Bowl<Box>> (T cap, Bowl<Box>) -> () {
....
}

readBowlData(bowl.caps.unwrap, bowl)
*/
#[allow(dead_code)]
struct Bowl<T> {
    cap: T,
    id: i64,
}

trait Read<T> {
    fn read(&self) -> T;
}

#[allow(unused_variables)]
fn create_readable_bowl(id: i64) {
    struct ReadBowl;
    impl Read<Bowl<ReadBowl>> for Bowl<ReadBowl> {
        fn read(&self) -> Bowl<ReadBowl> {
            Bowl::<ReadBowl> {
                id: self.id,
                cap: ReadBowl,
            }
        }
    }
}

fn main() {
    println!("Hello");

    let _r = create_readable_bowl(123);
}
