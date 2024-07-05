//! libsql support for the `bb8` connection pool. Note that in-memory
//! databases aren't supported, since they are always per-connection, and
//! therefore don't make sense in a pool environment.
#![deny(missing_docs, missing_debug_implementations)]

use std::sync::Arc;

use async_trait::async_trait;
use bb8::ManageConnection;
use libsql::{Connection, Database, Error};

#[cfg(test)]
mod tests;

/// A `bb8::ManageConnection` implementation for `libsql::Connection`
/// instances.
#[derive(Clone, Debug)]
pub struct LibsqlConnectionManager(Arc<Database>);

impl LibsqlConnectionManager {
    /// Construct a new connection manager.
    pub fn new(database: Database) -> Self {
        Self(Arc::new(database))
    }
}

#[async_trait]
impl ManageConnection for LibsqlConnectionManager {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // let options = self.0.clone();

        self.0.connect()
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // Matching bb8-postgres, we'll try to run a trivial query here.
        conn.query("SELECT 1", ()).await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        // There's no real concept of a "broken" connection in SQLite: if the
        // handle is still open, then we're good. (And we know the handle is
        // still open, because Connection::close() consumes the Connection, in
        // which case we're definitely not here.)
        false
    }
}
