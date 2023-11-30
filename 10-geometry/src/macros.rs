macro_rules! println_var {
    ($varname: ident) => {
        println!("{} = {}", stringify!($varname), $varname)
    }
}

macro_rules! println_var_debug {
    ($varname: ident) => {
        println!("{} = {:?}", stringify!($varname), $varname)
    }
}

macro_rules! println_var_debug_pretty {
    ($varname: ident) => {
        println!("{} = {:#?}", stringify!($varname), $varname)
    }
}

macro_rules! println_var_g {
    ($varname: ident) => {
        println!("{} = {}", stringify!($varname), $varname)
    };
    ($varname: ident*) => {
        println!("{} = {:?}", stringify!($varname), $varname)
    };
    ($varname: ident+) => {
        println!("{} = {:#?}", stringify!($varname), $varname)
    }
}

macro_rules! println_sliceable {
    ($sliceable: expr) => {
        {
            let _n = ($sliceable).len();
            if _n <= 10 {
                println!("{}#{:?}", _n, $sliceable)
            } else {
                println!("{}#{:?}...{:?}", _n, &($sliceable)[..5], &($sliceable)[_n-5..])
            }
        }
    };
    ($sliceable: expr, $threshold: expr) => {
        {
            let _n = ($sliceable).len();
            if _n <= $threshold {
                println!("{}#{:?}", _n, $sliceable)
            } else {
                println!("{}#{:?}...{:?}", _n, 
                    &($sliceable)[..($threshold / 2)], 
                    &($sliceable)[_n - ($threshold / 2) ..])
            }
        }
    }
}

macro_rules! println_vars{
    ($($varname: ident),*) =>
        {
            $(
                println_var!($varname);
            )*
        }
}

