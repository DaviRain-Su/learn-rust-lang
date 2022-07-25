use core::any::Any;
use std::{fmt::Formatter, sync::Arc, collections::BTreeMap, borrow::Borrow, any::TypeId};
use anyhow::Result;
use core::fmt::Debug;

fn main() {
    let mut context = Contenxt::new();

    let result = context
        .router()
        .has_route(&"transfer".to_string());

    println!("result = {}", result);

    let cb = context
        .router_mut()
        .get_route_mut(&"transfer".to_string()).unwrap();


    let result = TransferModule.on_recv_packet();

    match result {
        OnRecvPacketAck::Nil(write_fn) => {
           
            
            let ret = write_fn(cb.as_any_mut());
            println!("ret = {:?}", ret);
        }
        OnRecvPacketAck::Successful(_ack, write_fn) => {
           
            let ret = write_fn(cb.as_any_mut());
            println!("ret = {:?}", ret);

            let ack = _ack.as_any().downcast_ref::<Acknowledgement>().unwrap();
            println!("ack = {:?}", ack);
            
        }
        OnRecvPacketAck::Failed(_ack) => {
            println!("ack");
            let ack = _ack.as_any().downcast_ref::<Acknowledgement>().unwrap();
            println!("ack = {:?}", ack);
        }
    }

    let call_back = |ctx: &mut dyn Any| -> anyhow::Result<()>{
        let ctx = ctx.downcast_mut::<Contenxt>();
        println!("ctx = {:?}", ctx);
        println!("[proce_recv_packet] ctx3 = {:?}", ctx);
        println!("[process_recv_packet]");

        Ok(())
    };

    call_back(cb.as_any_mut());

}



pub trait AsAnyMut: Any {
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;
}


impl<M: Any + Module> AsAnyMut for M {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/// Types implementing this trait are expected to implement `From<GenericAcknowledgement>`
pub trait AcknowledgementInterface: AsRef<[u8]> {
    fn as_any(&self) -> &dyn Any;
}

pub type WriteFn = dyn FnOnce(&mut dyn Any) -> Result<(), String>;

pub enum OnRecvPacketAck {
    Nil(Box<WriteFn>),
    Successful(Box<dyn AcknowledgementInterface>, Box<WriteFn>),
    Failed(Box<dyn AcknowledgementInterface>),
}

pub trait Module: Send + Sync + AsAnyMut { 
    fn on_recv_packet(
        &self,
    ) -> OnRecvPacketAck {
        OnRecvPacketAck::Nil(Box::new(|_| Ok(())))
    }
}

#[derive(Clone, Debug, Default)]
struct Contenxt {
    router: MockRouter,
}

impl Contenxt  {
    fn new() -> Self {
        
		let r = MockRouterBuilder::default()
        .add_route("transfer".to_string(), TransferModule)
        .unwrap()
        .build();

        Self { router: r }
    }    
}

#[derive(Debug)]
struct TransferModule;

impl Module for TransferModule {
    fn on_recv_packet(&self) -> OnRecvPacketAck  {
        recv_packet(self)
    }
}

impl Ics26Context for Contenxt {
    type Router = MockRouter;

	fn router(&self) -> &Self::Router {

		&self.router
	}

	fn router_mut(&mut self) -> &mut Self::Router {

		&mut self.router
	}
}

impl Ics20Context for TransferModule {}

pub trait Ics20Context {}



pub trait Ics26Context {
    type Router: Router;

    fn router(&self) -> &Self::Router;

    fn router_mut(&mut self) -> &mut Self::Router;
}

pub fn recv_packet<Ctx: 'static + Ics20Context + Debug>(
    ctx: &Ctx,
) -> OnRecvPacketAck {

    let ack = match process_recv_packet(ctx) {
        Ok(write_fn) => OnRecvPacketAck::Successful(Box::new(Acknowledgement::success()), write_fn),
        Err(e) => OnRecvPacketAck::Failed(Box::new(Acknowledgement::from_error(e.to_string()))),
    };
    
    ack
}

pub fn process_recv_packet<Ctx: 'static + Ics20Context + Debug>(
    ctx: &Ctx,
) -> Result<Box<WriteFn>> {
    println!("[proce_recv_packet] ctx1 = {:?}", ctx);
    Ok(Box::new(move |ctx| {
        let ctx = ctx.downcast_mut::<Ctx>();
        println!("ctx = {:?}", ctx);
        println!("[proce_recv_packet] ctx3 = {:?}", ctx);
        println!("[process_recv_packet]");

        Ok(())
    }))
}

pub const ACK_ERR_STR: &str = "error handling packet on destination chain: see events for details";

/// A successful acknowledgement, equivalent to `base64::encode(0x01)`.
pub const ACK_SUCCESS_B64: &str = "AQ==";

#[derive(Clone, Debug, PartialEq)]
pub enum ConstAckSuccess {
    Success,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Acknowledgement {
    /// Successful Acknowledgement
    /// e.g. `{"result":"AQ=="}`
    Success(ConstAckSuccess),
    /// Error Acknowledgement
    /// e.g. `{"error":"cannot unmarshal ICS-20 transfer packet data"}`
    Error(String),
}

impl Acknowledgement {
    pub fn success() -> Self {
        Self::Success(ConstAckSuccess::Success)
    }

    pub fn from_error(err: String) -> Self {
        Self::Error(format!("{}: {}", ACK_ERR_STR, err))
    }
}

impl AsRef<[u8]> for Acknowledgement {
    fn as_ref(&self) -> &[u8] {
        match self {
            Acknowledgement::Success(_) => ACK_SUCCESS_B64.as_bytes(),
            Acknowledgement::Error(s) => s.as_bytes(),
        }
    }
}

impl AcknowledgementInterface for Acknowledgement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub trait Router {
    /// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
    fn get_route_mut(&mut self, module_id: &impl Borrow<String>) -> Option<&mut dyn Module>;

    /// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
    fn has_route(&self, module_id: &impl Borrow<String>) -> bool;
}


pub trait RouterBuilder: Sized {
    /// The `Router` type that the builder must build
    type Router: Router;

    /// Registers `Module` against the specified `ModuleId` in the `Router`'s internal map
    ///
    /// Returns an error if a `Module` has already been registered against the specified `ModuleId`
    fn add_route(self, module_id: String, module: impl Module) -> Result<Self, String>;

    /// Consumes the `RouterBuilder` and returns a `Router` as configured
    fn build(self) -> Self::Router;
}

#[derive(Default)]
pub struct MockRouterBuilder(MockRouter);

impl RouterBuilder for MockRouterBuilder {
	type Router = MockRouter;

	fn add_route(mut self, module_id: String, module: impl Module) -> Result<Self, String> {
		match self.0 .0.insert(module_id, Arc::new(module)) {
			None => Ok(self),
			Some(_) => Err("Duplicate module_id".to_owned()),
		}
	}

	fn build(self) -> Self::Router {
		self.0
	}
}

#[derive(Default, Clone)]
pub struct MockRouter(BTreeMap<String, Arc<dyn Module>>);

impl Debug for MockRouter {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		let mut keys = vec![];
		for (key, _) in self.0.iter() {
			keys.push(format!("{}", key));
		}

		write!(f, "MockRouter(BTreeMap(key({:?})", keys.join(","))
	}
}

impl Router for MockRouter {
	fn get_route_mut(&mut self, module_id: &impl Borrow<String>) -> Option<&mut dyn Module> {
	
		self.0.get_mut(module_id.borrow()).and_then(Arc::get_mut)
	}

	fn has_route(&self, module_id: &impl Borrow<String>) -> bool {
		self.0.get(module_id.borrow()).is_some()
	}
}

#[test]
fn test_type_id() {
    // let mut context = Contenxt::new();

    // let cb = context
    //     .router_mut()
    //     .get_route_mut(&"transfer".to_string()).unwrap();

    // let type_id = cb.as_any_mut().type_id();
    // println!("type_id = {:?}", type_id);
    
    // let type_id_left = TypeId::of::<Contenxt>();
    // println!("type_id_left = {:?}", type_id_left);
}
