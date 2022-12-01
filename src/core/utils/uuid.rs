use rand::{distributions::Alphanumeric, thread_rng, Rng};

// pub fn short_uuid() -> String {
//     thread_rng().sample_iter(&Alphanumeric).take(8).collect()
// }
//
// pub fn request_uuid() -> String {
//     // .take(32).collect::<String>()
//     let mut rng = rand::thread_rng();
//     let mut uuid = String::new();
//     let _uuid : String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
//     format!("Rid{}", _uuid)
// }
