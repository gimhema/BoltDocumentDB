use mio::Token;





pub struct BoltDBUser {
    name : String,
    password : String,
    stream_token : Token
}

impl BoltDBUser {
    pub fn new_zero() -> Self {
        return BoltDBUser { name: "".to_string(), password: "".to_string(), stream_token: Token(0) }
    }

    pub fn new(_name : String, _password : String, _stream_token : Token) -> Self {
        return BoltDBUser { name: _name, password: _password, stream_token: _stream_token }
    }

    pub fn set_name(&mut self, _name : String) {
        self.name = _name.clone();
    }

    pub fn set_password(&mut self, _password : String) {
        self.password = _password.clone();
    }

    pub fn set_token(&mut self, _token : Token) {
        self.stream_token = _token.clone();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone()
    }

    pub fn get_pass(&self) -> String {
        return self.password.clone()
    }

    pub fn get_token(&self) -> Token {
        return self.stream_token.clone()
    }
}


