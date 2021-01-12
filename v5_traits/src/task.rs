pub trait TaskArgument: 'static + Send{}
impl<T> TaskArgument for T where T: 'static + Send{}
pub trait TaskFunction<T>: 'static + FnOnce(T) + Send{}
impl<T, U> TaskFunction<T> for U where T: TaskArgument, U: 'static + FnOnce(T) + Send{}

pub trait TaskRunner{
    fn run_task<T>(&self, task: impl TaskFunction<T>, task_argument: T) where T: TaskArgument;
}
