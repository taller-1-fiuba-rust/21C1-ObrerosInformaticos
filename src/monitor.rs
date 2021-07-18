use crate::client::Client;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

/// Struct Monitor. It is composed of a
/// Vector which stores clients to send information.
pub struct Monitor {
    clients: RwLock<Vec<Arc<Client>>>,
}

/// Implementation of the Monitor structure.
impl Monitor {
	/// Create the Monitor structure. Initially with no clients.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::Monitor;
    /// let monitor = Monitor::new();
    /// ```
    ///
    pub fn new() -> Self {
        Monitor {
            clients: RwLock::new(vec![]),
        }
    }

    /// Send the executed command to the clients 
    /// that are active.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use proyecto_taller_1::Monitor;
    /// let monitor = Monitor::new();
    /// monitor.send("Keys *".to_string());
    /// ```
    ///
    pub fn send(&self, msg: &str) -> Result<(), &'static str> {
    	let lock_clients = self.clients.write().ok().ok_or("Failed to lock database")?;
    	for client in lock_clients.iter() {
    		match client.send(msg) {
    			Ok(_) => continue,
    			Err(s) => return Err(s),
    		}
    	};
    	Ok(())
    }

    /// Add a client to the clients list
    pub fn add(&self, client: Arc<Client>) -> Result<(), &'static str> {
    	let mut lock = self.clients.write().ok().ok_or("Failed to lock database")?;
    	lock.push(client);
    	Ok(())
    }

    pub fn remove(&self, client: Arc<Client>) -> Result<(), &'static str>{
        let mut lock = self.clients.write().ok().ok_or("Failed to lock database")?;
        self.do_remove(client, &mut lock)
    }

    fn do_remove(&self, client: Arc<Client>, lock: &mut RwLockWriteGuard<Vec<Arc<Client>>>) -> Result<(), &'static str>{
        if let Some(pos) = lock.iter().position(|x| *x == client) {
            lock.remove(pos);
        }   
        Ok(())
    }

    /// Return true if the monitor is active or false otherwise.
    pub fn is_active(&self) -> bool {
    	let mut lock = self.clients.write().unwrap();
        let mut clients: Vec<Arc<Client>> = vec![];
        for client in lock.iter() {
            if client.is_closed(){
                clients.push(client.clone());
            }
        }
        for client in clients{
            self.do_remove(client, &mut lock).unwrap();
        }
    	!lock.is_empty()
    }
}
