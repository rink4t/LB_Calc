
#[derive(Default)]

//|-----------------{Properties ( . .)φ}------------------|
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

    pub fn set_properties(&mut self, truth_vec: &Vec<bool>, equival: bool){
        match truth_vec.first() {
            Some(truval) => {
                if self.is_all_same(truth_vec){
                    if *truval{
                        self.tautology = true;
                        self.satisfactory = true;
                    }else { self.contradiction = true }
                }
            },
            None => {}
        }

        if !self.contradiction{ self.satisfactory =  true; }

        if !self.tautology && !self.contradiction { self.contingent = true; }

        self.equivalent = equival;
    }
}