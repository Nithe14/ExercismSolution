
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($x:expr => $y:expr ),+ $(,)?) => {
         {
             let mut tmp_map = ::std::collections::HashMap::new();
         $(
             tmp_map.insert($x, $y);
          )+
             tmp_map
         }
     };
    
}
