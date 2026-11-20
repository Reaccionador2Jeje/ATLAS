use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtlasEvent { pub sequence: u64, pub topic: String, pub source: String, pub payload: String }

#[derive(Debug)]
pub struct EventQueue { capacity: usize, next_sequence: u64, queue: VecDeque<AtlasEvent> }
impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        Self { capacity: capacity.max(1), next_sequence: 1, queue: VecDeque::new() }
    }
    /// Bounded queue; when full, the oldest unconsumed event is discarded.
    pub fn publish(&mut self, topic: impl Into<String>, source: impl Into<String>, payload: impl Into<String>) -> u64 {
        if self.queue.len() == self.capacity { self.queue.pop_front(); }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.queue.push_back(AtlasEvent { sequence, topic: topic.into(), source: source.into(), payload: payload.into() });
        sequence
    }
    pub fn drain(&mut self) -> impl Iterator<Item = AtlasEvent> + '_ { self.queue.drain(..) }
    pub fn len(&self) -> usize { self.queue.len() }
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_is_bounded_and_sequences_events() {
        let mut q = EventQueue::new(2);
        q.publish("a", "test", "{}"); q.publish("b", "test", "{}"); q.publish("c", "test", "{}");
        let events = q.drain().collect::<Vec<_>>();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].sequence, 2);
        assert_eq!(events[1].sequence, 3);
    }
}
