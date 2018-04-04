use futures::{Future,Stream,Async,Poll};
use redis_async::resp::{RespValue,FromResp};
use redis_async::error::Error;
use redis_async::client::paired::{
    PairedConnection,
    SendBox,
};


/// stream which "watches" a blocking redis command.
///
/// this stream will repeatedly execute a specified redis command,
/// yielding its output.  the primary purpose of this object is to
/// assist in building asynchronous consumers of list data via
/// the `BRPOP` (blocking right-handed pop) command.
///
pub struct WatchCommand<T> {
    value: RespValue,
    conn: PairedConnection,
    work: Option<SendBox<T>>,
}


impl<T> WatchCommand<T> {

    pub fn new(value: RespValue, conn: PairedConnection) -> Self {
        let work = Default::default();
        WatchCommand { value, conn ,work }
    }
}


impl WatchCommand<(String,String)> {

    /// BRPOP -- blocking right-handed pop against one or more lists.
    ///
    pub fn brpop<S: AsRef<str>>(lists: &[S], conn: PairedConnection) -> Self {
        let mut values: Vec<RespValue> = Vec::new();
        values.push("BRPOP".into());
        for elem in lists.iter() { values.push(elem.as_ref().into()); }
        values.push("0".into());
        Self::new(RespValue::Array(values),conn)
    }
}


impl<T> WatchCommand<T> where T: FromResp + Send + 'static {

    /// get mutable reference to current work future,
    /// creating one if none exists.
    #[inline]
    fn get_work_mut(&mut self) -> &mut SendBox<T> {
        if self.work.is_none() {
            self.work = Some(self.conn.send(self.value.clone()));
        }
        self.work.as_mut().expect("always exists")
    }
}


impl<T> Stream for WatchCommand<T> where T: FromResp + Send + 'static {

    type Item = T;

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        // get a mutable reference to current work future & poll.
        let item = try_ready!(self.get_work_mut().poll());
        
        // if we get this far, then the work future has been depleted
        // and can be discarded.
        let _ = self.work.take();

        // yield return item.
        Ok(Async::Ready(Some(item)))
    }
}


