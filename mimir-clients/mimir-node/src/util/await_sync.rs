use tokio_timer::Error as TimerError;
use tokio_timer::Interval;
//use tokio_timer::{Interval,TimerError};
use futures::{Future,Stream,Async,Poll};
use web3::error::{Error,ErrorKind};
use web3::types::{SyncState,SyncInfo};
use web3::Transport;
use util::{SyncReport,SyncReportFuture,Util};
use mimir_util::unix_time;


/// Future which resolves once the local node appears to be synced.
/// 
/// It cannot be known with absolute certainty if a blockchain node is
/// really fully synced.  As an approximation, this future resolves 
/// once three conditions are met simultaneously:
/// 
/// - local node is not currently performing a syncing operation
/// - local node is connected to at least one peer on the p2p net
/// - latest block's timestamp is less than three minutes old
/// 
/// It is doubtful that all three of the above would be true unless
/// the local node has either successfully synced, or is the victim
/// of an ongoing and successful sybil attack.  Preventing sybil attacks
/// is beyond the scope of responsibility for this future.  Sorry.
///
pub struct AwaitSync<T> where T: Transport {
    inner: Option<AwaitSyncInner<T>>,
    count: usize,
    state: AwaitSyncState<T>
}


impl<T> AwaitSync<T> where T: Transport {

    pub fn new(interval: Interval, util: Util<T>) -> Self {
        let inner = Some(AwaitSyncInner { interval, util });
        let state = AwaitSyncState::NextInterval;
        let count = 0;
        AwaitSync { inner, count, state }
    }
}


fn convert_timer_error(error: TimerError) -> Error {
    let message = format!("timer failed with `{}`",error); 
    Error::from(ErrorKind::Msg(message))
}


impl<T> Future for AwaitSync<T> where T: Transport {

    type Item = SyncReport;

    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        loop {
            let next_state = match self.state {
                AwaitSyncState::NextInterval => {
                    let inner = self.inner.as_mut()
                        .expect("always exists in this state");
                    let _ = try_ready!(inner.interval.poll()
                        .map_err(convert_timer_error));
                    let work = inner.util.sync_report();
                    AwaitSyncState::Report { work }
                },
                AwaitSyncState::Report { ref mut work } => {
                    let report = try_ready!(work.poll());
                    if self.count % 32 == 0 {
                        log_sync(&report);
                        //info!("peers-count: {:<10?} | sync-state: {:?}",report.peer_count,report.sync_state);
                    }
                    self.count += 1;
                    if let SyncState::NotSyncing = report.sync_state {
                        if report.peer_count > 0x0.into() {
                            let is_recent = match report.last_block {
                                Some(ref block) => {
                                    let cutoff_time = unix_time() - 180;
                                    block.timestamp > cutoff_time.into()
                                },
                                None => false,
                            };
                            if is_recent {
                                return Ok(Async::Ready(report));
                            }
                        }
                    }
                    AwaitSyncState::NextInterval
                }
            };
            self.state = next_state;
        }
    }
}


struct AwaitSyncInner<T> {
    interval: Interval,
    util: Util<T>
}


enum AwaitSyncState<T> where T: Transport {
    NextInterval,
    Report {
        work: SyncReportFuture<T>
    },
}



fn log_sync(report: &SyncReport) {
    let &SyncReport { ref sync_state, ref peer_count, .. } = report;
    match *sync_state {
        SyncState::Syncing(ref info) => {
            let &SyncInfo { ref starting_block, ref current_block, ref highest_block } = info;
            info!("peers: {:<10?}  state: syncing {}...{} (est. highest: {})",
                peer_count,starting_block,current_block,highest_block);
        },
        SyncState::NotSyncing => {
            info!(r#"peers: {:<10?}  state: not syncing"#,peer_count);
        }
    }
}
