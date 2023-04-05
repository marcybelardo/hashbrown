use hashbrown::md5::md5_run::run;

fn main() {
    let tests = ["", "a", "abc"];

    for test in tests {
        println!("TEST: {test}\n HASH: {:x}", run(test));
    }
}
