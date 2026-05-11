use borsh::{ BorshDeserialize, BorshSerialize };
use crosstown_bus::{ CrosstownBus, MessageHandler, HandleError, QueueProperties };
use std::time;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let _now = time::Instant::now();
        println!("In mug's Computer [2406347424]. Message received: {:?}", message);
        Ok(())
    }
}

fn main() {
    let mut listener = CrosstownBus::new_subscriber(
        "amqp://guest:guest@localhost:5672".to_owned()
    ).unwrap();

    _ = listener.subscribe(
        "user_created".to_owned(),
        UserCreatedHandler {},
        QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
            consume_queue_name: Some("user_created".to_string()),
        }
    );

    loop {
    }
}
