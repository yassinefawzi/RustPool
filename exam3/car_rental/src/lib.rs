use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Car {
    pub color: String,
    pub plate: String,
}

#[derive(Debug)]
pub struct RentalBusiness {
    pub car: RefCell<Car>,
}

impl RentalBusiness {
    // Borrow immutable reference to car
    pub fn rent_car(&self) -> Ref<Car> {
        self.car.borrow()
    }

    // Take ownership of car (replace with default Car)
    pub fn sell_car(&self) -> Car {
        self.car.replace(Car::default())
    }

    // Borrow mutable reference to car
    pub fn repair_car(&self) -> RefMut<Car> {
        self.car.borrow_mut()
    }

    // Replace current car with a new one
    pub fn change_car(&self, new_car: Car) {
        self.car.replace(new_car);
    }
}
