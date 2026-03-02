// Modulo loans
// - Debe contener enum LoanStatus: Active, Returned
// - pub struct Loan: book: Book, user: User, status: LoanStatus
// - metodo return_book(&mut self)
enum LoanStatus {
    Active,
    Returned,
}
struct Loan {
    book: Book,
    user: User,
    status: LounStatus,
}
imp Loan {
    fn return_book(&mut self) {
        
    } 
}