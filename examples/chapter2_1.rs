use rand::Rng;
use std::time::Instant;

fn main() {
    let mut n = 2;
    for _ in 1..100 {
        n *= 2;
        let mut a1 = Vec::with_capacity(n);
        for _ in 0..n {
            let v = rand::thread_rng().gen_range(1..=n);
            a1.push(v);
        }
        // for j in 1..n {
        //     let temp = a1[j];
        //     let mut i: i32 = (j - 1) as i32;
        //     while i >= 0 && a1[i as usize] > temp {
        //         a1[(i + 1) as usize] = a1[i as usize];
        //         i -= 1;
        //     }
        //     a1[(i + 1) as usize] = temp;
        // }
        // a1.reverse();
        let start = Instant::now();
        for j in 1..n {
            let temp = a1[j];
            let mut i: i32 = (j - 1) as i32;
            while i >= 0 && a1[i as usize] > temp {
                a1[(i + 1) as usize] = a1[i as usize];
                i -= 1;
            }
            a1[(i + 1) as usize] = temp;
        }
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("长度为 {n} 的排序运行了 {} 毫秒", duration.as_millis());
        if duration.as_secs() > 3 {
            break;
        }
    }
}
