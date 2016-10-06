use std::net::{TcpListener, ToSocketAddrs};
use net::Result;

/// A structure representing a GraphiteDb socket server.
///
/// # Examples
///
/// ```
/// use graphite_db::net;
///
/// let listener = net::Listener::new("[::]:0").unwrap();
/// ```
pub struct Listener {
    socket: TcpListener,
}

impl Listener {
    /// Constructs a new `Listner<T: ToSocketAddrs>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graphite_db::net;
    ///
    /// let listener = net::Listener::new("[::]:0").unwrap();
    /// ```
    pub fn new<T: ToSocketAddrs>(address: T) -> Result<Listener> {
        let socket = try!(TcpListener::bind(address));
        Ok(Listener { socket: socket })
    }
}
