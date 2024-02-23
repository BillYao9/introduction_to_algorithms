use rand::Rng;
use std::time::Instant;

fn main() {
    let mut n = 2;
    for _ in 1..100 {
        n *=2;
        let mut an = Vec::with_capacity(n);
        for _ in 0..n {
            an.push(rand::thread_rng().gen_range(1..=n));
        }
        let start = Instant::now();
        for j in 1..n {
            let temp = an[j];
            let mut i: i32 = (j - 1) as i32;
            while i >= 0 && an[i as usize] > temp {
                an[(i + 1) as usize] = an[i as usize];
                i -= 1;
            }
            an[(i + 1) as usize] = temp;
        }
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("长度为 {n} 的排序运行了 {} 毫秒", duration.as_millis());
        if duration.as_secs() > 3 {
            break;
        }
    }
}
