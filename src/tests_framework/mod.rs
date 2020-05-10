use once_cell::sync::{OnceCell};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    //result::Result as DResult,
};
use tokio_diesel::*;

mod tests;

#[cfg(test)]
static dbInitFlag: OnceCell<bool> = OnceCell::new();

#[cfg(test)]
pub static DBCONNPOOL: OnceCell<Pool<ConnectionManager<diesel::SqliteConnection>>> = OnceCell::new();

#[cfg(test)]
pub async fn transaction_inner<T: 'static + std::marker::Send, F>(f: F) -> std::result::Result<T, crate::lib::PoolError>
where
    F: 'static + FnOnce(&diesel::SqliteConnection) -> QueryResult<T> + Send,
{
    let pool = DBCONNPOOL.get_or_init(||{
        let manager = ConnectionManager::<diesel::SqliteConnection>::new(":memory:");
    	let pool = Pool::builder().build(manager).unwrap();
    	pool
    });


    if let None = dbInitFlag.get() {
        let r = {
            pool.transaction(|c|{
                    // use diesel_migrations::embed_migrations;
                    embed_migrations!();
                    embedded_migrations::run(c).unwrap();
                Ok(())
            })
        };
        dbInitFlag.set(true);
    };

    let res = pool.transaction(f).await;
	match res {
		Ok(data) => {
			Ok(data)
		},
		Err(AsyncError::Checkout(_)) => {
			Err(crate::lib::PoolError{msg: "Pool error"})
		},
		Err(AsyncError::Error(_)) => {
			Err(crate::lib::PoolError{msg: "Diesel error"})
		},
	}
}
