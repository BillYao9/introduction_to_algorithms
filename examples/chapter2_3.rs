use rand::Rng;
use std::time::Instant;

fn main() {
    let mut n = 2;
    for _ in 1..100 {
        n *= 2;
        let mut a1 = Vec::with_capacity(n);
        for _ in 0..n {
            let v: usize = rand::thread_rng().gen_range(1..=n);
            a1.push(v);
        }
        let start = Instant::now();
        merge_sort(&mut a1, 0, n);
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("长度为 {n} 的排序运行了 {} 毫秒", duration.as_millis());
        if duration.as_secs() > 3 {
            break;
        }
    }
}
fn merge_sort(a1: &mut Vec<usize>, p: usize, r: usize) {
    // println!("a1 : {:?}", a1);
    // println!("p : {}", p);
    // println!("r : {}", r);

    if p < r - 1 {
        let q = (p + r) / 2;
        //println!("q : {}", q);
        merge_sort(a1, p, q);
        merge_sort(a1, q, r);
        merge(a1, p, q, r);
    }
}
fn merge(a1: &mut Vec<usize>, p: usize, q: usize, r: usize) {
    let len1 = q - p;
    let len2 = r - q;
    let mut la = Vec::with_capacity(len1 + 1);
    let mut ra = Vec::with_capacity(len2 + 1);
    //println!("{:?}", a1);

    for i in 0..len1 {
        la.push(a1[p + i]);
    }
    la.push(usize::MAX);
    //println!("{:?}", la);

    for i in 0..len2 {
        ra.push(a1[q + i]);
    }
    ra.push(usize::MAX);
    //println!("{:?}", ra);
    let mut i = 0;
    let mut j = 0;
    for n in p..r {
        if la[i] <= ra[j] {
            a1[n] = la[i];
            i += 1;
        } else {
            a1[n] = ra[j];
            j += 1;
        }
    }
    //println!("{:?}", a1);
}
