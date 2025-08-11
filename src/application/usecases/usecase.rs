use crate::infrastructure;

pub trait Usecase<I, O> {
    async fn execute(&self, i: I) -> infrastructure::Result<O>;
}
