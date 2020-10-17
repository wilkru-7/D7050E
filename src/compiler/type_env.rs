use std::{collections::HashMap, collections::VecDeque};
use crate::ast_new::*;

pub type Error = String;

pub type IdType = HashMap<String, (Type, bool)>;

pub type VarEnv = VecDeque<IdType>;

// pub type FnArgs = HashMap<String, Type>;

pub type FnEnv = HashMap<String, Type>;

// pub type FnEnv = VecDeque<FnType>;
