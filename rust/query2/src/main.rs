macro_rules! parse_tuple {
    ($tokens:ident, $( $ty:ty ),* $(,)?) => {
        (
            $(
                $tokens.next().unwrap().parse::<$ty>().unwrap()
            ),*
        )
    };
}

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
        {
            $(
                $value:expr =>
                ( $( $field_ty:ty ),* )
                    => ( $( $field_vr:ident ),* )
                    => { $($code:tt)* }
            )+
        }: $count:expr $(,)?
    ) => {
        for _ in 0..$count {
            let _kind = $tokens.next().unwrap().parse().unwrap();
            match _kind {
                $(
                    $value => {
                        let ( $( $field_vr ),* ) = parse_tuple!($tokens, $( $field_ty ),*);

                        $($code)*
                    },
                )*
                _ => unimplemented!(),
            }
        }
    };
    (
        {
            $(
                $value:expr =>
                ( $( $field_ty:ty ),* )
                    => ( $( $field_vr:ident ),* )
                    => { $($code:tt)* }
            )+
        }: $count:expr
        , $rest:tt
    ) => {
        for _ in 0..$count {
            let _kind = $tokens.next().unwrap().parse().unwrap();
            match _kind {
                $(
                    $value => {
                        let ( $( $field_vr ),* ) = parse_tuple!($tokens, $( $field_ty ),*);

                        $($code)*
                    },
                )*
                _ => unimplemented!(),
            }
        }

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
       q: usize,
       {
           1 => (usize, usize) => (c, x) => {
               println!("1 {c} {x}");
           }
           2 => (usize) => (k) => {
                println!("2 {k}");
           }
       }: q,
    }

    println!("{q}");
}
