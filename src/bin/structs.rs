
fn main() {
    struct Point {
        x: u32,
        y: u32
    }

    let _p1 = Point {
        x: 100,
        y: 200
    };

    let _p2 = Point{
        x: 150,
        y: 300
    };

    println!("x distance: {}", _p2.x - _p1.x);
    println!("y distance: {}", _p2.y - _p1.y);
    
    struct Envelope {
        id: u64,
        subject: String,
        created_on: String,
        created_by: String,
        urgent: bool,
    }

    let _enve1 = Envelope {
        id: 120,
        subject: String::from("Pratica 120"),
        created_on: String::from("22/06/2018 22:30:11"),
        created_by: String::from("mrossi"),
        urgent: true,
    };

    println!("Enve subject: {}", _enve1.subject);

    let _enve1Copy = Envelope {
        id: 121,
        .._enve1
    };

    println!("EnveCopy id: {}, subject: {}", _enve1Copy.id, _enve1Copy.subject);
}