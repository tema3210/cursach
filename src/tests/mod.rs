#[cfg(test)]
mod init_tests {
    #[test]
    fn test_cell(){
        assert_eq!(crate::lib::DBCONNPOOL.get().is_some(),false)


    }
}
