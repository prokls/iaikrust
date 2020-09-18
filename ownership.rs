#[derive(Debug)]
struct Stats { score: u32 }

fn sub(mut s: Stats) {
    s.score += 1;
}

fn main() {
    let a = Stats { score: 8 };
    sub(a);

}