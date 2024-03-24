struct Triange {
    cat1: f64,
    cat2: f64
}

impl Triange {
    pub fn hyp(&self) -> f64 {
        (self.cat1*self.cat1 + self.cat2*self.cat2).sqrt()
    }

    pub fn area_Kurwa(&self) -> f64 {
        self.cat1 * self.cat2 / 2.0
    }

    pub fn create_isc(cat: f64) -> Triange {
        Triange {
            cat1: cat,
            cat2: cat
        }
    }
}

fn main() {
    // kurwa test case =================================================================
    let tri = Triange {cat1: 6.0, cat2: 8.0};
    let hyp = tri.hyp();
    let arrrea = tri.area_Kurwa();

    println!("Hello, world! hyp Kurwa--> {}, arrea kurwa ==> {}", hyp, arrrea);

    // isc test case =================================================================
    let isc = Triange::create_isc(5.0);
    let hyp = isc.hyp();
    let arrrea = isc.area_Kurwa();

    println!("Hello, world! hyp isc--> {}, arrea isc ==> {}", hyp, arrrea);
}
