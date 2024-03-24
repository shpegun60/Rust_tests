enum Person{
    Adult,
    Underage
}

enum Say {
    Hi(String),
    Bye(String),
    GoodMorning(String),
    GoodAfternoon(String),
    GoodEvening(String)
}

enum Math {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64)
}

impl Math {
    fn math_operations(&self) -> f64 {
        match self {
            Math::Add(x, y) => {
               x + y
            },
            Math::Sub(x, y) => {
               x - y
            },
            Math::Mul(x, y) => {
                x * y
            },
            Math::Div(x, y) => {
               x / y
            }
        }
    }
}

fn main() {
    let person = Person::Underage;

    match person {
        Person::Adult => {
            println!("intake:Adult");
            println!("idz w hui");
        },

        Person::Underage => {
            println!("intake:Underage");
            println!("ooo Kurwa!!!");
        }
    };
    //----------------------------------------------------------------
    let say = Say::Bye(String::from("hui"));

    match say {
        Say::Hi(name) => {
            println!("Hi {}", name);
        },

        Say::Bye(name) => {
            println!("Bye {}", name);
        },

        Say::GoodMorning(name) => {
            println!("GoodMorning {}", name);
        },

        Say::GoodAfternoon(name) => {
            println!("GoodAfternoon {}", name);
        },

        Say::GoodEvening(name) => {
            println!("GoodEvening {}", name);
        }
    };
    //---------------math operation-------------------------------------------------
    let math = Math::Add(10.0, 2.0);

    match math {
        Math::Add(x, y) => {
            println!("x + y = {}", x + y);
        },
        Math::Sub(x, y) => {
            println!("x - y = {}", x - y);
        },
        Math::Mul(x, y) => {
            println!("x * y = {}", x * y);
        },
        Math::Div(x, y) => {
            println!("x / y = {}", x / y);
        }
    };

    println!("from professional func --> {}", math.math_operations());
}

