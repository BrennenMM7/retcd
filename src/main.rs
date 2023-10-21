use retcd::retcdmain::retcd; 


fn main() {

    let args: Vec<String> = std::env::args().collect();

    retcd::start_retcd_or_proxy(args)
    

}