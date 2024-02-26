// The way changes behave has been changed, this change listener is no longer necessary, but I left it here as a guarantee of something that worked...

// use std::collections::hash_map::DefaultHasher;
// use std::fs::File;
// use std::hash::{Hash, Hasher};
// use std::io::{self, BufReader, Read};
// use std::thread;
// use std::time::Duration;
// use tauri::{AppHandle, Manager};
//
// pub fn theme_listener(app: AppHandle) -> io::Result<()> {
//     let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
//     let path = format!(
//         "{}/.local/share/com.nick.hyperpanel/style/hyperpanel.css",
//         home_dir
//     );
//
//     // Open file
//     let file = File::open(&path)?;
//     let mut reader = BufReader::new(file);
//
//     // Get inicial file hash
//     let mut hasher = DefaultHasher::new();
//     let mut buffer = Vec::new();
//     reader.read_to_end(&mut buffer)?;
//     buffer.hash(&mut hasher);
//     let mut current_hash = hasher.finish();
//
//     // Start thread to listen file changes
//     thread::spawn(move || {
//         loop {
//             // Pause the thread two seconds
//             thread::sleep(Duration::from_secs(5));
//
//             // Open file to calculate hash
//             let file = match File::open(&path) {
//                 Ok(file) => file,
//                 Err(e) => {
//                     println!("Error opening file: {:?}", e);
//                     continue;
//                 }
//             };
//             let mut reader = BufReader::new(&file);
//             let mut new_buffer = Vec::new();
//             match reader.read_to_end(&mut new_buffer) {
//                 Ok(_) => {
//                     let mut hasher = DefaultHasher::new();
//                     new_buffer.hash(&mut hasher);
//                     let new_hash = hasher.finish();
//
//                     // Compare hash to generate event
//                     if new_hash != current_hash {
//                         // Set new hash
//                         current_hash = new_hash;
//
//                         // Logic to handle theme change
//                         let _ = app.emit_all("theme-changed", &path);
//                     }
//                 }
//                 Err(e) => println!("Error reading file: {:?}", e),
//             }
//         }
//     });
//
//     Ok(())
// }
