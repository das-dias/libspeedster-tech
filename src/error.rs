/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 21-03-2022
* *[filename] errors.rs
* *[summary] Handle and Generate Errors
* ***********************************
*/

#[derive(Debug, Clone)]
/**
* Errors generated in the management of technology files
 */
pub enum ErrorContext{
    Import(String), // Error importing technology .tlef file
    Serialize(String), // Error serdes-serializing technology file 
    DeSerialize(String), // Error serdes-deserializing technology .json/.yaml file
    Unknown(String), // Unknown technology file attribute/parameter
}

/**
* *[name] Trait : ErrorHandler
* *[description] Error handling trait to manage custom generated errors from strings
*/
pub trait ErrorHandler
{
    type Error;
    
    /**
    * *[name] err
    * *[description] error trait type constructor
    * [input]
    * @par msg (String) : error message
    * [output]
    * @par Error (Error Type) : the error data type is returned
    */
    fn err(&self, msg: impl Into<String>) -> Self::Error;

    /**
    * *[name] fail
    * *[description] return a failure
    * [input]
    * @par msg (String) : error message
    * [output]
    * @par Result (Error Type) : the result of the error and its message
    */
    fn fail<T>(&self, msg : impl Into<String>) -> Result<T, Self::Error>
    {
        Err(self.err(msg)) // return the created error on fail
    }

    /**
    * *[name] unwrap
    * *[description] unwrap the Option: opt it is some value,
    * *              and return the created error if not
    * [input]
    * @par msg (String) : error message
    * [output]
    * @par Result (Error Type) : the result of the error and its message
    */
    fn unwrap<T>(&self, opt: Option<T>, msg: impl Into<String>) -> Result<T, Self::Error>
    {
        match opt
        {
            Some(val) => Ok(val),
            None => self.fail(msg),
        }
    }

    /**
    * *[name] assert
    * *[description] assert boolean conditions
    * [input]
    * @par msg (String) : error message
    * @par b_cond (bool) : boolean condition to assert
    * [output]
    * @par Result (Error Type) : the result of the assertion of the boolean condition
    */
    fn assert(&self, b_cond: bool, msg: impl Into<String>) -> Result<(), Self::Error>
    {
        match b_cond
        {
            true => Ok(()),
            false => self.fail(msg)
        }
    }
}