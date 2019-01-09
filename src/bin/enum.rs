
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let _v4 = IpAddrKind::V4;
    let _v6 = IpAddrKind::V6;

    enum State {
        INPROGRESS,
        WORKED(String),
        DELETED(String, String),
        STORED{ by: String, when: String },
    }

    let _state_del = State::DELETED(String::from("user01"), String::from("10/10/2010"));
    let _state_sto = State::STORED { by: String::from("user01"), when: String::from("10/10/2010")};

    //println!("DEL: {}", _state_del[0]);
    //println!("STO: {}", _state_sto.by);

}