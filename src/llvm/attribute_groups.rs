//! # Attribute Groups
//!
//! Attribute groups are groups of attributes that are referenced by
//! objects within the IR. They are important for keeping .ll files
//! readable, because a lot of functions will use the same set of
//! attributes. In the degenerative case of a .ll file that corresponds
//! to a single .c file, the single attribute group will capture the
//! important command line flags used to build that file.
//!
//! An attribute group is a module-level object. To use an attribute
//! group, an object references the attribute group’s ID (e.g. #37).
//! An object may refer to more than one attribute group. In that
//! situation, the attributes from the different groups are merged.
//!
//! [attribute-groups](https://llvm.org/docs/LangRef.html#attribute-groups)

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Personality(Vec<i32>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Attributes(pub i32, pub Vec<String>);

impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .1
            .iter()
            .fold(" ".to_string(), |s, x| format!("{s}{x} "));
        let s = format!("attributes #{} = {{{s}}}", self.0);
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .fold(String::new(), |s, x| format!("{s} #{x}"));
        write!(f, "{s}")
    }
}
