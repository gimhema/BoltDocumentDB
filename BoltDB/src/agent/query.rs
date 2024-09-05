
// document bundle control
// new <BUNDLE_NALE>
// restore <BUNDLE_NALE>
// backup <BUNDLE_NALE>
// remove <BUNDLE_NALE>


// data object control
// create <BUNDLE_NALE> <DOCMUENT_NAME> <JSON_QUERY>
// read <BUNDLE_NALE> <DOCMUENT_NAME> <KEY>
// update <BUNDLE_NALE> <DOCMUENT_NAME> <KEY> <JSON_QUERY>
// delete <BUNDLE_NALE> <DOCMUENT_NAME> <KEY>

pub struct BoltInterpreter {
    input_query : String
}

impl BoltInterpreter {
    pub fn new() -> Self {
        return BoltInterpreter { input_query : "".to_string() }
    }
}