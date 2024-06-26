use std::cell::{RefCell, RefMut};

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().push(wpis);
    });
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String> {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| wpisy.borrow().clone())
}

#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| wpisy.borrow_mut().remove(id_wpisu));
}

#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize, nowy_wpis: String) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        let mut binding: RefMut<Vec<String>> = wpisy.borrow_mut();
        let mut wpis: Option<&mut String> = binding.get_mut(id_wpisu);
        let stary: &mut String = wpis.unwrap();
        *stary = nowy_wpis;
    });
}
