#[derive(Debug)]
pub enum UserEvent<M> {
    Message(M),
}
