#[cfg(not(madsim))]
pub use tokio::*;

#[cfg(madsim)]
pub use self::sim::*;
#[cfg(madsim)]
mod sim {
    // no mod `runtime`
    pub mod runtime;
    // TODO: simulate `task_local`

    // simulated API
    #[cfg(feature = "rt")]
    pub use madsim::task::spawn;
    #[cfg(feature = "time")]
    pub use madsim::time;
    #[cfg(all(feature = "rt", feature = "macros"))]
    pub use madsim::{main, test};
    pub use madsim::{net, task};

    pub mod net;

    // not simulated API
    // TODO: simulate `fs`
    #[cfg(feature = "fs")]
    pub use tokio::fs;
    #[cfg(feature = "process")]
    pub use tokio::process;
    #[cfg(feature = "signal")]
    pub use tokio::signal;
    #[cfg(feature = "sync")]
    pub use tokio::sync;
    pub use tokio::{io, pin};
    #[cfg(feature = "macros")]
    pub use tokio::{join, select, try_join};
}
