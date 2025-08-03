use crate::infrastructure::redis::RedisConfig;

pub struct Worker {
    pub redis_config: RedisConfig,
}

impl Worker {
    pub fn new(redis_config: RedisConfig) -> Self {
        Worker { redis_config }
    }

    pub fn start_worker(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = redis::Client::open(self.redis_config.uri.clone())?;
        let mut con = client.get_connection()?;
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe("payment_process_channel")?;
        loop {
            let msg = pubsub.get_message()?;
            let payload: String = msg.get_payload()?;
            println!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    }
}
