macro_rules! query {
    // simple binding
    ($tokens:expr, $name:ident : $ty:ty $(,)?) => {
        let $name = $tokens.next().unwrap().parse::<$ty>().unwrap();
    };
    ($tokens:expr, $name:ident : $ty:ty , $($rest:tt)*) => {
        let $name = $tokens.next().unwrap().parse::<$ty>().unwrap();
        query!($tokens , $($rest)*);
    };
    // query bind
    (
        $tokens:expr,
        $name:ident $enum_name:ident : {
             $( $variant:ident : $value:expr => ( $( $field_ty:ty ),* ) ),* $(,)?
        } : $count:expr
        $(,)?
    ) => {
        enum $enum_name {
                $( $variant ( $( $field_ty ),* ) ),*
        }

        let mut $name: Vec<$enum_name> = vec![];
        for _ in 0..$count {
            let _kind = $tokens.next().unwrap().parse().unwrap();
                $name.push(match _kind {
                    $(
                        $value => $enum_name::$variant(
                            $( $tokens.next().unwrap().parse::<$field_ty>().unwrap() ),*
                        ),
                    )*
                    _ => unimplemented!(),
                });

        }
        let $name = $name;
    };
    (
        $tokens:expr,
        $name:ident $enum_name:ident : {
             $( $variant:ident : $value:expr => ( $( $field_ty:ty ),* ) ),* $(,)?
        } : $count:expr
        , $($rest:tt)*
    ) => {
                enum $enum_name {
                $( $variant ( $( $field_ty ),* ) ),*
        }

        let mut $name: Vec<$enum_name> = vec![];
        for _ in 0..$count {
            let _kind = $tokens.next().unwrap().parse().unwrap();
                $name.push(match _kind {
                    $(
                        $value => $enum_name::$variant(
                            $( $tokens.next().unwrap().parse::<$field_ty>().unwrap() ),*
                        ),
                    )*
                    _ => unimplemented!(),
                });

        }
        let $name = $name;

        query!($tokens , $($rest)*);
    };
    // 停止
    ($tokens:expr $(,)?) => {};
}

#[allow(unused)]

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    query! {
        stdin,
        a: usize,
        q Q: {
            One: 1 => (usize),
            Two: 2 => (usize, usize),
        }: a,
        b: usize,
        q2 Q2: {
            One: 1 => (usize),
            Two: 2 => (usize),
        }: a,
        c: usize,
    }

    println!("{a}");
}
