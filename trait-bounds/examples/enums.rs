/*
#![allow(dead_code)]
// This example doesn't work due to imcomatible implementation of copy
#[derive(Debug)]
struct Person {
    id: i64,
    firstname: String,
    lastname: String,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        let f = self.firstname.to_string();
        let l = self.lastname.to_string();
        Self {
            id : self.id.clone(),
            firstname: f, 
            lastname: l,
        }
    }
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
*/

fn main() {}