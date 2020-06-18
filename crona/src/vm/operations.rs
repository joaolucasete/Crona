#[macro_export]
macro_rules! operation {
    (RULE $self:ident $pattern:pat) => {
        $self.pop()
    };

    (RULE $self:ident $pattern:pat $(,$other:pat)*) => {
        ($self.pop()
            $(,operation!(RULE $self $other))*
        )
    };

    ($self:ident, ( $pattern:pat $(,$other:pat )* ) => push $thing:expr) => {
        if let ( $pattern $(,$other )* ) = operation!(RULE $self $pattern $(,$other)*) {
            $self.stack.push($thing)
        }else{

        }
    };
}
