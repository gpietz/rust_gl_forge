pub struct OperationStatus<T> {
    success: bool,
    error: Option<T>,
}

impl<T> OperationStatus<T> {
    pub fn new(success: bool, error: Option<T>) -> OperationStatus<T> {
        Self { success, error }
    }

    pub fn new_success() -> OperationStatus<T> {
        Self {
            success: true,
            error: None,
        }
    }
    
    pub fn new_error(error: T) -> OperationStatus<T> { 
        Self { 
            success: false,
            error: Some(error),
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }
    
    pub fn error(&self) -> Option<&T> {
        self.error.as_ref()
    }
}
