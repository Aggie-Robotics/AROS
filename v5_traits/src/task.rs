use core::fmt::Display;
pub trait TaskFunction<T, O>: 'static + FnOnce(T) -> O + Send where T: 'static + Send, O: 'static + Send{}
impl<T, U, O> TaskFunction<T, O> for U where T: 'static + Send, U: 'static + FnOnce(T) -> O + Send, O: 'static + Send{}

pub trait TaskRunner<T, O> where T: 'static + Send, O: 'static + Send{
    type TaskTracker;
    fn run_task(&self, name: impl Display, task: impl TaskFunction<T, O>, task_argument: T) -> Self::TaskTracker;
}
