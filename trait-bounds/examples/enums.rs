
#![allow(dead_code)]
// This example doesn't work due to imcomatible implementation of copy
#[derive(Debug, Clone, Copy)]
struct Person {
    id: i64,
    firstname: String,
    lastname: String,
}
enum Capability<T> {
    Read { id: i64},
    Create(T),
    Update(T),
    Delete(T),
}

impl Capability<Person> {
    fn action(&self, mut list: Vec<Person>) -> Person {
        match self {
            Capability::Create(s) => {
                let p = Person { 
                    id: rand::random(),
                    firstname: s.firstname.to_owned(),
                    lastname: s.lastname.to_owned()
                };
                list.push(p.clone());
                p
            },
            Capability::Read { id }=> { Person { id: *id, firstname: String::from("read"), lastname: String::from("Ready")}},
            Capability::Update(s) => {*s},
            Capability::Delete(s) => {*s},
        } 
    }
}

fn main() {
    let m = Capability::Create(Person { id: 0, firstname: String::from("Kenneth"), lastname: String::from("Fossen")});
    let mut list: Vec<Person> = vec![];
    m.action(list);

    println!("{:#?}", &list);
}