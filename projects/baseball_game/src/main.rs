extern crate rand;
extern crate shuffle;
use rand;
use std::io;
use shuffle::shuffler::Shuffler;

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: [i32; 10] = [0; 10];
    let mut game: [i32; 4] = [0; 4];
    for i in 0..10 {
        arr[i] = i as i32;
    }
    for i in (n - 1)..1=-1 {
        let j = rng.gen::<i32>() % (i + 1);
        let temp = arr[j];
        arr[j] = arr[i];
        arr[i] = temp;
    }
    for i in 0..4 {
        game[i] = arr[i];
    }

    let mut cond: bool = true;
    let mut line = String::new();
    while cond {
        let mut guess: [i32; 4] = [0; 4];
        for i in 0..4 {
            io::stdin().read_line(&mut line).expect(
                "Failed to read lint"
            );
            let number: i32 = line.parse::<i32>().unwrap();
            guess[i] = number
        }
        let mut counter: [i32; 10] = [0; 10];
        for i in 0..4 {
            counter[guess[i] as usize] += 1;
        }
        let mut is_right: bool = true;
        for i in 0..10 {
            if counter[i] >= 2 {
                is_right = false;
            }
        }
        if is_right == false {
            println!("wrong input");
            continue;
        }

        let mut strike = 0;
        let mut ball = 0;

        for i in 0..4 {
            if guess[i] == game[i] {
                strike += 1;
            } else if game.contains(&guess[i]) {
                ball += 1;
            }
        }

        println!("strike {} ball {}", strike, ball);
        if strike == 4 {
            cond = false;
        };
    }
}
