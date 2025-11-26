use std::fmt;

#[derive(Debug, Clone)]
pub enum ShindenError {
    HtmlParsing,
    WrongInput,
    NetworkError
}

impl fmt::Display for ShindenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ShindenError::HtmlParsing => write!(f, "Problem ocured during html parsing"),
           ShindenError::WrongInput => write!(f, "Invalid input"),
           ShindenError::NetworkError => write!(f, "Network Error")
       }
    }
}

