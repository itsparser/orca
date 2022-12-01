use actix_web::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix_web::http::StatusCode;

mod base;

pub use base::{InternalResult, OrcaError, OrcaResult};
