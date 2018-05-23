use futures::{Future,Stream,Async,Poll};
use tokio_timer::Interval;
use web3::types::{BlockNumber,Block,H256};
use web3::error::Error;
use web3::api::{Namespace,Eth};
use web3::helpers::CallResult;
use web3::Transport;


/// stream of incoming blocks
///
/// polls the `eth_getBlockByNumber` method on an interval.
/// inital poll is performed against `"latest"`, and all subsequent
/// poll are made against the highest block number seen incremented
/// by 1.
///
pub struct BlockStream<T,F> {
    interval: Interval,
    previous: Option<u64>,
    inner: Eth<T>,
    state: BlockStreamState<F>,
}


impl<T: Transport> BlockStream<T,T::Out> {


    pub fn new(interval: Interval, transport: T) -> Self {
        let (previous,state) = Default::default();
        let inner = Eth::new(transport);
        Self { interval, previous, inner, state }
    }

    fn get_next_work(&self) -> CallResult<Option<Block<H256>>,T::Out> {
        let block_number = match self.previous {
            Some(num) => BlockNumber::Number(num + 1),
            None => BlockNumber::Latest,
        };
        self.inner.block(block_number.into())
    }
}


impl<T: Transport> Stream for BlockStream<T,T::Out> {

    type Item = Block<H256>;

    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>,Self::Error> {
        loop {
            let next_state = match self.state {
                BlockStreamState::AwaitNextInterval => {
                    // poll inner interval & remap error if needed
                    let poll_rslt = self.interval.poll()
                        .map_err(|err| {
                            error!("in polling interval {:?}",err);
                            Error::from("timer failed for polling interval")
                        });
                    // explicitly discard returned `Instant`
                    let _ = try_ready_stream!(poll_rslt);
                    // if we got this far, interval has yielded
                    let work = Some(self.get_next_work());
                    BlockStreamState::GetNextBlock { work }
                },
                BlockStreamState::GetNextBlock { ref mut work } => {
                    if work.is_some() {
                        let query_result = try_ready!(work.as_mut()
                            .expect("always exists").poll());
                        // if we got this far, current work future has completed
                        // and must be explicitly discarded.
                        let _ = work.take();
                        // if a new block was successfully acquired, process it
                        if let Some(new_block) = query_result {
                            // perform basic checks & extract block number
                            let block_number = {
                                // we never request pending blocks, so `number` field should 
                                // always be `Some` (if not, an error has occurred).
                                if let Some(number) = new_block.number {
                                    let last_seen = number.low_u64();
                                    debug_assert!(number == last_seen.into(),"block number must be less than 2^64");
                                    if self.previous.map(|n| last_seen == n + 1).unwrap_or(true) {
                                        Ok(last_seen)
                                    } else {
                                        error!("non-sequential block {:?}",new_block);
                                        let message = "unexpected non-sequential block";
                                        Err(Error::from(message))
                                    }
                                } else {
                                    error!("expected block number {:?}",new_block);
                                    let message = "missing field `number` in non-pending block";
                                    Err(Error::from(message))
                                }
                            }?;
                            self.previous = Some(block_number);
                            // block is OK, yield to caller
                            return Ok(Async::Ready(Some(new_block)));
                        } else {
                            // next block not yet available, wait for next interval
                            BlockStreamState::AwaitNextInterval
                        }
                    } else {
                        // work future was already consumed, wait for next interval
                        BlockStreamState::AwaitNextInterval
                    }
                },
            };
            self.state = next_state;
        }
    }
}


#[derive(Debug)]
enum BlockStreamState<F> {
    AwaitNextInterval,
    GetNextBlock {
        work: Option<CallResult<Option<Block<H256>>,F>>,
    }
}

impl<F> Default for BlockStreamState<F> {

    fn default() -> Self { BlockStreamState::AwaitNextInterval }
}

