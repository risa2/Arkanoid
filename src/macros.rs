#[macro_export]
macro_rules! append {
	($list:expr; $( $x:expr ),* ) => {
		{
			let mut tmp=$list;
			$(
				tmp.push($x);
			)*
			tmp
		}
	};
}

#[macro_export]
macro_rules! new {
	($type:ident; $( $x:expr ),* ) => (
		Box::new( $type::new( $($x, )* ) )
	)
}

#[macro_export]
macro_rules! with {
	($value:expr => $name:ident; $code:block) => {
		{
			let $name=$value;
			$code
		}
	};
	($value:expr => mut $name:ident; $code:block) => {
		{
			let mut $name=$value;
			$code
		}
	};
}

#[macro_export]
macro_rules! if_true {
	($cond:expr; $code:stmt) => {
		if $cond {
			$code;
		}
	};
}