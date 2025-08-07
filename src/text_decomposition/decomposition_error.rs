#[derive(Debug)]
pub struct WtdError {
    pub(crate) msg: &'static str,
}

impl std::fmt::Display for WtdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// nothing needed in the impl block
impl std::error::Error for WtdError {}

pub const LEN_DIFF: WtdError = WtdError {
    msg: "Left Text and Right Text haven't the same size",
};

pub const NO_DIFF: WtdError = WtdError {
    msg: "Left Text and Right Text are the same",
};

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        todo!("Faire les TU");
        //assert_eq!(1, 1);
    }
}
