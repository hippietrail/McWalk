use mcwalk::walk;

fn main() {
    let target_dir = "/tmp"; // Use /tmp as a safe test directory

    println!("Walking path: {}", target_dir);
    let mut count = 0;

    if let Err(e) = walk(target_dir, |path| {
        println!("{}", path.display());
        count += 1;
    }) {
        eprintln!("Error: {}", e);
    } else {
        println!("\nTotal files/directories found: {}", count);
    }
}
