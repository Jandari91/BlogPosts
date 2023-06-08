use rand::{Rng, seq::SliceRandom};

const LEN:usize = 10;

fn main() {
    let mut vectors = vec![];
    let mut arrays= [0;LEN];
    

    for i in 1..=LEN {
        vectors.push(i);
        arrays[i-1] = i;
    }

    let mut rng = rand::thread_rng();
    println!("[Vector] before : {:?}", vectors);
    vectors.shuffle(&mut rng);
    println!("[Vector] after : {:?}", vectors);

    println!("[Array] before : {:?}", arrays);
    arrays.shuffle(&mut rng);
    println!("[Array] after : {:?}", arrays);
}
    
    
