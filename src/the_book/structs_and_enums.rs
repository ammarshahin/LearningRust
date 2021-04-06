#![allow(dead_code, unused_variables)]

// tuple structs
struct Color(i32, i32, i32);
struct Point(f32, f32, f32);

// Normal structs
#[derive(Debug)] // the annotation #[derive(Debug)] is used to enable the debug print of the structure info
struct User {
    name: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

// implementation block.
impl User {
    // constructor
    // new is an associated functions (because it doesn't take &self as an parameter) and it's called with the >> StructName::associated_fun();
    // the :: syntax is used for both associated functions and namespaces created by modules.
    //* Note: if the parameters name is the same as the fields name we can use it directly (like: sign_in_count, active)
    fn new(name: &str, email: &str, sign_in_count: u32, active: bool) -> Self {
        User {
            name: String::from(name),
            email: String::from(email),
            // Because the sign_in_count field and the sign_in_count parameter have the same name,
            // we only need to write sign_in_count rather than sign_in_count: sign_in_count.....
            sign_in_count,
            // Same as the active field
            active,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_email(&self) -> &str {
        &self.email
    }
}

pub fn run() {
    let mut u = User {
        name: String::from("Ammar Shahin"),
        email: String::from("ammar.shahin@NotRealDomain.com"),
        sign_in_count: 0,
        active: true,
    }; // construct using the struct implicit deceleration

    u.name = String::from("Ammar M.Shahin");
    println!("the user name {}", u.get_name());

    // construct using the explicit deceleration
    let u2 = User::new("AmmarShahin", "ammar.shahin@NotRealDomain.com", 0, true);
    println!("the user name {}", u2.get_name());

    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let u3 = User {
        name: String::from("another Ammar Shahin"),
        ..u2 // fill the remaining fields with the corresponding u2 struct fields
    }; // But note now that the u2 has been borrowed after partial move and can't be used anymore

    println!("the user email {}", u3.get_email());
    println!("the user {:#?}", u3); // using the debug trait to print the struct info [simple = {:?} , detailed = {:#?}]

    // tuple structs
    let black = Color(10, 20, 30);
    let origin = Point(1.0, 4.0, 6.0);
    println!("Color : ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin : ({}, {}, {})", origin.0, origin.1, origin.2);
}
