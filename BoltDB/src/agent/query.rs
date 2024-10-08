
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

// data object control detail
// get <BUNDLE_NALE> <DOCMUENT_NAME> <KEY> <JSON_KEY>
// set <BUNDLE_NALE> <DOCMUENT_NAME> <KEY> <JSON_KEY> <JSON_VALUE>

#[derive(Clone)]
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
    SET,
    GET,
    END
}

pub struct QueryResult {
    query_type : QueryType,
    bundle_name : String,
    document_name : String,
    key : i64,
    json_query : String,
    json_key : String,
    json_value : String
}

impl QueryResult {
    pub fn new(
        _query_type : QueryType,
        _bundle_name : String,
        _document_name : String,
        _key : i64,
        _json_query : String,
        _json_key : String,
        _json_value : String) -> Self {
            return QueryResult { 
                query_type: _query_type, bundle_name: _bundle_name,
                 document_name: _document_name, key: _key, json_query: _json_query,
                 json_key : _json_key, json_value : _json_value }
        }

    pub fn get_query_type(&self) -> QueryType {
        return self.query_type.clone()
    }

    pub fn get_bundle_name(&self) -> String {
        return self.bundle_name.clone()
    }

    pub fn get_document_name(&self) -> String {
        return self.document_name.clone()
    }

    pub fn get_key(&self) -> i64 {
        return self.key.clone()
    }

    pub fn get_json_query(&self) -> String {
        return self.json_query.clone()
    }

    pub fn get_json_key(&self) -> String {
        return self.json_key.clone()
    }

    pub fn get_json_value(&self) -> String {
        return self.json_value.clone()
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

    pub fn get_input_query(&self) -> String {
        return self.input_query.clone()
    }

    pub fn clear_input(&mut self) {
        self.input_query.clear();
    }

    fn get_first_word<'a>(&mut self, s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }

    fn get_divided_words<'a>(&mut self, s: &'a str) -> Vec<&'a str> {
        s.split_whitespace().collect()
    }
    

    pub fn parse(&mut self) -> QueryResult {
        let mut _result = 
        QueryResult::new(QueryType::NONE, "".to_string(), 
        "".to_string(), 0, "".to_string(),
        "".to_string(), "".to_string());

        let mut _query = self.input_query.clone();
    
        let mut _splited = self.get_divided_words(_query.as_str());

        let mut _prefix = _splited[0];
        let mut _queryType = self.parse_prefix(_prefix.to_string());
        
        // <BUNDLE_NALE> <DOCMUENT_NAME> <KEY> <JSON_QUERY>
        _result.query_type = _queryType;
        _result.bundle_name = _splited[1].to_string();
        _result.document_name = _splited[2].to_string();

        // _result.key = _splited[3].to_string();
        match _splited[3].to_string().parse::<i64>() {
            Ok(n) => _result.key = n,
            Err(e) => _result.key = 0,
        }

        _result.json_query = _splited[4].to_string();

        _result.json_key = _splited[5].to_string();
        _result.json_value = _splited[6].to_string();

        return _result
    }

    pub fn parse_prefix(&mut self, _prefix: String) -> QueryType {
        let mut _result = QueryType::NONE;
        
        match _prefix.to_lowercase().as_str() {
            "new" => _result = QueryType::NEW,
            "restore" => _result = QueryType::RESTORE,
            "backup" => _result = QueryType::BACKUP,
            "remove" => _result = QueryType::REMOVE,
            "create" => _result = QueryType::CREATE,
            "read" => _result = QueryType::READ,
            "update" => _result = QueryType::UPDATE,
            "delete" => _result = QueryType::DELETE,
            "set" => _result = QueryType::SET,
            "get" => _result = QueryType::GET,
            _ => _result = QueryType::NONE,
        }
    
        _result
    }
    

    pub fn empty_input(&self) -> bool {
        
        if self.input_query.is_empty() {
            return true
        }

        return false
    }
}
