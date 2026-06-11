use redis::Commands;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    id: String,
    name: String,
}

fn main() {
    println!("🚀 worker started");

    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => client,
        Err(err) => {
            eprintln!("failed to connect to redis: {}", err);
            return;
        }
    };

    let mut conn = match client.get_connection() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("failed to get redis connection: {}", err);
            return;
        }
    };

    loop {
        let result: redis::RedisResult<Option<[String; 2]>> =
            conn.blpop("jobs_queue", 0.0);

        let payload = match result {
            Ok(Some([_, job_json])) => job_json,
            Ok(None) => {
                continue;
            }
            Err(err) => {
                eprintln!("redis error: {}", err);
                sleep(Duration::from_secs(2));
                continue;
            }
        };

        let job: Job = match serde_json::from_str(&payload) {
            Ok(job) => job,
            Err(err) => {
                eprintln!("invalid job payload: {}", err);
                continue;
            }
        };

        println!("⚙️ processing: {}", job.name);

        sleep(Duration::from_secs(3));

        println!("✅ completed: {}", job.id);
    }
}