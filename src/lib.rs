use ic_cdk_macros::*;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::defualt();
}


#[update]
fn vote(){

}

#[update]
fn trade(){

}

#[update]
fn contribute(){}
