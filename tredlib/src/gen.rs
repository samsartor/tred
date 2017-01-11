#[macro_export]
macro_rules! _tredgen_append {
    ($pos:ident, $text:ident, $out:expr, $res:ident) => {
        {
            let __r = $res;
            $text = __r.0;
            $pos = __r.1;
            if __r.2.is_some() { $out(&mut __r.2.as_mut().unwrap()); }
        }
    };
}


#[macro_export]
macro_rules! _tredgen_match_str {
    ($pos:ident, $text:ident, $x:expr) => {
        {
            let __tmp = $x;
            let __len = tmp.len();
            if $text.starts_with(__tmp) {
                ::std::result::Result::Ok((&$text[__len..], $pos + __len, None))
            } else {
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos});
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_match_regex {
    ($pos:ident, $text:ident, $x:expr) => {
        {
            if let ::std::option::Option::Some((_, end)) = $x.find($text) {
                ::std::result::Result::Ok((&$text[end..], $pos + end, None))
            } else {
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos})
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_or {
    ($pos:ident, $text:ident, $out:expr, $x1:expr, $($x2:expr),*) => {
        {
            if let ::std::result::Result::Ok(__res) = $x1 { 
                _tredgen_append!($pos, $text, $out, __res);
            } 
            $(
                else if let ::std::result::Result::Ok(__res) = $x2 {
                   _tredgen_append!($pos, $text, $out, __res);
                }
            )*
            else { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos}); }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_not {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        if let ::std::result::Result::Ok(_) = $x { 
           return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos});
        }
    };
}

#[macro_export]
macro_rules! _tredgen_option {
    ($pos:ident, $text:ident, $out:expr, $x1:expr, $($x2:expr),*) => {
        if let ::std::result::Result::Ok(mut __res) = $x1 {
            _tredgen_append!($pos, $text, $out, __res);
        }
        $(else if let ::std::result::Result::Ok(mut __res) = $x2 {
            _tredgen_append!($pos, $text, $out, __res);
        })*
    };
}

#[macro_export]
macro_rules! _tredgen_many {
    ($pos:ident, $text:ident, $out:expr, $x1:expr, $($x1:expr),*) => {
        {
            let mut __mark = false;
            loop {
                if let ::std::result::Result::Ok(mut __res) = $x1 {
                    _tredgen_append!($pos, $text, $out, __res);
                } else {
                    if !__mark { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos}); }
                    else { break; }
                }
                __mark = true;
                $(if let ::std::result::Result::Ok(mut __res) = $x2 {
                    _tredgen_append!($pos, $text, $out, __res);
                } else {
                    break;
                })*
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_some {
    ($pos:ident, $text:ident, $out:expr, $($x:expr),+) => {
        loop {
            $(if let ::std::result::Result::Ok(mut __res) = $x {
                _tredgen_append!($pos, $text, $out, __res);
            } else {
                break; 
            })+
        }
    };
}

#[macro_export]
macro_rules! _tredgen_all {
    ($pos:ident, $text:ident, $out:expr, $($x:expr),+) => {
        while $text.len() > 0 {
            $(match $x {
                Ok(mut __res) => _tredgen_append!($pos, $text, $out, __res),
                e @ _ => return e;
            })+
        }
    };
}