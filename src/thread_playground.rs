use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono;

struct Msg(String, String);

fn calculate_aggregation (s_id: &str, content: &str) {
    println!("Go to sleep id {} with content {} {}", s_id, content, chrono::offset::Utc::now());
    thread::sleep(Duration::from_secs(1));
}

pub fn main () -> Duration {
    let msgs = vec![
        Msg(String::from("s1"), String::from("content1")),
        Msg(String::from("s2"), String::from("content2")),
        Msg(String::from("s1"), String::from("content3")),
        Msg(String::from("s2"), String::from("content4")),
        Msg(String::from("s2"), String::from("content5")),
    ];
    let mut locks_map : HashMap<String, Arc<Mutex<String>>> = HashMap::new();
    let start = Instant::now();
    let mut t_vec = vec![];
    for Msg(s_id, content) in msgs {
        let mutex_id = locks_map
            .entry(String::from(&s_id))
            .or_insert_with(|| {
                Arc::new(Mutex::new(String::from(&s_id)))
            });

        let mutex_id = Arc::clone(&mutex_id);
        let t = thread::spawn(move || {
            let _a = mutex_id.lock().unwrap();
            calculate_aggregation(&s_id, &content);
        });
        t_vec.push(t);
    }
    for t in t_vec {
        t.join().unwrap();
    }
    let duration = start.elapsed();
    duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_at_least_3_sec() {
        let duration = main();
        let duration = duration.as_secs();
        assert!(duration >= 3);
    }
}