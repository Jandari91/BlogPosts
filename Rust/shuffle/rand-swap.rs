use rand::Rng;

const LEN:usize = 10;

fn shuffle<T>(data: &mut [T]) {
    let mut rng = rand::thread_rng();
    let len = data.len();
    for i in (1..len).rev() {
        let j = rng.gen_range(0..=i);
        data.swap(i, j);
    }
}

fn main() {
    let mut nums = vec![];
    for i in 1..=LEN {nums.push(i);}

    let mut rng = rand::thread_rng();

    println!("before : {:?}", nums);

    shuffle(&mut nums);
    println!("after : {:?}", nums);
}
