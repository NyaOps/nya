#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Event {
  TestEvent,
  Custom(String),
}