fn main() {
    println!("{:09b}=000101010 {:>10}=      IAIK", 42, "IAIK");
    println!("{num:06b}=001010 {who}=rustaceans",
             who = "rustaceans", num = 10);
    let variable = 99;
    println!("{} Luftballoons", variable);
    let l: u64 = 0;
    print!("{}\n", format!("{:04x}", l));
}
