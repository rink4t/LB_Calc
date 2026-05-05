// Notes(o･ω･o): I use the following pattern only for order
// Some struct and impl
// |------------{Some info ( . .)φ}----------------|
// another struct and impl



//|-----------------{Diagnostics ( . .)φ}------------------|

pub struct Diagnostic{
    msgs: Vec<String>,
    owner: String,
}

impl Diagnostic{
    pub fn new(owner_str: String) -> Diagnostic{
        Diagnostic{msgs: Vec::new(), owner: owner_str}
    }
}