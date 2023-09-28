pub mod registers;

/// This is a trait that simplifies the interface of passing messages both ways
pub trait Mailbox<M> {
    fn send(&self, message: M) -> Result<(), std::sync::mpsc::SendError<M>>;
    fn recv(&self) -> Result<M, std::sync::mpsc::RecvError>;
    fn try_recv(&self) -> Result<M, std::sync::mpsc::TryRecvError>;
}
