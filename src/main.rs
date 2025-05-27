pub mod query;

fn main() {
    println!("Try to query minecraft process(es).");
    let data = query::query_process().unwrap();

    if data.is_empty() {
        println!("There is no minecraft process.");
    } else {
        println!("{}", format!("Found {} minecraft process(es).", data.len()));
        for info in data {
            println!("{}", info)
        }
    }
}
