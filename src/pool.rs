// struct Pool{
//     workers : Vec<Workers>,
//     rx : Recv<usize>,
// }



// struct Worker{
//     index : usize,
//     tx : Sender<usize>,
//     conn : PgConnection,
// }

// fn create_pool(db_url : String) -> 

use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use diesel::{PgConnection, Connection};

// pub struct UnsafePool{
//     workers : Vec<Worker>,
//     rx : Recv<usize>,
// }

// impl UnsafePool{
//     pub fn new(database_uri : String, count : usize) -> Self{
//         let mut workers = Vec::with_capacity(count);
//         for _i in 0..count{
//             let conn = diesel::PgConnection::establish(&db_url).expect("db connection failed");
//             workers.push_back(conn)
//         }
//         let (mut tx, mut rx ) = mpsc::channel();

//     }
// }

// pub struct Worker{
//     connection : PgConnection, 
//     index : usize,
//     tx : Sender<usize>,
// }

////// expensive lock impl ////
pub struct Pool{
    workers : Arc<Mutex<VecDeque<PgConnection>>>,
}

impl Clone for Pool{
    fn clone(&self) -> Self {
        Self{
            workers : self.workers.clone(),
        }
    }
}

impl Pool{
    pub fn new(db_url : String, count: usize) -> Result<Self, ()> {
        let mut vec_d = VecDeque::with_capacity(count);
        for _i in 0..count{
            let conn = diesel::PgConnection::establish(&db_url).expect("db connection failed");
            vec_d.push_back(conn)
        }

        Ok(Self{
            workers: Arc::new(Mutex::new(vec_d))
        })
    }

    pub async fn execute<F>(&self, func : F) -> Result<(),()>
    where
        F : FnOnce(&mut PgConnection) -> Result<(),()>,
    {
        match self.workers.try_lock(){
            Ok(mut guard_vec) =>{
                let conn = guard_vec.pop_back();
                drop(guard_vec);
                let mut r = Err(());
                match conn {
                    None => {
                        println!("no connection found");
                    },
                    Some(mut c) => {
                        //println!("starting function exec");
                        r = func(&mut c);
                        loop{
                            match self.workers.try_lock(){
                                Ok(mut guard) => {
                                    guard.push_back(c);
                                    drop(guard);
                                    break
                                },
                                Err(_) => println!("trying for lock")
                            }
                        }

                    }
                }
                r
            }
            Err(e) => {
                println!("lock attain error");
                Err(())
            }
        }
    } 
}