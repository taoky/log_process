use std::collections::HashSet;

pub fn process(iterator: std::io::Lines<std::io::BufReader<std::boxed::Box<dyn std::io::Read>>>) {
    let mut all_counter = 0;
    let mut all_ip_pool: HashSet<String> = HashSet::new();
    let mut index_counter = 0;
    let mut index_ip_pool: HashSet<String> = HashSet::new();
    for line in iterator {
        match line {
            Err(err) => panic!("cannot read line: {}", err),
            Ok(line) => {
                let ip: String = line.split(" ").next().unwrap().into();
                all_counter += 1;
                all_ip_pool.insert(ip.clone());
                if line.contains("GET / HTTP") && line.contains("Mozilla") {
                    index_counter += 1;
                    index_ip_pool.insert(ip);
                }
            }
        }
    }
    println!("All requests:");
    println!("Lines: {}", all_counter);
    println!("IPs: {}", all_ip_pool.len());
    println!("Index pages (Mozilla):");
    println!("Lines: {}", index_counter);
    println!("IPs: {}", index_ip_pool.len());
}