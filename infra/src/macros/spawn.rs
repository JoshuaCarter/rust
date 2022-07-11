
/// Spawn a tokio async task with cloned and/or moved variables.
///
/// #### Format:
/// ```no_run
/// spawn!((/* cloned vars */) => { /* code */ });
/// ```
///
/// #### Usage example:
/// ```no_run
/// let data_stream = /*...*/;
/// let mpsc_sender = /*...*/;
/// let task_handle = spawn!((mpsc_sender, self.delay => delay) => {
///     // mpsc_sender is cloned, data_stream is moved
///     mpsc_sender.send(data_stream).await;
///     // self.delay is cloned as delay
///     tokio::time::sleep(Duration::from_millis(delay)).await;
/// });
/// // wait for task to complete
/// task_handle.await;
/// ```
///
/// #### Generated example:
/// ```no_run
/// tokio::spawn({
///     let mpsc_sender = mpsc_sender.clone();
///     let delay = self.delay.clone();
///     async move {
///         mpsc_sender.send(data_stream).await;
///         tokio::time::sleep(Duration::from_millis(delay)).await;
///     }
/// })
/// ```
#[macro_export]
macro_rules! spawn {
    (($($clones:tt)*) => $($code:tt)*) => {
        tokio::spawn(enclose::enclose!(($($clones)*) async move $($code)*))
    };
}
