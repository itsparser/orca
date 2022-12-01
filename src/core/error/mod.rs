use actix_web::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix_web::http::StatusCode;

pub use base::error::{InternalResult, OrcaError, OrcaResult};
