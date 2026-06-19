//|-----------------{Properties ( . .)φ}------------------|

#[derive(Default)]
pub struct Properties{
    pub tautology: bool,
    pub contradiction: bool,
    pub satisfactory: bool,
    pub contingent: bool,
    pub equivalent: bool
}

impl Properties {
    fn is_all_same<T: PartialEq>(&mut self, slice: &[T]) -> bool{
        slice.first().map_or(true, |first| slice.iter().all(|x| x == first))
    }

    pub fn set_properties(&mut self, truth_vec: &Vec<bool>){

        if self.is_all_same(truth_vec){
            match truth_vec.first() {
                Some(value) => {
                    if *value {
                        self.tautology = true;
                    }else {
                        self.contradiction = true;
                    }
                },
                None => {unreachable!()}
            }
        }

        if !self.contradiction{ self.satisfactory =  true; }

        if !self.tautology && !self.contradiction { self.contingent = true; }

    }

    pub fn set_properties_equivexpr(&mut self, eqtruth_vec: &Vec<bool>, expr_a: &Vec<bool>, expr_b: &Vec<bool>) {
        if self.is_all_same(expr_a) && self.is_all_same(expr_b) {
            let val_a = if let Some(val) = expr_a.first() { *val } else {unreachable!()};
            let val_b = if let Some(val) = expr_b.first() { *val } else {unreachable!()};

            if (val_a == true) && (val_b == true) {
                self.tautology = true;
            }else if (val_a == false) && (val_b == false) {
                self.contradiction = true;
            }
        }

        if self.is_all_same(eqtruth_vec) {
            self.equivalent = true;
        }else { self.equivalent = false }

        if !self.contradiction { self.satisfactory = true; }

        if !self.tautology && !self.contradiction { self.contingent = true; }
    }
}