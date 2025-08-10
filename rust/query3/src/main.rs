macro_rules! parse_value {
    ($tokens:ident, [$inner:tt; $id:expr]) => {{
        let mut v = Vec::with_capacity($id);
        for _ in 0..$id {
            v.push(parse_value!($tokens, $inner));
        }
        v
    }};

    ($tokens:ident, ( $( $ty:ty ),* ) $(,)?) => {
        (
            $(
                $tokens.next().unwrap().parse::<$ty>().unwrap()
            ),*
        )
    };


    ($tokens:ident, $t:ty) => {{ $tokens.next().unwrap().parse::<$t>().unwrap() }};
}

macro_rules! query {
    // simple binding
    ($tokens:ident, $name:ident : $ty:tt $(,)?) => {
        let $name = parse_value!($tokens, $ty);
    };

    ($tokens:ident, $name:ident : $ty:tt , $($rest:tt)*) => {
        let $name = parse_value!($tokens, $ty);
        query!($tokens , $($rest)*);
    };
    // query bind
    (
        $tokens:ident,
        {
            $(
                $value:expr =>
                ( $( $field_ty:tt ),* )
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
                        let ( $( $field_vr ),* ) = parse_value!($tokens, ( $( $field_ty ),* ));

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
                        let ( $( $field_vr ),* ) = parse_value!($tokens, ( $( $field_ty ),* ));

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
    // let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let stdin = r#"
5
1 1
2 2
3 3
4 4
5 5
1 1
2 2
3 3
4 4
5 5
Helllo
1 1 1
1 2 2
2 3
2 4
1 5 5
"#;
    let mut stdin = stdin.split_whitespace();

    query! {
        stdin,
        q: usize,
        a: [[(usize, usize); 2]; q],
        s: String,
        {
            1 => (usize, usize) => (c, x) => {
                println!("1 {c} {x}");
            }
            2 => (usize) => (k) => {
                println!("2 {k}");
            }
        }: q,
    }

    dbg!(q);
    dbg!(a);
    dbg!(s);
}
