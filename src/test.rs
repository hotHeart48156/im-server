// extern crate actix;
// use actix::prelude::*;
// use actix_web_actors::ws;
// #[derive(Message)]
// #[rtype(result = "()")]
// struct OrderShipped(usize);

// #[derive(Message)]
// #[rtype(result = "()")]
// struct Ship(usize);

// /// Subscribe to order shipped event.
// #[derive(Message)]
// #[rtype(result = "()")]
// struct Subscribe(pub Recipient<OrderShipped>);

// /// Actor that provides order shipped event subscriptions
// struct OrderEvents {
//     subscribers: Vec<Recipient<OrderShipped>>,
// }

// impl OrderEvents {
//     fn new() -> Self {
//         OrderEvents {
//             subscribers: vec![]
//         }
//     }
// }

// impl Actor for OrderEvents {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl OrderEvents {
//     /// Send event to all subscribers
//     fn notify(&mut self, order_id: usize) {
//         for subscr in &self.subscribers {
//            subscr.do_send(OrderShipped(order_id));
//         }
//     }
//     pub fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
//        let y= ctx.address().recipient();
//     }
// }

// /// Subscribe to shipment event
// impl Handler<Subscribe> for OrderEvents {
//     type Result = ();

//     fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
//         self.subscribers.push(msg.0);
//     }
// }

// //Subscribe to ship message
// impl Handler<Ship> for OrderEvents {
//     type Result = ();
//     fn handle(&mut self, msg: Ship, ctx: &mut Self::Context) -> Self::Result {
//         self.notify(msg.0);
//         System::current().stop();
//     }

    

// } 

// /// Email Subscriber 
// struct EmailSubscriber;
// impl Actor for EmailSubscriber {
//     type Context = Context<Self>;
// }

// impl Handler<OrderShipped> for EmailSubscriber {
//     type Result = ();
//     fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
//         println!("Email sent for order {}", msg.0)
//     }
    
// }

// impl Handler<Ship> for EmailSubscriber {
//     type Result = ();
//     fn handle(&mut self, msg: Ship, _ctx: &mut Self::Context) -> Self::Result {
//         println!("Email sent for order {}", msg.0)
//     }
    
// }
// struct SmsSubscriber;
// impl Actor for SmsSubscriber {
//     type Context = Context<Self>;
// }

// impl Handler<OrderShipped> for SmsSubscriber {
//     type Result = ();
//     fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
//         println!("SMS sent for order {}", msg.0)
//     }
    
// }

// pub fn yy() {
//     let system = System::new("events");
//     let tt=EmailSubscriber{}.start().recipient::<OrderShipped>();
//     let email_subscriber = Subscribe(EmailSubscriber{}.start().recipient());
//     let sms_subscriber = Subscribe(SmsSubscriber{}.start().recipient());
//     let order_event = OrderEvents::new().start();
//     order_event.do_send(email_subscriber);
//     order_event.do_send(sms_subscriber);
//     order_event.do_send(Ship(1000));
//     system.run().unwrap();
// }
