mod _perm {
    pub trait Permission {}
}

pub struct Permission;
impl _perm::Permission for Permission {}

macro_rules! define_perm {
    ($name:ident) => {
        paste::item! {
            pub trait [<$name T>]: _perm::Permission {}
            struct $name;
            impl _perm::Permission for $name {}
            impl [<$name T>] for $name {}
        }
    };
}

// 黒魔術
macro_rules! comb_perm {
    ($self:ident, $name:ident) => {
        impl<T, U> $name for $self<T, U>
        where
            T: $name,
            U: _perm::Permission,
        {
        }
        impl<T, U> $name for &$self<T, U>
        where
            T: _perm::Permission,
            U: $name,
        {
        }
    };
}

define_perm!(Read);
define_perm!(Delete);
define_perm!(Insert);

define_perm!(ReadMeta);
define_perm!(WriteMeta);

pub struct CombPerm<T, U>
where
    T: _perm::Permission,
    U: _perm::Permission,
{
    _left: T,
    _right: U,
}

impl<T, U> _perm::Permission for CombPerm<T, U>
where
    T: _perm::Permission,
    U: _perm::Permission,
{
}
impl<T, U> _perm::Permission for &CombPerm<T, U>
where
    T: _perm::Permission,
    U: _perm::Permission,
{
}

comb_perm!(CombPerm, ReadT);
comb_perm!(CombPerm, DeleteT);
comb_perm!(CombPerm, InsertT);

comb_perm!(CombPerm, ReadMetaT);
comb_perm!(CombPerm, WriteMetaT);

// structの初期化はユーザー側でしかできないようにしないとだめだからまだ未完成
// FnOnce(PermissionBuilder) -> T
// をユーザーに書いてもらってPermissionBuilderはcrate内からだけ初期化できるようにすれば大丈夫
pub struct PermissionBuilder<T>(T)
where
    T: _perm::Permission;

macro_rules! define_builder {
    ($self:ident, $name:ident) => {
        paste::item! {
            pub fn [<$name:snake>](self) -> $self<CombPerm<impl [<$name T>], T>> {
                $self(CombPerm {
                    _left: $name,
                    _right: self.0,
                })
            }
        }
    };
}

impl<T> PermissionBuilder<T>
where
    T: _perm::Permission,
{
    fn new() -> PermissionBuilder<Permission> {
        PermissionBuilder(Permission)
    }

    pub fn build(self) -> T {
        self.0
    }

    define_builder!(PermissionBuilder, WriteMeta);
    define_builder!(PermissionBuilder, ReadMeta);

    define_builder!(PermissionBuilder, Insert);
    define_builder!(PermissionBuilder, Delete);
    define_builder!(PermissionBuilder, Read);
}

pub trait Plugin<T: _perm::Permission> {
    const IDENT: &str;

    fn run(&mut self, perm: T);
}

pub struct Writer;
impl<T> Plugin<T> for Writer
where
    T: InsertT + DeleteT + WriteMetaT,
{
    const IDENT: &str = "writer";

    fn run(&mut self, _perm: T) {
        println!("Hello from {}", <Self as Plugin<T>>::IDENT);
    }
}

fn main() {
    let writer = Writer;
    writer.run(
        PermissionBuilder::new()
            .insert()
            .delete()
            .write_meta()
            .build(),
    );
}
