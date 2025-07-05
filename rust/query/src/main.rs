#[macro_export]
macro_rules! parse_tuple {
    ($tokens:ident, ($first:ty)) => {
        ($tokens.next().unwrap().parse::<$first>().unwrap())
    };
    ($tokens:ident, ($first:ty, )) => {
        ($tokens.next().unwrap().parse::<$first>().unwrap(),)
    };
    ($tokens:ident, ($first:ty, $second:ty $(,)?)) => {
        (
            $tokens.next().unwrap().parse::<$first>().unwrap(),
            $tokens.next().unwrap().parse::<$second>().unwrap(),
        )
    };
    ($tokens:ident, ($first:ty, $second:ty, $third:ty $(,)?)) => {
        (
            $tokens.next().unwrap().parse::<$first>().unwrap(),
            $tokens.next().unwrap().parse::<$second>().unwrap(),
            $tokens.next().unwrap().parse::<$third>().unwrap(),
        )
    };
    ($tokens:ident, ($first:ty, $second:ty, $third:ty, $fourd:ty $(,)?)) => {
        (
            $tokens.next().unwrap().parse::<$first>().unwrap(),
            $tokens.next().unwrap().parse::<$second>().unwrap(),
            $tokens.next().unwrap().parse::<$third>().unwrap(),
            $tokens.next().unwrap().parse::<$fourd>().unwrap(),
        )
    };
}

#[macro_export]
macro_rules! query {
    // simple binding
    ($tokens:ident, $name:ident : $ty:ty $(,)?) => {
        let $name = $tokens.next().unwrap().parse::<$ty>().unwrap();
    };
    ($tokens:ident, $name:ident : $ty:ty , $($rest:tt)*) => {
        let $name = $tokens.next().unwrap().parse::<$ty>().unwrap();
        query!($tokens , $($rest)*);
    };
    // query bind
    (
        $tokens:ident,
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
                            $( parse_tuple!($tokens, ($field_ty)) ),*
                        ),
                    )*
                    _ => unimplemented!(),
                });

        }
        let $name = $name;
    };
    (
        $tokens:ident,
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
                            $( parse_tuple!($tokens, ($field_ty)) ),*
                        ),
                    )*
                    _ => unimplemented!(),
                });

        }
        let $name = $name;

        query!($tokens , $($rest)*);
    };
    // 停止
    ($tokens:ident $(,)?) => {};
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
