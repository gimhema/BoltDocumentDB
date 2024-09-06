
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

pub enum QueryType {
    NONE,
    NEW,
    RESTORE,
    BACKUP,
    REMOVE,
    CREATE,
    READ,
    UPDATE,
    DELETE,
    END
}

pub struct QueryResult {
    query_type : QueryType,
    bundle_name : String,
    document_name : String,
    key : i64,
    json_query : String
}

impl QueryResult {
    pub fn new(
        _query_type : QueryType,
        _bundle_name : String,
        _document_name : String,
        _key : i64,
        _json_query : String) -> Self {
            return QueryResult { 
                query_type: _query_type, bundle_name: _bundle_name,
                 document_name: _document_name, key: _key, json_query: _json_query }
        }
}

pub struct BoltInterpreter {
    input_query : String
}

impl BoltInterpreter {
    pub fn new() -> Self {
        return BoltInterpreter { input_query : "".to_string() }
    }

    pub fn set_input_query(&mut self, _query : String) {
        self.input_query = _query;
    }

    pub fn parse(&mut self) -> QueryResult {
        let mut _result = 
        QueryResult::new(QueryType::NONE, "".to_string(), "".to_string(), 0, "".to_string());

        let mut _query = self.input_query.clone();



        return _result
    }

    pub fn parse_prefix(&mut self, _prefix : String)  -> QueryType {
        let mut _result = QueryType::NONE;

        

        return _result
    }
}