pub trait PersonTrait {
    fn talk(&mut self, speach: String);
    fn get_name(&self) -> String;
    fn new(speach: String, name: String) -> Self;
}

pub struct PersonStruct {
    pub(crate) speach: String,
    pub(crate) name: String,
}

impl PersonTrait for PersonStruct {
    fn talk(&mut self, speach: String) {
        self.speach = speach;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn new(speach: String, name: String) -> Self {
        Self { speach, name }
    }
}

pub fn handle_person(speach: String, name: String) -> String {
    let person = PersonStruct::new(speach, name);

    return person.get_name();
}
