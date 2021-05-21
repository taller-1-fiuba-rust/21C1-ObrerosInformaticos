mod listener_thread;
mod server;
mod threadpool;
mod storage;
use crate::storage::data_storage::DataStorage;

fn main() {
	//print_data();
    let addr = "127.0.0.1:1234".to_string();
    let mut server = server::Server::new(addr);
    server.run();
    server.join();
}

/*fn print_data(){
	println!("PRINTING DATA");
    let mut data_storage = DataStorage::new();
    data_storage.load_data("/home/dani/Documents/Taller de programación - Deymonnaz/Trabajo Practico Grupal/ObrerosInformaticos/src/storage/data.txt");

    data_storage.print_hash();
}*/