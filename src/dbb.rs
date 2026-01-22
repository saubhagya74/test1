

use sqlx::{Pool, Postgres};

pub async fn pgcon()-> Result<Pool<Postgres>,sqlx::Error>{

    let db_url = "postgresql://devuser:mypassword@192.168.1.97:5432/channelapi";
    
    let conn_pool =sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await;
    // match conn_pool{
    //         Ok(value)=>{println!("connected to database")},
    //         Err(e)=>{ println!("error connecting database {:?}",e)}
    // }
    return conn_pool;
}