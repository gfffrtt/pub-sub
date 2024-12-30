pub struct Payload {
    pub queue: String,
    pub message: Vec<u8>,
}

impl Payload {
    pub fn new(queue: String, message: Vec<u8>) -> Self {
        Self { queue, message }
    }
}
