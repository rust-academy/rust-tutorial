use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct CustomSmartPointer<T: Display>{
    data: T,
}


pub type CSP<T> = CustomSmartPointer<T>;


impl<T: Display> CSP<T>  {
    pub fn new(data: T) -> Self{
        return CustomSmartPointer{data}
    }
}


impl<T: Display> Deref for CSP<T>{
    type Target = T;  // Defines an associated type for the CSP trait
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<T: Display> DerefMut for CSP<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}


impl<T: Display> Drop for CSP<T>{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


impl<T: Display> Display for CSP<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}


pub fn test_custom_smart_pointer() {
    let _a = CSP::new(String::from("my stuff"));
    println!("CSP _a created: {}", _a);

    {
        let _b = CSP::new(String::from("other stuff"));
        println!("CSP _b created");

        // drop cannot be called directly,
        // all you can do is call mem::drop to prevent double free
        std::mem::drop(_b);
    } // _b goes out of scope and drop is called automatically


    println!("CSP _a dropped.");
} // _a gets out of scope and calls drop() automatically


