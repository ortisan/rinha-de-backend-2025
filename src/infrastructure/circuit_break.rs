enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

pub trait CircuitBreaker {
    fn get_state(&self) -> CircuitBreakerState;
    fn open(&self);
    fn close(&self);
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn is_half_open(&self) -> bool;
    fn execute<T, E>(&self, f: fn() -> Result<T, E>) -> Result<T, E>;
}
