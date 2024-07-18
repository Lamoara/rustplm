use AsBytes;

pub struct Login{
    username: String,
    password: String
}

impl AsBytes for Login{
    fn as_bytes() -> [u8]{

    }
}

impl Login{
    pub fn new(username: String, password: String) -> Login{
        Login{
            username,password
        }
    }
}