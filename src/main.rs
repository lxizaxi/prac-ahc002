use rand::Rng;
use std::fmt;
use std::time::Instant;

const MAP_SIZE: u32 = 10000;

#[derive(Copy, Clone, Debug)]
struct Request {
    row: u32,
    col: u32,
    area: u32,
}

impl Request {
    fn new(row: u32, col: u32, area: u32) -> Self {
        Self { row, col, area }
    }
}

#[derive(Clone, Debug)]
struct Input {
    count: usize,
    requests: Vec<Request>,
    since: Instant,
}

impl Input {
    fn new(count: usize, requests: Vec<Request>) -> Self {
        Self {
            count,
            requests,
            since: Instant::now(),
        }
    }
}

struct Advertisement {
    row0: u32,
    col0: u32,
    row1: u32,
    col1: u32,
}

impl Advertisement {
    fn new(row0: u32, col0: u32, row1: u32, col1: u32) -> Self {
        debug_assert!(row0 < row1);
        debug_assert!(col0 < col1);
        debug_assert!(row1 < MAP_SIZE);
        debug_assert!(col1 < MAP_SIZE);
        Self {
            row0,
            col0,
            row1,
            col1,
        }
    }

    fn intersects(&self, other: &Advertisement) -> bool {
        self.row0.max(other.row0) < self.row1.min(other.row1)
            && self.col0.max(other.col0) < self.col1.min(other.col1)
    }

    fn contains(&self, row: u32, col: u32) -> bool {
        self.row0 <= row && row < self.row1 && self.col0 <= col && col < self.col1
    }

    fn width(&self) -> u32 {
        self.row1 - self.row0
    }

    fn height(&self) -> u32 {
        self.col1 - self.col0
    }

    fn area(&self) -> u32 {
        self.width() * self.height()
    }
}

impl fmt::Display for Advertisement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.row0, self.col0, self.row1, self.col1)
    }
}

fn main() {
    proconio::input! {
        n: usize,
        requests: [(u32, u32, u32);n]
    };

    let input = Input::new(
        n,
        requests
            .iter()
            .map(|r| Request::new(r.0, r.1, r.2))
            .collect(),
    );
    let results = random_expand(input);

    for ad in results {
        println!("{}", ad);
    }
}

fn random_expand(input: Input) -> Vec<Advertisement> {
    // 乱数生成器の初期化
    let mut rng = rand_pcg::Pcg64Mcg::new(42);

    // 初期広告の生成:
    let mut results = input
        .requests
        .iter()
        .map(|r| Advertisement::new(r.row, r.col, r.row + 1, r.col + 1))
        .collect::<Vec<_>>();

    // ループの初期化
    let mut iter = 0;
    let mut time = Instant::now();
    const TIME_LIMIT: f64 = 4.98;

    // 焼きなまし法のパラメータ
    let initial_temp = 100.0;
    let cooling_rate = 0.99;
    let mut current_temp = initial_temp;

    // メインループ
    'main: while (time - input.since).as_secs_f64() / TIME_LIMIT < 1.0 {
        // 反復回数の更新と時間更新
        iter += 1;
        if iter % 100 == 0 {
            time = Instant::now();
        }

        // ランダムな広告の選択
        let index = rng.gen_range(0..input.count);
        let last = &results[index];

        // 広告の拡張
        let (r0, c0, r1, c1) = match rng.gen_range(0..8) {
            0 => (last.row0.wrapping_sub(1), last.col0, last.row1, last.col1),
            1 => (last.row0, last.col0.wrapping_sub(1), last.row1, last.col1),
            2 => (last.row0, last.col0, last.row1 + 1, last.col1),
            3 => (last.row0, last.col0, last.row1, last.col1 + 1),
            4 => (
                last.row0.wrapping_sub(1),
                last.col0,
                last.row1.wrapping_sub(1),
                last.col1,
            ),
            5 => (
                last.row0,
                last.col0.wrapping_sub(1),
                last.row1,
                last.col1.wrapping_sub(1),
            ),
            6 => (last.row0 + 1, last.col0, last.row1 + 1, last.col1),
            7 => (last.row0, last.col0 + 1, last.row1, last.col1 + 1),
            _ => unreachable!("arienai!"),
        };

        // マップ範囲のチェック
        if r0 >= MAP_SIZE || c0 >= MAP_SIZE || r1 >= MAP_SIZE || c1 >= MAP_SIZE {
            continue;
        }

        // 新しい広告の生成
        let new = Advertisement::new(r0, c0, r1, c1);

        // 広告の重複チェック
        for (i, ad) in results.iter().enumerate() {
            if i != index && ad.intersects(&new) {
                continue 'main;
            }
        }

        // スコアの計算
        let current_score = calc_score_each(&input.requests[index], &results[index]) as f64;
        let new_score = calc_score_each(&input.requests[index], &new) as f64;

        // 遷移確率の計算
        let delta_score = new_score - current_score;
        let acceptance_probability = if delta_score > 0.0 {
            1.0
        } else {
            (delta_score / current_temp).exp()
        };

        // 状態遷移の決定
        if rng.gen_bool(acceptance_probability)
            && new.area() <= input.requests[index].area
            && new.contains(input.requests[index].row, input.requests[index].col)
        {
            results[index] = new;
        }

        // 温度の更新
        current_temp *= cooling_rate;
    }

    eprintln!("");
    eprintln!("iter: {}", iter);
    eprintln!("score: {}", calc_score(&input, &results));
    eprintln!("");

    results
}

fn calc_score(input: &Input, ads: &Vec<Advertisement>) -> i32 {
    fn round(x: f64) -> i32 {
        (((x * 2.0) as i32) + 1) >> 1
    }

    let mut score = 0.0;

    for (req, ad) in input.requests.iter().zip(ads) {
        score += calc_score_each(req, ad);
    }

    round(1e9 * score / (input.count as f64))
}

fn calc_score_each(req: &Request, ad: &Advertisement) -> f64 {
    if ad.contains(req.row, req.col) {
        let area = ad.area();
        let x = 1.0 - (req.area.min(area) as f64) / (req.area.max(area) as f64);
        1.0 - x * x
    } else {
        0.0
    }
}
