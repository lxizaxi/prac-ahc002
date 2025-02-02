use std::process;

fn get_input() -> (u32, u32, Vec<Vec<u32>>, Vec<Vec<u32>>) {
    proconio::input! {
        (init_i, init_j): (u32, u32)
    };

    let mut t: Vec<Vec<u32>> = Vec::new();
    for _ in 0..50 {
        proconio::input! {
            line: [u32;50],
        }
        t.push(line);
    }

    let mut p: Vec<Vec<u32>> = Vec::new();
    for _ in 0..50 {
        proconio::input! {
            line: [u32;50],
        }
        p.push(line);
    }

    return (init_i, init_j, t, p);
}

fn main() {
    let (init_t, init_j, t, p) = get_input();

    print!("{:?}", init_t);
    print!("{:?}", init_j);
    print!("{:?}", t);
    print!("{:?}", p);
}
