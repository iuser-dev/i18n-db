#[macro_export]
macro_rules! tx_cf {
  ($self:ident)=>{
    let cf = &$self.cf;
    let tx = $self.db.transaction();

    macro_rules! define {
      ($op:ident) => {
        paste! {
          #[allow(unused_macros)]
          macro_rules! $op {
            ($cf:ident,$$$$($$$$args:expr),*) => {{
              tx.[<$op _cf>](&cf.$cf,$$$$($$$$args),*)?
            }};
          }
        }
      };
      ($$($$op:ident),+) => {
        $$(define!($$op);)+
      };
    }

    define!(put, get, delete, get_pinned);
    tx.commit()?;
  }
}
