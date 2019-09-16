use Timestamp;

// TODO: Just an experiment
pub trait Lifetime {
    fn start() -> Timestamp;
    fn finish() -> Option<Timestamp>;
}